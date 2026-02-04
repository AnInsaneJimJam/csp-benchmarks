//! Keccak-256 using alloy-primitives native-keccak hook.

extern crate ceno_rt;

use alloy_primitives::keccak256;
use ceno_keccak::Keccak;
#[unsafe(no_mangle)]
pub extern "C" fn sys_panic(_ptr: *const u8, _len: usize) -> ! {
    ceno_rt::halt(1)
}

fn main() {
    let _ = Keccak::v256();

    let input: Vec<u8> = ceno_rt::read();

    let digest = keccak256(&input);
    let output = digest.as_slice().to_vec();

    ceno_rt::commit(&output);
}
