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