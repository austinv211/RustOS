#![no_std]
#![no_main]

// our lame panic handler
use core::panic::PanicInfo;

// we mark this as a diverging function by having it return the never type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // get a raw pointer for the memory address of the vga buffer
    let vga_buffer = 0xb8000 as *mut u8;
    
    // enum through characters in Hello world, write the character byte and the color byte at an offset
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}