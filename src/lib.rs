#![no_std]
#![feature(core_intrinsics, lang_items)] //used here
// #![feature(use_extern_macros)]
#![feature(asm)]                         //used in uart
#![allow(dead_code)] //complaining about unused code is really annoying in early development

use core::intrinsics::abort;

//#[macro_use]
pub mod uart;
pub mod atag;
pub mod mem;
pub mod test;
//pub mod video;
//pub mod process;

#[no_mangle]
pub extern fn kernel_main(_r0: u32, _r1: u32, atags_addr: u32) {
    uart::init();
    uart::write_str("PiOS booted!\n");

    uart::write_str("atags start addr: ");
    uart::write_hex(atags_addr);
    uart::write_str("\n");

    let atags = atag::parse_atags(atags_addr);
    let mem_size = match atags.mem {
        Some(tag) => {
            uart::write_str("Mem tag found.\n");
            tag.size
        },
        None => {
            uart::write_str("No mem tag found.\n");
            1024 * 1024 * 128
        },
    };
    uart::write_str("Mem size: ");
    uart::write_u32(mem_size);
    uart::write_str("\n\n");

    test::test_ff();

    loop {
        uart::write_c(uart::get_c())
    }
}


// These functions below provide definitions for symbols libcore
// expects which are not present on our bare metal target.

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}

#[lang = "panic_fmt"]
pub extern fn panic_fmt(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    unsafe { abort() }
}

// There has got to be a better solution than this...
// This panic defn is expected when attempting plain ol' int addition
#[no_mangle]
pub extern fn _ZN4core9panicking5panic17h55432cad82b6074eE() -> ! {
    unsafe { abort() }
}
