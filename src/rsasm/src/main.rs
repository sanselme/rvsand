// `global_asm!` and `asm!` are stable features since Rust nightly 1.59.
// Before that version, you need `#![feature(global_asm)] and the import
// of `core::arch::global_asm` is not required.
#![feature(restricted_std)]
use core::arch::global_asm;

// global_asm!("
//     foo:
//     mov rax, 1337
//     ret
// ");
// or with include_str!: relative to src directory
// // Cargo will rebuild the file if "foo.S" changes.
global_asm!(include_str!("foo.S"));

extern "C" {
    fn foo() -> u64;
}

fn main() {
    println!("foo: {}", unsafe { foo() });
}
