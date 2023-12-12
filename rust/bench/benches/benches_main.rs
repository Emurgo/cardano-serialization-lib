use criterion::criterion_main;

mod block_bench;

criterion_main! {
   block_bench::benches
}