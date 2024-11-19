#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // specify test_main to be called in test contexts

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

// TESTS
// The runner prints a short debug message and then calls each test function in the list.
// The argument type &[&dyn Fn()] is a slice of a trait object references on the Fn() trait
// basically a list of references to types that can be called like a function
#[cfg(test)] // only include function for tests
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// create a new Testable Trait
pub trait Testable {
    fn run(&self) -> ();
}

// we implement the run function by first printing the function name using the any::type_name function
// this function is implemented directly in the compiler and returns a string description of every type
// for functions, the type is their name, so this is exactly what we want in this case
impl<T> Testable for T where T: Fn() {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// we mark this as a diverging function by having it return the never type
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // panic!("Some panic message"); for testing panic

    // only loaded on test
    #[cfg(test)]
    test_main();

    loop {}
}
