//! Keccak-256 using ceno_keccak (syscall-backed permutation).

extern crate ceno_rt;

use ceno_keccak::{Hasher, Keccak};

#[unsafe(no_mangle)]
pub extern "C" fn sys_panic(_ptr: *const u8, _len: usize) -> ! {
    ceno_rt::halt(1)
}

fn main() {
    let input: Vec<u8> = ceno_rt::read();

    let mut hasher = Keccak::v256();
    hasher.update(&input);

    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    let digest = output.to_vec();
    ceno_rt::commit(&digest);
}
