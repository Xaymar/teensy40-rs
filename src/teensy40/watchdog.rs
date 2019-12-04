#![allow(dead_code)]
use core;

// Teensy 4.0 Watchdog
// - WDOG1 at 0x400B8000
// - WDOG2 at 0x400D0000
// See IMXRT1060RM_rev1.pdf Chapter 56 (WDOG1-2)

#[repr(C, packed)]
pub struct Watchdog {
	// 56.8.1.1
	// Watchdog Control
	wcr: u16,
	// Watchdog Service
	wsr: u16,
	// Watchdog Reset Status (Read-Only)
	wrsr: u16,
	// Watchdog Interrupt Control
	wicr: u16,
	// Watchdog Miscellaneous Control
	wmcr: u16,
}

impl Watchdog {
	pub fn new(idx: isize) -> &'static mut Watchdog {
		match idx {
			0 => unsafe { return &mut *(0x400B8000 as *mut Watchdog) },
			1 | _ => unsafe { return &mut *(0x400D0000 as *mut Watchdog) },
		}
	}

	pub fn wcr_write(&mut self, wcr: u16) {
		unsafe {
			core::ptr::write_volatile(&mut self.wcr, wcr);
		}
	}

	pub fn wcr_get_wt(&self) -> u8 { return ((self.wcr & 0b1111111100000000) >> 8) as u8; }
	pub fn wcr_set_wt(&mut self, timeout: u8) {
		let mut buf: u16 = self.wcr;
		buf &= 0b0000000011111111;
		buf |= (timeout as u16) << 8;
		self.wcr_write(buf);
	}

	pub fn wcr_get_wdw(&self) -> bool { return (self.wcr & 0b10000000) != 0; }
	pub fn wcr_set_wdw(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111101111111;
		if flag {
			buf |= 0b10000000;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_sre(&self) -> bool { return (self.wcr & 0b1000000) != 0; }
	pub fn wcr_set_sre(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111110111111;
		if flag {
			buf |= 0b1000000;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_wda(&self) -> bool { return (self.wcr & 0b100000) != 0; }
	pub fn wcr_set_wda(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111011111;
		if flag {
			buf |= 0b100000;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_srs(&self) -> bool { return (self.wcr & 0b10000) != 0; }
	pub fn wcr_set_srs(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111101111;
		if flag {
			buf |= 0b10000;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_wdt(&self) -> bool { return (self.wcr & 0b1000) != 0; }
	pub fn wcr_set_wdt(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111110111;
		if flag {
			buf |= 0b1000;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_wde(&self) -> bool { return (self.wcr & 0b100) != 0; }
	pub fn wcr_set_wde(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111111011;
		if flag {
			buf |= 0b100;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_wdbg(&self) -> bool { return (self.wcr & 0b10) != 0; }
	pub fn wcr_set_wdbg(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111111101;
		if flag {
			buf |= 0b10;
		}
		self.wcr_write(buf);
	}

	pub fn wcr_get_wdzst(&self) -> bool { return (self.wcr & 0b1) != 0; }
	pub fn wcr_set_wdzst(&mut self, flag: bool) {
		let mut buf: u16 = self.wcr;
		buf &= 0b1111111111111110;
		if flag {
			buf |= 0b1;
		}
		self.wcr_write(buf);
	}
}
