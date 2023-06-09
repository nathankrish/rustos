#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;

use shell::shell;
use pi::atags::{Atag, Atags};
use allocator::Allocator;
use fs::FileSystem;
use crate::console::{kprint, kprintln, CONSOLE};
use alloc::vec::Vec;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {
    unsafe {
        ALLOCATOR.initialize();
        // FILESYSTEM.initialize();
    }
    // let mut iter = Atags::get();
    // while let Some(atag) = iter.next() {
    //     kprintln!("{:#?}", atag);
    // }
    // kprintln!("{:?}", allocator::memory_map());

    let mut v = Vec::new();
    for i in 0..50 {
        v.push(i);
        kprintln!("{:?}", v);
    }
    kprintln!("Welcome to cs3210!");
    shell::shell("> ");
}
