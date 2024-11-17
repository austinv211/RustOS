#![no_std]
#![no_main]
mod vga_buffer;

// our lame panic handler
use core::panic::PanicInfo;

// we mark this as a diverging function by having it return the never type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
