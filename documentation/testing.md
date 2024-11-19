# Testing
Exploring unit integration testing in no_std executables. We will use Rust's support for custom test frameworks to execute test functions inside our kernel

* Rust has a built-in test framework that is capable of running unit tests without the need to set anything up. Just create a function that checks some results through assertions and add the #[test] attribute to the function header
* unfortanately it is a bit more complicated in no_std, the test library depends on std

## Custom Test Frameworks
Rust supports replacing the default test framework through the unstable `custom_test_frameworks` feature
* collects all functions annotated with a #[test_case] attribute and then invoking a user-specified runner function with the lists of test as an argument
* tradeoff that many advanced features such as `should_panic` tests are not available

## I/O Ports
There are two different approaches for communicating between the CPU and peripheral hardware on x86
* memory-mapped I/O
* port-mapped I/O

Port-mapped I/O uses a seperate I/O bus for communication. Each connected peripheral has one or more port numbers. To communicate with such an I/O port, there are special CPU instructions called `in` and `out`, which take a port number and a data byte (there are also variations of these commands that allow sending a u16 or u32)

The isa-debug-exit device uses port-mapped I/O. The iobase parameter specifies on which port address the device should live (0xf4 is generally unused port on x86's IO bus) and the iosize specifies the port size (0x04 means four bytes).

Instead of manually invoking the in and out assembly instructions, we use the abstractions provided by the x86_64 crate

## Serial Port
A simple way to send data is to use the serial port, an old inteface standard which is no longer found in modern computers
it is easu to program and QEMU can redirect the bytes sent over the serial to the host's standard output or a file

The chips implementing a serial interface are called UARTs. The common UARTs today are all compatible with the 16550 UA?RT, so we will use that model for our testing framework

We will use the uart_16550 crate to initialize the UART and send data over the serial port.

The uart_16550 crate contains a `SerialPort` struct that represents the UART registers, but we still need to construct an instance of it ourselves. For that, we create a new `serial` module

## Timeouts
since `cargo test` waits until the test runner exits, a test that never returns can block the test runner forever.
That's unfortanate, but not a big problem in practice since it's usually easy to avoid endless loops. In our case endless loops can occur in a few circumstances
* the bootloader fails to load our kernel, which causes the system to reboot endlessly
* the BIOS/UEFI firware fails to load the bootloader, which causes the same endless rebooting
* the CPU enters a loop {} statement at the end of some of our functions, for example because the QEMU device doesn't work properly
* the hardware causes a system reset, for example when a CPU exception is not caught

Since endless loops can occur in so many situations, the bootimage tool sets a timeout of 5 minutes for each test executable by default.
If the test does not finish within this time, it is marked as failed and a "Timed Out" error is printed to the console. This ensures that tests that are stuck in an endless loop don't block `cargo test` forever.

## Insert Printing Automatically
manually adding print statements for every test we write is cumbersome, so let's update our test_runner to print these messages automatically. To do that, we need to create a new Testable trait
