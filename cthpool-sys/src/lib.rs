use std::ffi::c_void;

pub type Thpool = c_void;

pub type Job = extern "C" fn(*mut JobArg);
pub type JobArg = c_void;

extern "C" {
    pub fn thpool_init(num_threads: i32) -> *mut Thpool;
    pub fn thpool_add_work(pool: *mut Thpool, job: Job, arg: *mut JobArg) -> i32;
    pub fn thpool_wait(pool: *mut Thpool);
    pub fn thpool_pause(pool: *mut Thpool);
    pub fn thpool_resume(pool: *mut Thpool);
    pub fn thpool_destroy(pool: *mut Thpool);
    pub fn thpool_num_threads_working(pool: *mut Thpool) -> i32;
}
