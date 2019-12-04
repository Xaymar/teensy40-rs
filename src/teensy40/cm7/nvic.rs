#![allow(dead_code)]

use core::mem::transmute;
use volatile::*;

// Nested Vectored Interrupt Controller

// Maximum number of Interrupts
const MAX_INTERRUPTS: usize = 240;

// Base Pointer Addresses
// Interrupt Set-Enable Registers
const ISER_BASE: u32 = 0xE000_E100;
// Interrupt Clear-Enable Registers
const ICER_BASE: u32 = 0xE000_E180;
// Interrupt Set-Pending Registers
const ISPR_BASE: u32 = 0xE000_E200;
// Interrupt Clear-Pending Registers
const ICPR_BASE: u32 = 0xE000_E280;
// Interrupt Active Bit Registers
const IABR_BASE: u32 = 0xE000_E300;
// Interrupt Priority Registers
const IPR_BASE: u32 = 0xE000_E400;

// Memory Pointers
const ICTR: *mut ReadOnly<u32> = (0xE000_E004) as *mut ReadOnly<u32>; // Interrupt Controller Type Register
const VTOR: *mut Volatile<u32> = (0xE000_ED08) as *mut Volatile<u32>; // Vector Table Offset Register

// Global Interrupt Table
static mut _INTERRUPTS: [Option<&unsafe extern "C" fn()>; 256] = [None; 256];

#[link_section = ".fastrun"]
#[inline(always)]
fn is_index_valid(index: usize) -> bool { return index < MAX_INTERRUPTS; }

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn set_interrupt_enabled(index: usize, enabled: bool) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	if enabled {
		let ptr: *mut Volatile<u32> = (ISER_BASE + (index / 32) as u32) as *mut Volatile<u32>;
		(*ptr).write(index as u32 % 32);
	} else {
		let ptr: *mut Volatile<u32> = (ICER_BASE + (index / 32) as u32) as *mut Volatile<u32>;
		(*ptr).write(index as u32 % 32);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn is_interrupt_enabled(index: usize) -> bool {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	let ptr: *mut Volatile<u32> = (ISER_BASE + (index / 32) as u32) as *mut Volatile<u32>;
	return (*ptr).read() & (index as u32 % 32) != 0;
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn set_interrupt_pending(index: usize, enabled: bool) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	if enabled {
		let ptr: *mut Volatile<u32> = (ISPR_BASE + (index / 32) as u32) as *mut Volatile<u32>;
		(*ptr).write(index as u32 % 32);
	} else {
		let ptr: *mut Volatile<u32> = (ICPR_BASE + (index / 32) as u32) as *mut Volatile<u32>;
		(*ptr).write(index as u32 % 32);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn is_interrupt_pending(index: usize) -> bool {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	let ptr: *mut Volatile<u32> = (IPR_BASE + (index / 32) as u32) as *mut Volatile<u32>;
	return (*ptr).read() & (index as u32 % 32) != 0;
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn set_interrupt_callback(index: usize, cb: &'static unsafe extern "C" fn()) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	_INTERRUPTS[index] = Some(cb);
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn clear_interrupt_callback(index: usize) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}
	_INTERRUPTS[index] = None;
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn set_interrupt_priority(index: usize, priority: u8) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}

	let ptr: *mut Volatile<u8> = (IPR_BASE + index as u32) as *mut Volatile<u8>;
	(*ptr).write(priority);
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn get_interrupt_priority(index: usize) -> u8 {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}

	let ptr: *mut Volatile<u8> = (IPR_BASE + index as u32) as *mut Volatile<u8>;
	return (*ptr).read();
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn trigger_software_interrupt(index: usize) {
	if !is_index_valid(index) {
		panic!("Out of Bounds");
	}

	// Software Triggered Interrupt Register
	const STIR: *mut WriteOnly<u32> = (0xE000_EF00) as *mut WriteOnly<u32>;
	(*STIR).write(index as u32);
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn initialize() {
	// Disable all hardware triggered interrupts.1
	for i in 0..MAX_INTERRUPTS {
		clear_interrupt_callback(i);
		set_interrupt_priority(i, 128);
	}

	(*VTOR).write(transmute(&_INTERRUPTS));
}
