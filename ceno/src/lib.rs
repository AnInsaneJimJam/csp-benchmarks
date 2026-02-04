use std::cell::RefCell;

use bincode::serialized_size;
use ceno_emul::{Platform, Program};
use ceno_host::CenoStdin;
use ceno_zkvm::e2e::{run_e2e_with_checkpoint, setup_platform, Checkpoint, MultiProver, Preset};
use ceno_zkvm::scheme::{create_backend, create_prover};
use ff_ext::BabyBearExt4;
use gkr_iop::cpu::default_backend_config;
use mpcs::BasefoldDefault;
use utils::harness::{AuditStatus, BenchProperties};

mod guest;

const STACK_SIZE: u32 = 32 * 1024;
const HEAP_SIZE: u32 = 2 * 1024 * 1024;
const PUBLIC_IO_SIZE: u32 = 4 * 1024;

pub type CenoField = BabyBearExt4;
pub type CenoPcs = BasefoldDefault<CenoField>;

#[derive(Clone, Copy, Debug)]
pub enum KeccakImpl {
    CenoKeccak,
    AlloyKeccak,
}

pub struct CenoPrepared {
    program: Program,
    platform: Platform,
    hints: Vec<u32>,
    public_io: Vec<u32>,
    max_steps: usize,
    elf_size: usize,
}

pub struct CenoProof {
    proof_size: usize,
    result: RefCell<Option<ceno_zkvm::e2e::E2ECheckpointResult<CenoField, CenoPcs>>>,
}

pub fn ceno_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "Ceno",
        "BabyBear",
        "GKR",
        Some("BaseFold"),
        "GKR",
        false,
        true,
        100,
        true,
        true,
        AuditStatus::NotAudited,
        Some("RISC-V RV32IM"),
    )
}

pub fn prepare_keccak(input_size: usize, impl_kind: KeccakImpl) -> CenoPrepared {
    let (message_bytes, digest_bytes) = utils::generate_keccak_input(input_size);

    let mut hints = CenoStdin::default();
    hints
        .write(&message_bytes)
        .expect("failed to serialize hints for keccak input");

    let mut public_io = CenoStdin::default();
    public_io
        .write(&digest_bytes)
        .expect("failed to serialize public io for keccak digest");

    let elf = guest_program_bytes(impl_kind);
    let program = Program::load_elf(elf, u32::MAX).expect("failed to load ceno guest ELF");
    let platform = setup_platform(
        Preset::Ceno,
        &program,
        STACK_SIZE,
        HEAP_SIZE,
        PUBLIC_IO_SIZE,
    );

    CenoPrepared {
        program,
        platform,
        hints: Vec::<u32>::from(&hints),
        public_io: Vec::<u32>::from(&public_io),
        max_steps: usize::MAX,
        elf_size: elf.len(),
    }
}

pub fn prepare_sha256(input_size: usize) -> CenoPrepared {
    let (message_bytes, digest_bytes) = utils::generate_sha256_input(input_size);
    let digest_words = digest_bytes_to_u32s(&digest_bytes);

    let mut hints = CenoStdin::default();
    hints
        .write(&message_bytes)
        .expect("failed to serialize hints for sha256 input");

    let mut public_io = CenoStdin::default();
    public_io
        .write(&digest_words)
        .expect("failed to serialize public io for sha256 digest");

    let elf = guest_program_sha256_bytes();
    let program = Program::load_elf(elf, u32::MAX).expect("failed to load ceno guest ELF");
    let platform = setup_platform(
        Preset::Ceno,
        &program,
        STACK_SIZE,
        HEAP_SIZE,
        PUBLIC_IO_SIZE,
    );

    CenoPrepared {
        program,
        platform,
        hints: Vec::<u32>::from(&hints),
        public_io: Vec::<u32>::from(&public_io),
        max_steps: usize::MAX,
        elf_size: elf.len(),
    }
}

pub fn prove(prepared: &CenoPrepared) -> CenoProof {
    let (max_num_variables, security_level) = default_backend_config();
    let backend = create_backend::<CenoField, CenoPcs>(max_num_variables, security_level);

    let result = run_e2e_with_checkpoint::<CenoField, CenoPcs, _, _>(
        create_prover(backend),
        prepared.program.clone(),
        prepared.platform.clone(),
        MultiProver::default(),
        &prepared.hints,
        &prepared.public_io,
        prepared.max_steps,
        Checkpoint::PrepVerify,
        None,
    );

    let proofs = result
        .proofs
        .as_ref()
        .expect("expected proofs from Ceno checkpoint");
    let proof_size = proofs
        .iter()
        .map(|proof| serialized_size(proof).expect("failed to measure proof size"))
        .sum::<u64>() as usize;

    CenoProof {
        proof_size,
        result: RefCell::new(Some(result)),
    }
}

pub fn verify(_prepared: &CenoPrepared, proof: &CenoProof) {
    let result = proof
        .result
        .borrow_mut()
        .take()
        .expect("missing verify step");
    result.next_step();
}

pub fn preprocessing_size(prepared: &CenoPrepared) -> usize {
    prepared.elf_size
}

pub fn proof_size(proof: &CenoProof) -> usize {
    proof.proof_size
}

fn guest_program_bytes(impl_kind: KeccakImpl) -> &'static [u8] {
    match impl_kind {
        KeccakImpl::CenoKeccak => guest::keccak_ceno_keccak,
        KeccakImpl::AlloyKeccak => guest::keccak_alloy_keccak,
    }
}

fn guest_program_sha256_bytes() -> &'static [u8] {
    guest::sha256
}

fn digest_bytes_to_u32s(digest: &[u8]) -> [u32; 8] {
    let mut words = [0u32; 8];
    for (idx, word) in words.iter_mut().enumerate() {
        let start = idx * 4;
        *word = u32::from_be_bytes([
            digest[start],
            digest[start + 1],
            digest[start + 2],
            digest[start + 3],
        ]);
    }
    words
}
