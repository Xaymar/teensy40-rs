use super::cm7;
use super::iomuxc;
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

	// Initialize Cortex-M7
	cm7::initialize();

	// super::ccm::Ccm::new().sanitize();
	main();
}
