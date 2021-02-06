use criterion::{black_box, criterion_group, criterion_main, Criterion};

use async_client;


fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    
    c.bench_function("get time", |b| b.iter(|| async_client::now_monotonic()));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);