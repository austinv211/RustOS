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

in our [main.rs](../src/main.rs) file, we then need to disable to std library being implicitly loaded with the no_std attribute
* you will notice that the println! macro will now be error highlighted as we don't have access to it without std


When we try and build with an empty main function, Rust will complaing that we need a panic handler (don't panic)
for now, let's define our own panic handler that does nothing using the core PanicInfo item
```Rust
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

### language items
Language items are special functions and types that are required internally by the compiler. By default, Rust uses unwinding to run the destructors of all live stack variables in case of a panic.
This ensures all memory is freed and the allows the parent thread to catch the panic and continue execution. Unwinding is a complication process however and requires some base OS libraries

when we set panic to abort in our profiles in [Cargo.toml](../Cargo.toml) and the error we notice about panic hanlder impl goes away
but, when we try and build we get the following error
```
> cargo build
error: requires `start` lang_item
```

### start attribute
our freestanding executable does not have access to the Rust runtime and crt0, so we need to define our own entry point
```Rust
#![no_main]
```


Finally, we are glonna get some linker errors
for this we need to specify a build target
to make this easy I add it to config.toml