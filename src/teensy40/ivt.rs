#![allow(clippy::transmute_ptr_to_ptr)]

use super::startup::startup;
use core::mem::transmute;

extern "C" {
	static _FLASH_IMAGE_LENGTH: core::ffi::c_void;
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [Option<&'static ()>; 2] = [
	unsafe { Some(transmute(0x2001_0000)) }, // Stack Pointer
	unsafe { Some(transmute(&startup)) } // Reset Handler
];

// IMXRT1060 Chapter 8.7.1.2
#[link_section = ".bootdata"]
#[no_mangle]
pub static _BOOTDATA: [Option<&'static ()>; 3] = [
	unsafe { Some(transmute(0x6000_0000)) },
	unsafe { Some(transmute(&_FLASH_IMAGE_LENGTH)) },
	None,
];

// IMXRT1060 Chapter 8.7.1
#[link_section = ".ivt"]
#[no_mangle]
pub static _IMAGE_VECTOR_TABLE: [Option<&'static ()>; 8] = [
	unsafe { Some(transmute(0x40_20_00_D1))}, // Version
	unsafe { Some(transmute(&_VECTORS)) }, // 0x40 requires Vector Table, 0x41 requires Start Address
	None,
	None, // Device Configuration Data (null if unused)
	unsafe { Some(transmute(&_BOOTDATA)) }, // Boot Data
	unsafe { Some(transmute(&_IMAGE_VECTOR_TABLE)) }, // This IVT table. (self)
	None, // Command Sequence File (null if not doing TrustZone boot)
	None,
];
