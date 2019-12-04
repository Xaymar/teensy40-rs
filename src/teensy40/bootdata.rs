#![allow(clippy::transmute_ptr_to_ptr)]

use super::startup::startup;
use core::mem::transmute;

extern "C" {
	fn _INITIAL_STACK();
	static _FLASH_IMAGE_LENGTH: core::ffi::c_void;
}

// 448 byte common FlexSPI configuration block, 8.6.3.1 page 223 (RT1060 rev 0)
// MCU_Flashloader_Reference_Manual.pdf, 8.2.1, Table 8-2, page 72-75
#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u32; 128] = [
	0x4246_4346,
	0x5601_0000,
	0,
	0x0002_0101,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0003_0401,
	0,
	0,
	0x0020_0000,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0A18_04EB,
	0x2604_3206,
	0,
	0,
	0x2404_0405,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0000_0406,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0818_0420,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0818_04D8,
	0,
	0,
	0,
	0x0818_0402,
	0x0000_2004,
	0,
	0,
	0,
	0,
	0,
	0,
	0x0000_0460,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	256,
	4096,
	1,
	0,
	0x0001_0000,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
	0,
];

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
	_INITIAL_STACK, // Stack Pointer
	startup // Reset Handler
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
