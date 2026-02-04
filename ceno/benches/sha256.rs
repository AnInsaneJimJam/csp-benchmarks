use ceno::{ceno_bench_properties, prepare_sha256, preprocessing_size, proof_size, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Ceno,
    None,
    "sha256_mem_ceno",
    ceno_bench_properties(),
    prepare_sha256,
    |_| 0,
    prove,
    verify,
    preprocessing_size,
    proof_size
);
