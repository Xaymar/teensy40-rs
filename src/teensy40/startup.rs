use super::ccm;
use super::cm7;
use super::gpio::*;
use super::iomuxc;
use core::mem::transmute;
use volatile::*;

extern "C" {
	static _FLEXRAM_BANK_CONFIG: core::ffi::c_void;
	static _STACK_END: core::ffi::c_void;

	static _TEXT_START: core::ffi::c_void;
	static _TEXT_END: core::ffi::c_void;
	static _TEXT_LEN: core::ffi::c_void;
	static _TEXT_LOAD: core::ffi::c_void;

	static _DATA_START: core::ffi::c_void;
	static _DATA_END: core::ffi::c_void;
	static _DATA_LEN: core::ffi::c_void;
	static _DATA_LOAD: core::ffi::c_void;

	static _BSS_START: core::ffi::c_void;
	static _BSS_END: core::ffi::c_void;
	static _BSS_LEN: core::ffi::c_void;

	fn main();
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn memcpy_u32(dst: *mut u32, src: *mut u32, src_end: *mut u32) {
	// Copy to same location is not allowed.
	if src == dst {
		return;
	}

	let mut _src: *mut u32 = src;
	let mut _dst: *mut u32 = dst;
	while _src < src_end {
		(*_dst) = *_src;
		_src = _src.offset(1);
		_dst = _dst.offset(1);
	}
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn memset_u32(dst: *mut u32, dst_end: *mut u32, value: u32) {
	let mut _dst: *mut u32 = dst;
	while _dst < dst_end {
		(*_dst) = value;
		_dst = _dst.offset(1);
	}
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn enable_debug_led() {
	(*iomuxc::SW_MUX_CTL_PAD_GPIO_B0_03).write(5);
	(*iomuxc::SW_PAD_CTL_PAD_GPIO_B0_03).write(0b111 << 3);
	(*iomuxc::GPR27).write(0xFFFF_FFFF);
	toggle_debug_led();
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn disable_debug_led() {
	(*iomuxc::SW_MUX_CTL_PAD_GPIO_B0_03).write(0);
	(*iomuxc::SW_PAD_CTL_PAD_GPIO_B0_03).write(0);
	let gpio7_dir: *mut Volatile<u32> = (0x42004004) as *mut Volatile<u32>;
	let gpio7_dr_clear: *mut Volatile<u32> = (0x42004088) as *mut Volatile<u32>;
	(*gpio7_dir).write(1 << 3);
	(*gpio7_dr_clear).write(1 << 3);
}

#[link_section = ".startup"]
#[no_mangle]
unsafe fn toggle_debug_led() {
	let gpio7_dir: *mut Volatile<u32> = (0x42004004) as *mut Volatile<u32>;
	let gpio7_dr_toggle: *mut Volatile<u32> = (0x4200408C) as *mut Volatile<u32>;
	(*gpio7_dir).write(1 << 3);
	(*gpio7_dr_toggle).write(1 << 3);
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
		asm!("mov sp, $0" : : "r"(transmute::<&core::ffi::c_void, u32>(&_STACK_END)));
	}

	// Initialize Memory
	memcpy_u32(
		transmute::<&core::ffi::c_void, *mut u32>(&_TEXT_LOAD),
		transmute::<&core::ffi::c_void, *mut u32>(&_TEXT_START),
		transmute::<&core::ffi::c_void, *mut u32>(&_TEXT_END),
	);
	memcpy_u32(
		transmute::<&core::ffi::c_void, *mut u32>(&_DATA_LOAD),
		transmute::<&core::ffi::c_void, *mut u32>(&_DATA_START),
		transmute::<&core::ffi::c_void, *mut u32>(&_DATA_END),
	);
	memset_u32(
		transmute::<&core::ffi::c_void, *mut u32>(&_BSS_START),
		transmute::<&core::ffi::c_void, *mut u32>(&_BSS_END),
		0,
	);

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
	cm7::mpu::initialize();
	{
		// ITCM: No Cache, ReadWrite All, 512KB
		cm7::mpu::set_region_address(0, 0x0000_0000);
		cm7::mpu::set_region_access(
			true,
			cm7::mpu::AccessPermissions::ReadWrite,
			cm7::mpu::AccessPermissions::ReadWrite,
		);
		cm7::mpu::set_region_custom_attributes(cm7::mpu::CachePolicy::None, cm7::mpu::CachePolicy::None);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_512KB);
	}
	{
		// Boot ROM: Write Through, ReadOnly, 128KB
		cm7::mpu::set_region_address(1, 0x0020_0000);
		cm7::mpu::set_region_access(true, cm7::mpu::AccessPermissions::ReadOnly, cm7::mpu::AccessPermissions::ReadOnly);
		cm7::mpu::set_region_custom_attributes(
			cm7::mpu::CachePolicy::WriteThroughNoAlloc,
			cm7::mpu::CachePolicy::WriteThroughNoAlloc,
		);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_128KB);
	}
	{
		// DTCM: No Cache, ReadWrite NoExec, 512KB
		cm7::mpu::set_region_address(2, 0x2000_0000);
		cm7::mpu::set_region_access(
			false,
			cm7::mpu::AccessPermissions::ReadWrite,
			cm7::mpu::AccessPermissions::ReadWrite,
		);
		cm7::mpu::set_region_custom_attributes(cm7::mpu::CachePolicy::None, cm7::mpu::CachePolicy::None);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_512KB);
	}
	{
		// RAM (AXI bus): Write Back With Alloc, ReadWrite, 1MB
		cm7::mpu::set_region_address(3, 0x2020_0000);
		cm7::mpu::set_region_access(
			false,
			cm7::mpu::AccessPermissions::ReadWrite,
			cm7::mpu::AccessPermissions::ReadWrite,
		);
		cm7::mpu::set_region_custom_attributes(
			cm7::mpu::CachePolicy::WriteBackAlloc,
			cm7::mpu::CachePolicy::WriteBackAlloc,
		);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_1MB);
	}
	{
		// Peripherals: Device NonShared, ReadWrite NoExec, 64MB
		cm7::mpu::set_region_address(4, 0x4000_0000);
		cm7::mpu::set_region_access(
			false,
			cm7::mpu::AccessPermissions::ReadWrite,
			cm7::mpu::AccessPermissions::ReadWrite,
		);
		cm7::mpu::set_region_attributes(cm7::mpu::MemoryType::DeviceNonShared, cm7::mpu::CachePolicy::None);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_64MB);
	}
	{
		// QSPI Flash: Write Back With Alloc, ReadOnly, 16MB
		cm7::mpu::set_region_address(5, 0x6000_0000);
		cm7::mpu::set_region_access(true, cm7::mpu::AccessPermissions::ReadOnly, cm7::mpu::AccessPermissions::ReadOnly);
		cm7::mpu::set_region_custom_attributes(
			cm7::mpu::CachePolicy::WriteBackAlloc,
			cm7::mpu::CachePolicy::WriteBackAlloc,
		);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_16MB);
	}
	{
		// Trap Null Pointer Deref
		cm7::mpu::set_region_address(6, 0x0000_0000);
		cm7::mpu::set_region_access(false, cm7::mpu::AccessPermissions::None, cm7::mpu::AccessPermissions::None);
		cm7::mpu::set_region_custom_attributes(cm7::mpu::CachePolicy::None, cm7::mpu::CachePolicy::None);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_32B);
	}
	{
		// Trap Stack Overflow
		cm7::mpu::set_region_address(7, transmute::<&core::ffi::c_void, u32>(&_BSS_END) - 32u32);
		cm7::mpu::set_region_access(false, cm7::mpu::AccessPermissions::None, cm7::mpu::AccessPermissions::None);
		cm7::mpu::set_region_custom_attributes(cm7::mpu::CachePolicy::None, cm7::mpu::CachePolicy::None);
		cm7::mpu::set_region_shareable(false);
		cm7::mpu::set_region_size(cm7::mpu::SIZE_32B);
	}
	// Enable MPU
	cm7::mpu::enable();

	// Cortex-M7: Enable L1 Cache
	cm7::cache::enable();
	
	// Cortex-M7: Configure SysTick

	// Turn on Debug LED
	enable_debug_led();

	loop {
		main();
	}
}
