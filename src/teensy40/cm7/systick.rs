#![allow(dead_code)]

use volatile::ReadOnly;
use volatile::Volatile;

// SysTick Control and Status Register
const SYST_CSR: *mut Volatile<u32> = (0xE000_E010) as *mut Volatile<u32>;
// SysTick Reload Value Register
const SYST_RVR: *mut Volatile<u32> = (0xE000_E014) as *mut Volatile<u32>;
// SysTick Current Value Register
const SYST_CVR: *mut Volatile<u32> = (0xE000_E018) as *mut Volatile<u32>;
// SysTick Calibration Value Register
const SYST_CALIB: *mut ReadOnly<u32> = (0xE000_E01C) as *mut ReadOnly<u32>;

pub struct SysTick {
	control_and_status: Volatile<u32>,
	reload_value: Volatile<u32>,
	current_value: Volatile<u32>,
	calibration_value: ReadOnly<u32>,
}

pub enum ClockSource {
	EXTERNAL,
	INTERNAL,
}

impl SysTick {
	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn new() -> &'static mut SysTick { unsafe { return &mut *(0xE000E010 as *mut SysTick) } }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn enable() { unsafe { (*SYST_CSR).write((*SYST_CSR).read() | 0b1) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn disable() { unsafe { (*SYST_CSR).write((*SYST_CSR).read() & !0b1) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn enable_exception() { unsafe { (*SYST_CSR).write((*SYST_CSR).read() | 0b10) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn disable_exception() { unsafe { (*SYST_CSR).write((*SYST_CSR).read() & !0b10) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn set_clock_source(source: ClockSource) {
		match source {
			ClockSource::EXTERNAL => unsafe { (*SYST_CSR).write((*SYST_CSR).read() & !0b100) },
			ClockSource::INTERNAL => unsafe { (*SYST_CSR).write((*SYST_CSR).read() | 0b100) },
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn get_clock_source() -> ClockSource {
		unsafe {
			match ((*SYST_CSR).read() & 0b100) != 0 {
				false => ClockSource::EXTERNAL,
				true => ClockSource::INTERNAL,
			}
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn has_ticked() -> bool {
		unsafe {
			return ((*SYST_CSR).read() & 0x10000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn has_reference_clock() -> bool {
		unsafe {
			return ((*SYST_CALIB).read() & 0x80000000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn is_10ms_skewed() -> bool {
		unsafe {
			return ((*SYST_CALIB).read() & 0x40000000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn get_10ms_tick_count() -> u32 {
		unsafe {
			return (*SYST_CALIB).read() & 0xFFFFFF;
		}
	}
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn initialize() {
	// Teensy 4.0 has two clocks:
	// - Internal at 24 MHz
	// - External at 100 KHz
	// Depending on if the Internal clock is skewed or not, we will select the correct one.

	let tick = SysTick::new();
	SysTick::disable();
	if SysTick::is_10ms_skewed() {
		// Use external clock, accurate to 1/100000th of a second.
		// 100µs = 100 - 1 ticks.
		SysTick::set_clock_source(ClockSource::EXTERNAL);
		tick.reload_value.write(100 - 1); // 100µs
	} else {
		// Use internal clock, accurate to 1/24000000th of a second.
		// 100µs = 2400 - 1 ticks. May stop being accurate with overclocking.
		SysTick::set_clock_source(ClockSource::INTERNAL);
		tick.reload_value.write(SysTick::get_10ms_tick_count() / 100 - 1) // 100µs
	}
	tick.current_value.write(0);
	SysTick::disable_exception();
	SysTick::enable();
}
