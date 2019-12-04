use super::cm7;
use super::iomuxc;
use super::ccm;
use core::mem::transmute;

extern "C" {
	static _FLEXRAM_BANK_CONFIG: u32;
	static _STACK_END: u32;

	static _TEXT_START: u32;
	static _TEXT_END: u32;
	static _TEXT_LOAD: u32;
	static _DATA_START: u32;
	static _DATA_END: u32;
	static _DATA_LOAD: u32;

	static _BSS_START: u32;
	static _BSS_END: u32;
	fn main();
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn memcpy_u32(dst: *mut u32, src: *mut u32, len: usize) {
	// Copy to same location is not allowed.
	if src == dst {
		return;
	}
	// Zero-length copy is no-op.
	if len == 0 {
		return;
	}

	let mut _src: *mut u32 = src;
	let mut _dst: *mut u32 = dst;
	for _i in 0..(len - 1) {
		(*_dst) = *_src;
		_src = _src.offset(1);
		_dst = _dst.offset(1);
	}
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn memset_u32(dst: *mut u32, len: usize, value: u32) {
	// Zero-length set is no-op.
	if len == 0 {
		return;
	}

	let mut _dst: *mut u32 = dst;
	for _i in 0..(len - 1) {
		(*_dst) = value;
		_dst = _dst.offset(1);
	}
}

#[link_section = ".startup"]
#[no_mangle]
pub unsafe extern "C" fn startup() {
	// Only on IMXRT1062
	{
		// FlexRAM Bank Configuration (Configure ITCM and DTCM)
		(*iomuxc::GPR17).write(transmute(&_FLEXRAM_BANK_CONFIG));
		// Enable FlexRAM, DTCM and ITCM from reset.
		(*iomuxc::GPR16).write(0b00000000_00000000_00000000_00000111u32);
		// Configure DTCM and ITCM to be 512KB
		(*iomuxc::GPR14).write(0b00000000_10101010_00000000_00000000u32);
		// Set up new Stack Pointer
		asm!("mov sp, $0" : : "r"(&_STACK_END));
	}

	// Initialize Memory
	memcpy_u32(transmute(&_TEXT_START), transmute(&_TEXT_LOAD), transmute(&_TEXT_END));
	memcpy_u32(transmute(&_DATA_START), transmute(&_DATA_LOAD), transmute(&_DATA_END));
	memset_u32(transmute(&_BSS_START), transmute(&_BSS_END), 0);

	// Enable FPU
	cm7::fpu::enable();

	// Cortex-M7: Initialize NVIC
	cm7::nvic::initialize();

	// Configure Clocks
	// CCM Setup for PIZ & GPT
	(*ccm::CSCMR1).write(((*ccm::CSCMR1).read() & !0b11_1111) | (1 << 6));
	// CCM Setup for UART Clock
	(*ccm::CSCDR1).write(((*ccm::CSCDR1).read() & !0b11_1111) | (1 << 6));
	
	// Only on IMXRT1062
	{
		// Arduino code claims "fast GPIO6, GPIO7, GPIO8, GPIO9".
		// All this does is toggle between GPIO1/GPIO6, etc.
		(*iomuxc::GPR26).write(0xFFFF_FFFF);
		(*iomuxc::GPR27).write(0xFFFF_FFFF);
		(*iomuxc::GPR28).write(0xFFFF_FFFF);
		(*iomuxc::GPR29).write(0xFFFF_FFFF);
	}

	// Cortex-M7: Configure MPU
	cm7::mpu::disable();
	cm7::mpu::disable_during_hardfault();
	cm7::mpu::enable_privileged_default_access();

	// Cortex-M7: Enable L1 Cache
	cm7::cache::enable();





	// super::ccm::Ccm::new().sanitize();
	main();
}
