use std::sync::Arc;
use std::ops::Deref;

pub struct ThreadPool {
    inner: Arc<super::ThreadPool>,
}

impl From<super::ThreadPool> for ThreadPool {
    fn from(pool: super::ThreadPool) -> Self {
        let inner = Arc::new(pool);
        ThreadPool { inner }
    }
}

impl Clone for ThreadPool {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);
        ThreadPool { inner }
    }
}

impl Deref for ThreadPool {
    type Target = super::ThreadPool;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
