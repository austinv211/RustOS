use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// the UART is programmed using port I/O and is more complex that the previous VGA buffer we worked on
// it uses multiple I/O ports for programming different device registers
// the unsafe SerialPort::new(0x3f8) expects the first addresses of all needed ports
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) }; // standard port number for the first serial interface
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// to make the serial port easily usable, we can create a few macros
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}