use ceno::{KeccakImpl, ceno_bench_properties, prepare_keccak, preprocessing_size, proof_size, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,
    ProvingSystem::Ceno,
    Some("ceno_keccak"),
    "keccak_mem_ceno_keccak",
    ceno_bench_properties(),
    |input_size| prepare_keccak(input_size, KeccakImpl::CenoKeccak),
    |_| 0,
    prove,
    verify,
    preprocessing_size,
    proof_size
);
