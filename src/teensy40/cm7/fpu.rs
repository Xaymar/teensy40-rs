#![allow(dead_code)]

// 4.5.5 Floating Point Unit

use super::scb;

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn enable() {
	(*scb::CPACR).write((*scb::CPACR).read() | 0b00000000_11110000_00000000_00000000);
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn disable() {
	(*scb::CPACR).write((*scb::CPACR).read() & !0b00000000_11110000_00000000_00000000);
}