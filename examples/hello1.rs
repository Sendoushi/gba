#![feature(start)]
#![no_std]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
  unsafe {
    (0x04000000 as *mut u16).write_volatile(0x0403);
    (0x06000000 as *mut u16).offset(120 + 80 * 240).write_volatile(0x001F);
    (0x06000000 as *mut u16).offset(136 + 80 * 240).write_volatile(0x03E0);
    (0x06000000 as *mut u16).offset(120 + 96 * 240).write_volatile(0x7C00);
    loop {}
  }
}
