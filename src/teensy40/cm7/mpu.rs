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

pub enum AccessPermissions {
	None,
	ReadOnly,
	ReadWrite,
}

pub enum MemoryType {
	StronglyOrdered,
	Device,
	DeviceNonShared,
	Normal,
}

pub enum CachePolicy {
	None,
	WriteBackAlloc,
	WriteThroughNoAlloc,
	WriteBackNoAlloc,
}

pub const SIZE_32B: u8 = 0b00100;
pub const SIZE_64B: u8 = 0b00101;
pub const SIZE_128B: u8 = 0b00110;
pub const SIZE_256B: u8 = 0b00111;
pub const SIZE_512B: u8 = 0b01000;
pub const SIZE_1KB: u8 = 0b01001;
pub const SIZE_2KB: u8 = 0b01010;
pub const SIZE_4KB: u8 = 0b01011;
pub const SIZE_8KB: u8 = 0b01100;
pub const SIZE_16KB: u8 = 0b01101;
pub const SIZE_32KB: u8 = 0b01110;
pub const SIZE_64KB: u8 = 0b01111;
pub const SIZE_128KB: u8 = 0b10000;
pub const SIZE_256KB: u8 = 0b10001;
pub const SIZE_512KB: u8 = 0b10010;
pub const SIZE_1MB: u8 = 0b10011;
pub const SIZE_2MB: u8 = 0b10100;
pub const SIZE_4MB: u8 = 0b10101;
pub const SIZE_8MB: u8 = 0b10110;
pub const SIZE_16MB: u8 = 0b10111;
pub const SIZE_32MB: u8 = 0b11000;
pub const SIZE_64MB: u8 = 0b11001;
pub const SIZE_128MB: u8 = 0b11010;
pub const SIZE_256MB: u8 = 0b11011;
pub const SIZE_512MB: u8 = 0b11100;
pub const SIZE_1GB: u8 = 0b11101;
pub const SIZE_2GB: u8 = 0b11110;
pub const SIZE_4GB: u8 = 0b11111;

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
pub fn set_region_address(index: usize, address: u32) {
	unsafe {
		(*RNR).write((index as u32) & 0xFF);
		(*RBAR).write(((*RBAR).read() & 0b01111) | (address & !0b11111));
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn set_region_access(instruction: bool, privileged: AccessPermissions, unprivileged: AccessPermissions) {
	let mask: u32 = !(0b10111 << 24);
	let mut set: u32 = 0;

	if !instruction {
		set = set | (0b10000 << 24);
	}
	match privileged {
		AccessPermissions::ReadOnly => match unprivileged {
			AccessPermissions::None => set = set | (0b101 << 24),
			AccessPermissions::ReadOnly => set = set | (0b110 << 24),
			AccessPermissions::ReadWrite => panic!("Invalid Access Mode"),
		},
		AccessPermissions::ReadWrite => match unprivileged {
			AccessPermissions::None => set = set | (0b001 << 24),
			AccessPermissions::ReadOnly => set = set | (0b010 << 24),
			AccessPermissions::ReadWrite => set = set | (0b011 << 24),
		},
		_ => set = set,
	}

	unsafe {
		(*RASR).write(((*RASR).read() & mask) | set);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn set_region_attributes(memory_type: MemoryType, cache: CachePolicy) {
	let mask: u32 = !(0b111111 << 16);
	let set: u32;

	match memory_type {
		MemoryType::StronglyOrdered => set = 0b000000 << 16,
		MemoryType::Device => set = 0b000010 << 16,
		MemoryType::DeviceNonShared => set = 0b010000 << 16,
		MemoryType::Normal => match cache {
			CachePolicy::None => set = 0b001000 << 16,
			CachePolicy::WriteBackAlloc => set = 0b001110 << 16,
			CachePolicy::WriteThroughNoAlloc => set = 0b000100 << 16,
			CachePolicy::WriteBackNoAlloc => set = 0b000110 << 16,
		},
	}

	unsafe {
		(*RASR).write(((*RASR).read() & mask) | set);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn set_region_custom_attributes(inner_cache: CachePolicy, outer_cache: CachePolicy) {
	let mask: u32 = !(0b111111 << 16);
	let mut set: u32;

	match outer_cache {
		CachePolicy::None => set = (0b00 << 2) << 16,
		CachePolicy::WriteBackAlloc => set = (0b01 << 2) << 16,
		CachePolicy::WriteThroughNoAlloc => set = (0b10 << 2) << 16,
		CachePolicy::WriteBackNoAlloc => set = (0b11 << 2) << 16,
	}
	match inner_cache {
		CachePolicy::None => set = set | (0b00 << 16),
		CachePolicy::WriteBackAlloc => set = set | (0b01 << 16),
		CachePolicy::WriteThroughNoAlloc => set = set | (0b10 << 16),
		CachePolicy::WriteBackNoAlloc => set = set | (0b11 << 16),
	}
	set = set | 0b100000 << 16;

	unsafe {
		(*RASR).write(((*RASR).read() & mask) | set);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn set_region_shareable(shareable: bool) {
	unsafe {
		(*RASR).write(
			((*RASR).read() & !(1 << 18))
				| (match shareable {
					true => 1,
					false => 0,
				} << 18),
		);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn set_region_size(size: u8) {
	unsafe {
		(*RASR).write(((*RASR).read() & !0b111110) | ((size as u32) << 1));
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
// exp: Size in 2^(exp + 1)
pub fn set_region_enabled(enabled: bool) {
	unsafe {
		(*RASR).write(
			((*RASR).read() & !0b1)
				| match enabled {
					true => 1,
					false => 0,
				},
		);
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub fn initialize() {
	let reg_count = unsafe { (((*CTRL).read() & 0x0000FF00) >> 8) as usize };

	disable();
	disable_during_hardfault();
	enable_privileged_default_access();
	for i in 0..reg_count {
		set_region_address(i, 0);
		set_region_access(true, AccessPermissions::ReadWrite, AccessPermissions::ReadWrite);
		set_region_attributes(MemoryType::StronglyOrdered, CachePolicy::None);
		set_region_shareable(true);
		set_region_size(SIZE_4GB);
		set_region_enabled(false);
	}
}
