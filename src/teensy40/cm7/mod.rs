#![allow(dead_code)]

pub mod fpu;
pub mod nvic;
pub mod scb;
pub mod systick;

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn initialize() {
	fpu::initialize();
	nvic::initialize();
	systick::initialize();
}
