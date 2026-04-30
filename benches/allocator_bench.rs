use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bumper::BumpAllocator;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

/// Benchmark: allocate 1000 u32s using our bump allocator
fn bench_bump_alloc_u32(c: &mut Criterion) {
    c.bench_function("bump_alloc 1000 u32s", |b| {
        b.iter(|| {
            let mut arena = BumpAllocator::new(1024 * 64);
            for i in 0..1000u32 {
                let _ = arena.alloc_val(black_box(i));
            }
        });
    });
}

/// Benchmark: allocate 1000 u32s using the default heap allocator (Box)
fn bench_heap_alloc_u32(c: &mut Criterion) {
    c.bench_function("heap_alloc 1000 u32s (Box)", |b| {
        b.iter(|| {
            let mut v: Vec<Box<u32>> = Vec::with_capacity(1000);
            for i in 0..1000u32 {
                v.push(Box::new(black_box(i)));
            }
        });
    });
}

/// Benchmark: allocate 1000 Point structs using bump allocator
fn bench_bump_alloc_struct(c: &mut Criterion) {
    c.bench_function("bump_alloc 1000 Point structs", |b| {
        b.iter(|| {
            let mut arena = BumpAllocator::new(1024 * 64);
            for i in 0..1000u32 {
                let _ = arena.alloc_val(black_box(Point {
                    x: i as f64,
                    y: i as f64 * 2.0,
                    z: i as f64 * 3.0,
                }));
            }
        });
    });
}

/// Benchmark: allocate 1000 Point structs using the default heap (Box)
fn bench_heap_alloc_struct(c: &mut Criterion) {
    c.bench_function("heap_alloc 1000 Point structs (Box)", |b| {
        b.iter(|| {
            let mut v: Vec<Box<Point>> = Vec::with_capacity(1000);
            for i in 0..1000u32 {
                v.push(Box::new(black_box(Point {
                    x: i as f64,
                    y: i as f64 * 2.0,
                    z: i as f64 * 3.0,
                })));
            }
        });
    });
}

/// Benchmark: arena reset and reuse
fn bench_bump_reset(c: &mut Criterion) {
    c.bench_function("bump_alloc reset and reuse x1000", |b| {
        let mut arena = BumpAllocator::new(1024 * 64);
        b.iter(|| {
            for i in 0..1000u32 {
                let _ = arena.alloc_val(black_box(i));
            }
            arena.reset();
        });
    });
}

criterion_group!(
    benches,
    bench_bump_alloc_u32,
    bench_heap_alloc_u32,
    bench_bump_alloc_struct,
    bench_heap_alloc_struct,
    bench_bump_reset,
);
criterion_main!(benches);