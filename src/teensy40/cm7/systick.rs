#![allow(dead_code)]

use super::scb;
use volatile::ReadOnly;
use volatile::Volatile;

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
	pub fn enable() { unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() | 0b1) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn disable() { unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() & !0b1) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn enable_exception() { unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() | 0b10) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn disable_exception() { unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() & !0b10) }; }

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn set_clock_source(source: ClockSource) {
		match source {
			ClockSource::EXTERNAL => unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() & !0b100) },
			ClockSource::INTERNAL => unsafe { (*scb::SYST_CSR).write((*scb::SYST_CSR).read() | 0b100) },
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn get_clock_source() -> ClockSource {
		unsafe {
			match ((*scb::SYST_CSR).read() & 0b100) != 0 {
				false => ClockSource::EXTERNAL,
				true => ClockSource::INTERNAL,
			}
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn has_ticked() -> bool {
		unsafe {
			return ((*scb::SYST_CSR).read() & 0x10000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn has_reference_clock() -> bool {
		unsafe {
			return ((*scb::SYST_CALIB).read() & 0x80000000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn is_10ms_skewed() -> bool {
		unsafe {
			return ((*scb::SYST_CALIB).read() & 0x40000000) != 0;
		}
	}

	#[link_section = ".fastrun"]
	#[inline(always)]
	pub fn get_10ms_tick_count() -> u32 {
		unsafe {
			return (*scb::SYST_CALIB).read() & 0xFFFFFF;
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
