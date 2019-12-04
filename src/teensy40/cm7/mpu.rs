#![allow(dead_code)]

// 4.6 Operational Memory Protection Unit

use volatile::*;

pub const TYPE: *mut ReadOnly<u32> = (0xE000_ED90) as *mut ReadOnly<u32>;
pub const CTRL: *mut Volatile<u32> = (0xE000_ED94) as *mut Volatile<u32>;
pub const RNR: *mut Volatile<u32> = (0xE000_ED98) as *mut Volatile<u32>;
pub const RBAR: *mut Volatile<u32> = (0xE000_ED9C) as *mut Volatile<u32>;
pub const RASR: *mut Volatile<u32> = (0xE000_EDA0) as *mut Volatile<u32>;
pub const RBAR_A1: *mut Volatile<u32> = (0xE000_EDA4) as *mut Volatile<u32>;
pub const RASR_A1: *mut Volatile<u32> = (0xE000_EDA8) as *mut Volatile<u32>;
pub const RBAR_A2: *mut Volatile<u32> = (0xE000_EDAC) as *mut Volatile<u32>;
pub const RASR_A2: *mut Volatile<u32> = (0xE000_EDB0) as *mut Volatile<u32>;
pub const RBAR_A3: *mut Volatile<u32> = (0xE000_EDB4) as *mut Volatile<u32>;
pub const RASR_A3: *mut Volatile<u32> = (0xE000_EDB8) as *mut Volatile<u32>;

#[link_section = ".fastrun"]
#[inline(always)]
pub fn enable() { unsafe { (*CTRL).write((*CTRL).read() | 0b1) } }

#[link_section = ".fastrun"]
#[inline(always)]
pub fn disable() { unsafe { (*CTRL).write((*CTRL).read() & !0b1) } }

#[link_section = ".fastrun"]
#[inline(always)]
pub fn is_enabled() -> bool { unsafe { return ((*CTRL).read() & 0b1) != 0 } }

#[link_section = ".fastrun"]
#[inline(always)]
pub fn enable_during_hardfault() {
	unsafe {
		(*CTRL).write((*CTRL).read() | 0b10);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn disable_during_hardfault() {
	unsafe {
		(*CTRL).write((*CTRL).read() & !0b10);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn is_enabled_during_hardfault() -> bool {
	unsafe {
		return ((*CTRL).read() & 0b10) != 0;
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn enable_privileged_default_access() {
	unsafe {
		(*CTRL).write((*CTRL).read() | 0b100);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn disable_privileged_default_access() {
	unsafe {
		(*CTRL).write((*CTRL).read() & !0b100);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn is_privileged_default_access_enabled() -> bool {
	unsafe {
		return ((*CTRL).read() & 0b100) != 0;
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn select_region(index: usize) {
	unsafe {
		(*RNR).write(index as u32);
	}
}

