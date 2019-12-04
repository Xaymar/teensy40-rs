#![allow(dead_code)]

pub mod cache;
pub mod fpu;
pub mod mpu;
pub mod nvic;
pub mod scb;
pub mod systick;

#[inline(always)]
pub fn instruction_synchronization_barrier() {
	unsafe { asm!("ISB") };
}

#[inline(always)]
pub fn data_synchronization_barrier() {
	unsafe { asm!("DSB") };
}
