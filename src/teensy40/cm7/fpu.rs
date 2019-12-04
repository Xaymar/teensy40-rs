#![allow(dead_code)]

// 4.5.5 Floating Point Unit

use super::scb;

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn initialize() {
	if ((*scb::PID0).read() & 0b1100) == 0b1100 {
		(*scb::CPACR).write((*scb::CPACR).read() | 0b0000_0000_1111_0000_0000_0000_0000_0000);
	}
}
