/// Simple `Arc<ThreadPool>` wrapper.
pub mod arc;

mod pooljob;

use std::ops::Drop;
use cthpool_sys as ffi;

pub struct ThreadPool {
    inner: *mut ffi::Thpool,
}

pub struct Builder {
    num_threads: Option<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self { num_threads: None }
    }

    pub fn num_threads(mut self, n: usize) -> Self {
        self.num_threads = Some(n);
        self
    }

    pub fn build(self) -> ThreadPool {
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);
        let inner = unsafe { ffi::thpool_init(num_threads as i32) };
        ThreadPool { inner }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        unsafe { ffi::thpool_destroy(self.inner) }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = pooljob::new(job);
        let status = unsafe { ffi::thpool_add_work(self.inner, pooljob::call::<F>, job) };
        assert_eq!(status, 0);
    }

    pub fn join(&self) {
        unsafe { ffi::thpool_wait(self.inner) }
    }

    pub fn pause(&self) {
        unsafe { ffi::thpool_pause(self.inner) }
    }

    pub fn resume(&self) {
        unsafe { ffi::thpool_resume(self.inner) }
    }

    pub fn active_count(&self) -> usize {
        unsafe { ffi::thpool_num_threads_working(self.inner) as usize }
    }
}

unsafe impl Send for ThreadPool {}
unsafe impl Sync for ThreadPool {}
