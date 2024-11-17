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