extern crate ceno_rt;

use ceno_sha2::{Digest, Sha256};

#[unsafe(no_mangle)]
pub extern "C" fn sys_panic(_ptr: *const u8, _len: usize) -> ! {
    ceno_rt::halt(1)
}

fn main() {
    let input: Vec<u8> = ceno_rt::read();

    let h = Sha256::digest(&input);
    let h: [u8; 32] = h.into();
    let h: [u32; 8] = core::array::from_fn(|i| {
        let chunk = &h[4 * i..][..4];
        u32::from_be_bytes(chunk.try_into().expect("invalid sha256 chunk"))
    });

    ceno_rt::commit(&h);
}
