use std::time::Duration;

pub struct RateLimiter {
    backend: Box<dyn Backend>,
}

trait Backend {
    fn check(&self, key: &str, limit: u64, window: Duration) -> bool;
}

impl RateLimiter {
    pub fn new(backend: impl Backend + 'static) -> Self {
        Self { backend: Box::new(backend) }
    }
    
    pub fn allow(&self, key: &str, limit: u64, window: Duration) -> bool {
        self.backend.check(key, limit, window)
    }
}

struct RedisBackend;
impl RedisBackend {
    pub fn new(_url: &str) -> Self { Self }
}
impl Backend for RedisBackend {
    fn check(&self, _key: &str, _limit: u64, _window: Duration) -> bool { true }
}
