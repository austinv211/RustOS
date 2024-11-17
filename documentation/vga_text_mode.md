# VGA text mode
the VGA text mode is a simple way to print text to the screen

goals of this section
* create an interface for VGA text mode that makes its usage safe and simple
* implement support for Rust's formatting macros

## The VGA text buffer
to print a character to the screen in VGA text mode, one has to write it to the text buffer of the VGA hardware.
The VGA text buffer is a two-dimensional array with typically 25 rows and 80 columns, which is directly rendered to the screen

* Bit 0-7: ASCII code point
* Bit 8-11: Foreground color
* Bit 12-14: Background color 
* 15: Blink

see bit and color table at [here](https://os.phil-opp.com/vga-text-mode/)

The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000. This means that reads and writes to that address don't access the RAM but directly access the text buffer on the VGA hardware. This means we can read and write it through normal memory operations to that address

note: memory-mapped hardware might not support all normal RAM operations
example: a devbice could only support byte-wise reads and return junk when a u64 is read

Fortunately, the text buffer supports normal reads and writes

now lets create a new module for the VGA buffer in [vga_buffer.rs](../src/vga_buffer.rs)

```Rust
// in src/vga_buffer.rs

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
```

in above, we create a writer that points to the VGA buffer at 0xb8000.
First, we cast the integer 0xb8000 as a mutable raw pointer.
Then, we convert it to a mutable reference by deferencing it with * and immediately borrow it again through &mut
this conversion requires the `unsafe` block. since the compiler can't guarantee that the raw pointer is valid.
We will notic e a 2 byte printout for the UTF-8 character because the individual bytes of multi-byte values are never valid ASCII

## Volatile
the way we print right now may not work with future Rust compilers that optimize more aggressively, let's fix that.

The problem is that we only write to the Buffer and never read from it again. The compiler doesn't know that we really access VGA buffer memory (instead of normal RAM) and knows nothing about the side effect that some characters appear on the screen, so it might decide that these writes are unnecessary and can be ommited.

To avoid this we need to specify these writes as `volatile` -> This tells the compiler that the write has side effects and should not be optimized away.

In order to use volatile writes for the VGA buffer, we use the `volatile` crate. This crate provides a Volatile wrapper type with read and write methods. These methods internally use the `read_volatile` and `write_volatile` functions of the core library and thus guarantee that the reads/writes are not optimized away.

note: make sure to use version 0.2.6

## Formatting Macros
It would be nice to support Rust's formatting macros, that way we can easily print different types.

To do this we need to implement the `core::fmt::Write` trait.
* the only method of this trait is `write_str`
* for this we'll update the write_str method to return a `fmt::Result`

## A Global Interface
To provide a global writer that can be used as an interface from other modules without carrying a Writer instance around, we try to create a static WRITER

```Rust
// in src/vga_buffer.rs

pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
}
```

but if we try to compile we'll get a couple errors because statics are initialized at compile time, in contrats to normal variables that are initialized at run time.

We can get around this with the lazy_static crate
* this crate provides a `lazy_static!` macro that defines a lazily initialized `static` that is initialized when it is accessed fro the first time
* we need the `spin_no_std` since we aren't loading no std

for having a proper synchronized interior mutability, users of the std library can use Mutex, but our basic kernel does not have any blocking support or even a concept of threads

so instead let's use a spinlock
```
spin = "0.5.2"
```