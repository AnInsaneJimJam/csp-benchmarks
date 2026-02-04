extern crate ceno_rt;

#[unsafe(no_mangle)]
pub extern "C" fn sys_panic(_ptr: *const u8, _len: usize) -> ! {
    ceno_rt::halt(1)
}
