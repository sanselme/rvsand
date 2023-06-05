// Copyright (c) 2022 Schubert Anselme <schubert@anselm.es>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![feature(restricted_std)]

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
