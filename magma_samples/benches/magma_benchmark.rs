use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use cipher_magma::{Magma, CipherOperation, cipher_mode::CipherMode};

fn magma_block_benchmark(c: &mut Criterion) {
    let magma = Magma::new();
    c.bench_function("encrypt", |bencher| {
        bencher.iter(|| magma.encrypt(black_box(0_u64)))
    });
    c.bench_function("decrypt", |bencher| {
        bencher.iter(|| magma.decrypt(black_box(0_u64)))
    });
}

fn magma_buffer_benchmark(c: &mut Criterion) {
    let source_buffer = [0_u8; 4096];
    let mut magma = Magma::new();
    c.bench_function("encrypt", |bencher| {
        bencher.iter(|| magma.cipher(&source_buffer, &CipherOperation::Encrypt, &CipherMode::ECB))
    });
    c.bench_function("decrypt", |bencher| {
        bencher.iter(|| magma.cipher(&source_buffer, &CipherOperation::Decrypt, &CipherMode::ECB))
    });
}

fn magma_multiple_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Magma");

    let magma = Magma::new();
    for block_u64 in [u64::MIN, u32::MAX as u64, u64::MAX].iter() {
        group.bench_with_input(BenchmarkId::new("encrypt", block_u64), block_u64, |bencher, block_u64| {
            bencher.iter(|| magma.encrypt(*block_u64))
        });
        group.bench_with_input(BenchmarkId::new("decrypt", block_u64), block_u64, |bencher, block_u64| {
            bencher.iter(|| magma.encrypt(*block_u64))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    magma_block_benchmark,
    magma_buffer_benchmark,
    magma_multiple_benchmark,
);

criterion_main!(benches);
