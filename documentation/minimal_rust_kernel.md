# A Minimal Rust Kernel
When you turn on a computer, it begins executing firmware code that is stored in the motherboard ROM.
This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware.
Afterwards, it looks for a bootable disk and starts booting to the operating system kernel.

on x86 there are two firmware standards:
* BIOS
* UEFI

For right now we are starting with BIOS support since easier to setup, and UEFI machines support emulated BIOS
this has a downside that we get placed into a 16-bit compatibility mode call real mode before booting

1. BIOS is loaded from motherboard
2. self-test and initialization of hardware
3. looks for bootable disks
4. if found, control transferred to bootloader (512-byte portion of executable code stored at the disks beginning)
  * most bootloaders are more than 512-byte so they split into a smaller first stage which is subsequently loaded
5. the bootloader has to determine the location of the kernel image on the disk and load it into memory
  * it also needs to switch the CPU from the 16-bit real mode to the 32-bit protected mode, and then to the 64-bbit long mode
  * the 64-bit registers and the complete main memory is available
  * query info like memory-map and pass it to the OS kernel

the walkthrough I am following provied a bootimage tool that automatically prepends a bootloader to our kernel for simplicity right now.

Multiboot: an open bootloader standard
* the reference implementation is GNU GRUB

to make a kernel Multiboot compliant, we need to insert a Multiboot header at the beginning of the kernel file; however GRUB and multiboot have some problems too
* they support only the 32-bit protected mode. You still have to do the CPU configuration to switch to the 64-bit long mode.
* they are designed to make the bootloader simple instead of the kernel. Example: adjuested page size and boot information
* both GRUB and Multiboot are only sparsely documented
* GRUB needs to be installed on the host system to create a bootable disk image from the kernel file

Because of these drawbacks, we are not going to use GRUB or multiboot
for later, if interested in created a multiboot compliant kernel [here](https://os.phil-opp.com/edition-1/)

## Starting a Kernel
our goal is to create a disk image that prints a "Hello World" to the screen when booted

for some of the experimental features we are using we are going to install Rust nightly.

to override rust nightly for the current directory
```
rustup override set nightly
```
for purposes of checking in the code, I am going to create a rust-toolchain file.

## Target Specification
we can specify targer information with a json file, see [x86_64-rustos.json](../x86_64-blog_os.json)
note the os in the llvm target is none since we want to run on bare-metal

we add a couple properties for using the corrs-platform linker lld, disabling stack unwinding, and disabling a stack pointer optimization called redzone [disabling redzone](https://os.phil-opp.com/red-zone/)

we then disabale mmx and sse and enable soft-float

export on mmx and sme
```
The mmx and sse features determine support for Single Instruction Multiple Data (SIMD) instructions, which can often speed up programs significantly. However, using the large SIMD registers in OS kernels leads to performance problems. The reason is that the kernel needs to restore all registers to their original state before continuing an interrupted program. This means that the kernel has to save the complete SIMD state to main memory on each system call or hardware interrupt. Since the SIMD state is very large (512–1600 bytes) and interrupts can occur very often, these additional save/restore operations considerably harm performance. To avoid this, we disable SIMD for our kernel (not for applications running on top!).

A problem with disabling SIMD is that floating point operations on x86_64 require SIMD registers by default. To solve this problem, we add the soft-float feature, which emulates all floating point operations through software functions based on normal integers.
```

we can now specify our target in the build to use the json file.

Wehn trying to build we then get the following error
```
error[E0463]: can't find crate for `core`
  |
  = note: the `x86_64-rustos` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-rustos`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`

For more information about this error, try `rustc --explain E0463`.
warning: `rustos` (bin "rustos") generated 1 warning
error: could not compile `rustos` (bin "rustos") due to 1 previous error; 1 warning emitted
```

to fix this we need to recompile the core and compiler_builtins for our new custom target

for access tror memcpy, memset, memcmp it is included in compiler_builtins by default, but we need to specify the following to enable mem features
```
build-std-features: ["compiler-builtins-mem"]
```

## Printing to the Screen
the easiest way for printing to the screen at this stage is the VGA text buffer
* the VGA text buffer is a special memory area mapped to the VGA hardware that contains the contents displayed on the screen.
* it normally consists of 25 lines that each contain 80 character cells. Each character cell displays an ASCII character with some foreground and background colors

for printing "Hello World", we just need to know that the buffer is locateed at address `0xb8000` and that each character cell consists of an ASCII byte and a color byte

we start at the address for the buffer and write the character and color bytes for our phrase

#### note on unsafe block
unsafe does not turn off Rust's safety checks, this just lets us do some additional things:
* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of a union

see additional info on unsafe Rust [unsafe rust](https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html#unsafe-superpowers)

we want to minimize the use of unsafe as much as possible, one way to do this is with safe abstractions
example, we could create a VGS buffer type that encapsulates all unsafety and ensures that it is impossible to do anything wrong from the outside

### Running our Kernel
* first we need to turn our compiled kernel into a bootable disk image by linking it with a bootloader
* then we can run the disk image in the QEMU virtual machine or boot it on real hardware using a USB stick

as mentioned previously, we are using a crafte for the bootloader; so lets add bootloader to our dependencies in Cargo.toml

add the bootloader as a dependency is not enough to actually create a bootable disk image.
The problem is that we need to link our kernel with the bootloader after compilation, but cargo has no support for post-build scripts

To solve this issue, there is a tool named `bootimage` that first compiles the kernel and bootloader, then links them together to create a bootable disk image

1. outside of project install bootimage with `cargo install bootimage`
2. add llvm-tools-preview with `rustup component add llvm-tools-preview`
3. back in project directory run `cargo bootimage`

for running in QEMU
```
qemu-system-x86_64.exe -drive format=raw,file=target/x86_64-rustos/debug/bootimage-rustos.bin
```