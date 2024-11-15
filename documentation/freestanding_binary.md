# A freestanding Rust Binary

The first step to writing an OS kernel is we need a Rust executable that does not link to the [std](https://doc.rust-lang.org/std/) Crate because it relies on primitives that won't be present in an bare-metal environment. reference [no_std](https://docs.rust-embedded.org/book/intro/no-std.html#:~:text=As%20mentioned%20before%20using%20libstd,like%20bootloaders%2C%20firmware%20or%20kernels.)

we can't use threads, files, heap memory, the network, random numbers, etc.
there are a lot of Rust features we still can use:
  * iterators
  * closures
  * pattern matching
  * option and result
  * string formatting
  * ownership system

An exectuibale that can be run without an underlying operating system is often called "freestanding" or "bare-metal" executable.

We can create our executable project with the following command
`cargo new rustos --bin`