#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(const_raw_ptr_deref)]
#![feature(stdsimd)]
#![feature(asm)]
#![no_builtins]
#![no_std]
#![no_main]

mod teensy40;

#[no_mangle]
pub extern "C" fn main() { loop {} }

#[panic_handler]
fn teensy_panic(_: &core::panic::PanicInfo) -> ! { loop {} }
