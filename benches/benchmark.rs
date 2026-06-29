//! Performance benchmark tests

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use base_framework::prelude::*;
use std::sync::Arc;

fn bench_monitor_register(c: &mut Criterion) {
    let monitor = Monitor::new(MonitorConfig::default());

    c.bench_function("monitor_register_app", |b| {
        let mut i = 0;
        b.iter(|| {
            let app = AppInfo::new(
                format!("bench_app_{}", i),
                "Benchmark App",
                "1.0.0",
            );
            let _ = monitor.register_app(app);
            i += 1;
        });
    });
}

fn bench_monitor_metrics(c: &mut Criterion) {
    let monitor = Monitor::new(MonitorConfig::default());

    c.bench_function("monitor_record_metric", |b| {
        b.iter(|| {
            monitor.record_metric(black_box("bench_metric"), black_box(42.0));
        });
    });
}

fn bench_middleware_check(c: &mut Criterion) {
    let chain = MiddlewareChain::new();
    let whitelist = WhitelistChecker::new();
    whitelist.add_caller("bench_caller");
    chain.add_checker(Arc::new(whitelist));
    chain.add_checker(Arc::new(RateLimiter::new(100000, 60000)));

    let ctx = CheckContext::new("bench_caller", "bench_interface");

    c.bench_function("middleware_check", |b| {
        b.iter(|| {
            let _ = chain.check(black_box(&ctx));
        });
    });
}

fn bench_concurrent_metrics(c: &mut Criterion) {
    let monitor = Arc::new(Monitor::new(MonitorConfig::default()));

    c.bench_function("concurrent_record_metric", |b| {
        b.iter(|| {
            let m = monitor.clone();
            std::thread::spawn(move || {
                m.record_metric("concurrent", 1.0);
            });
        });
    });
}

criterion_group!(
    benches,
    bench_monitor_register,
    bench_monitor_metrics,
    bench_middleware_check,
    bench_concurrent_metrics,
);
criterion_main!(benches);
