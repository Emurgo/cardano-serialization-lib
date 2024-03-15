use cardano_serialization_lib::Block;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_decode_block(c: &mut Criterion) {
    let raw_data = include_bytes!("data/block_with_certs.bin").to_vec();
    c.bench_function("decode block", |b| {
        b.iter(|| {
            let data_copy = raw_data.clone();
            let block = Block::from_bytes(data_copy);
            assert!(block.is_ok());
        })
    });
}

criterion_group!(benches, bench_decode_block);
