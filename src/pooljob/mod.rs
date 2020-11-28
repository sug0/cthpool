use cthpool_sys as ffi;

pub fn new<F>(job: F) -> *mut ffi::JobArg
where
    F: FnOnce() + Send + 'static,
{
    let boxed_job = Box::new(job);
    Box::into_raw(boxed_job) as *mut _
}

pub extern "C" fn call<F>(job: *mut ffi::JobArg)
where
    F: FnOnce() + Send + 'static,
{
    let job: Box<F> = unsafe {
        // safety: thpool_destroy() will wait for running jobs
        // to finish, which will deallocate the memory associated
        // with the raw pointer
        Box::from_raw(job as *mut _)
    };
    job()
}
