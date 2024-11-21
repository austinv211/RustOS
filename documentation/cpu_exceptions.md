# CPU Exceptions
CPU exceptions occur in various erroneous situations, for example, when accessing an invalid memory address or when dividing by zero.
To react to them, we have to set up an interrupt descriptor table that provides handler functions.

our goal for this section is to get our kernel to be able to catch breakpoint exceptions and resume normal execution afterward.
 
An exception signals that something is wrong with the current instruction. For example, the CPU issues an exception if the current instruction tries to divide by 0.
When an exception occurs, the CPU interrupts its current work and immediatley calls a specific exception handler function, depending on the exception type.

on x86 there are about 20 different CPU exception types. The most important are:
* Page Fault: A page fault occurs on illegal memory accesses. For example, if the current instruction tries to read from an unmapped page or tries to write tto a read-only page.
* Invalid Opcode: This exception occurs when the current instruction is invalid, for example, when we try to use new SSE instructions on an old CPU that does not support them.
* General Protection Fault: This is the exception with the broadest range of causes. It occurs on various kinds of access violations, such as trying to execute a privileged instruction in user-level code or writing reserved fields in configuration registers
* Double Fault: When an exception occurs, the CPU tries to call the corresponding handler function. If another exception occurs when calling the exception handler, the CPU raises a double fault exception. This exception also occurs when there is no handler function registered for an exception.
* Triple Fault: If an exception occurs while the CPU tires to call the double fault handler function, it issues a fatal triple fault. We can't catch or handle a triple fault. Most processors react by resetting themselves and rebooting the operating system.

In order to catch and handle exceptions, we have to set up an interupt descriptor table (IDT). Each entry must have a 16-byte structure

When an exception occurs, the CPU roughly does the following
* Push some registers on the stack, including the instruction pointer and the RFLAGS register
* Read the corresponding entry from the Interupt Descriptor Table (IDT). For example, the CPU reads the 14th entry when a page fault occurs.
* Check if the entry is present and, if not, raise a double fault
* Disable hardware interupts if the entry is an interrupt gate (bit 40 not set)
* Load the specified GDT selector into the CS (code segment)
* jump to the specified handler function

## An IDT Type
Instead of creating our own IDT type, we will use the InteruptDescriptorTable struct of the x86_64 crate which looks like this

```Rust
#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<HandlerFuncWithErrCode>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<HandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    // some fields omitted
}
```

The fields have an idt::Entry<F>, which is a struct that represents the fields of an IDT entry (The type parameter F defines the expected handler function type).
The page fault even has its own special type: `PageFaultHandlerFunc`

Looking at the HandlerFunc type:
```Rust
type HandlerFunc = extern "x86-interupt" fn(_:InterruptStackFrame);
```
this is a type alias for an extern "x86_interrupt" fn type. The extern keyword defines a function with a foreign calling convention and is often used to communicate with C code

## The Interrupt Calling Convention
Exceptions are quite similar to function calls: The CPU jumps to the first instruction of the called function and executes it. Afterwards, the CPU jumps to the return address and continues the execution of the parent function.

* A function call is invoked voluntarily by a compiler-inserted call instruction
* an exception might occur at any instruction

Calling conventions specify the details of a function call. For example, they specify where function parameters are places or how results are returned.
On x86_64 Linux, the following rules apply for C functions (specified in the [System V ABI](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf))
* The first six integer arguments are passed in registers, rdi, rsi, rdx, rcx, r8, r9
* additional arguments are passed on the stack
* results are returned in rax and rdx

Note: Rust does not follow the C ABI so these rules only apply to functions declared as extern "C" fn