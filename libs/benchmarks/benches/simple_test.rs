use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn simple_benchmark(c: &mut Criterion) {
    println!("Starting simple benchmark");
    c.bench_function("hello_world", |b| {
        b.iter(|| {
            black_box("hello world".to_string())
        })
    });
}

criterion_group!(benches, simple_benchmark);
criterion_main!(benches);
