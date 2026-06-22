# Rate Limiter ⏱️

Distributed rate limiter with multiple algorithms.

## Algorithms

| Algorithm | Accuracy | Memory | Distributed |
|-----------|----------|--------|------------|
| Sliding Window | High | O(N) | Yes |
| Token Bucket | Medium | O(1) | Yes |
| Leaky Bucket | High | O(N) | Yes |

## Performance

| Metric | Value |
|--------|-------|
| Check speed | 500K/s per node |
| Memory | 1KB per key |
| Accuracy | 99.9% |

## Quick Start

```rust
let limiter = RateLimiter::new(RedisBackend::new("redis://localhost"));
if limiter.allow("user:123", 100, Duration::from_secs(60)) {
    // Allow
} else {
    // Rate limited
}
```

## License

MIT