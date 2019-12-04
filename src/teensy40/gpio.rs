// 11 GPIO
#![allow(dead_code)]

use volatile::*;

// Maximum value for the id parameter to GPIO::new (0..COUNT)
pub const COUNT: usize = 8;
// Maximum value for the pin parameter (0..COUNT_PIN)
pub const COUNT_PIN: usize = 31;

pub enum InterruptConfiguration {
	LowLevel,
	HighLevel,
	RisingEdge,
	FallingEdge,
}

#[repr(C, packed)]
pub struct GPIO {
	data: Volatile<u32>,
	direction: Volatile<u32>,
	pad_status: ReadOnly<u32>,
	interrupt_configuration_1: Volatile<u32>,
	interrupt_configuration_2: Volatile<u32>,
	interrupt_mask: Volatile<u32>,
	interrupt_status: Volatile<u32>,
	interrupt_edge_selection: Volatile<u32>,
	_pad0: [u8; 100],
	data_set: WriteOnly<u32>,
	data_clear: WriteOnly<u32>,
	data_toggle: WriteOnly<u32>,
}

impl GPIO {
	pub fn new(id: usize) -> &'static mut Self {
		unsafe {
			match id {
				0 | 1 | 2 | 3 => return &mut *((0x401B_8000 + (0x4000 * id)) as *mut GPIO),
				4 => return &mut *(0x400C_0000 as *mut GPIO),
				5 | 6 | 7 | 8 => return &mut *((0x4200_0000 + (0x4000 * (id - 4))) as *mut GPIO),
				_ => panic!("Invalid Id"),
			}
		}
	}

	fn get_pin_mask(pin: usize) -> u32 { return 1 << pin; }

	pub fn set_pin_state(&mut self, pin: usize, state: bool) {
		unsafe {
			let mask = GPIO::get_pin_mask(pin);
			match state {
				// false => self.data.write(self.data.read() & !mask),
				// true => self.data.write(self.data.read() | mask),
				false => self.data_clear.write(mask),
				true => self.data_set.write(mask),
			}
		}
	}

	pub fn get_pin_state(&mut self, pin: usize) -> bool {
		unsafe {
			let mask = GPIO::get_pin_mask(pin);
			match self.data.read() & mask {
				0 => false,
				_ => true,
			}
		}
	}

	pub fn set_pin_output(&mut self, pin: usize, is_output: bool) {
		unsafe {
			let mask = GPIO::get_pin_mask(pin);
			match is_output {
				false => self.direction.write(self.direction.read() & !mask),
				true => self.direction.write(self.direction.read() | mask),
			}
		}
	}

	pub fn is_pin_output(&mut self, pin: usize) -> bool {
		unsafe {
			let mask = GPIO::get_pin_mask(pin);
			return match self.direction.read() & mask {
				0 => false,
				_ => true,
			};
		}
	}

	pub fn get_pin_pad_status(&mut self, pin: usize) -> bool {
		unsafe {
			let mask = GPIO::get_pin_mask(pin);
			return match self.direction.read() & mask {
				0 => false,
				_ => true,
			};
		}
	}

	pub fn set_pin_interrupt_configuration(&mut self, pin: usize, config: InterruptConfiguration) {
		unsafe {
			let val: u32;
			match config {
				InterruptConfiguration::LowLevel => val = 0b00,
				InterruptConfiguration::HighLevel => val = 0b01,
				InterruptConfiguration::RisingEdge => val = 0b10,
				InterruptConfiguration::FallingEdge => val = 0b11,
			}

			if pin < 16 {
				// interrupt_configuration_1
				let offset = pin as u32 * 2;
				self.interrupt_configuration_1
					.write((self.interrupt_configuration_1.read() & !(3 << offset)) | (val << offset));
			} else {
				// interrupt_configuration_2
				let offset = (pin as u32 - 16) * 2;
				self.interrupt_configuration_2
					.write((self.interrupt_configuration_2.read() & !(3 << offset)) | (val << offset));
			}
		}
	}

	pub fn get_pin_interrupt_configuration(&mut self, pin: usize) -> InterruptConfiguration {
		unsafe {
			let val: u32;
			let offset: u32;
			if pin < 16 {
				// interrupt_configuration_1
				offset = pin as u32 * 2;
				val = self.interrupt_configuration_1.read();
			} else {
				// interrupt_configuration_2
				offset = (pin as u32 - 16) * 2;
				val = self.interrupt_configuration_2.read();
			}
			return match (val & (3 << offset)) >> offset {
				0b00 => InterruptConfiguration::LowLevel,
				0b01 => InterruptConfiguration::HighLevel,
				0b10 => InterruptConfiguration::RisingEdge,
				0b11 => InterruptConfiguration::FallingEdge,
				_ => panic!("Rust generated impossible case"),
			};
		}
	}

	pub fn set_pin_interrupt_enabled(&mut self, pin: usize, enabled: bool) {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			match enabled {
				false => self.interrupt_mask.write(self.interrupt_mask.read() & !mask),
				true => self.interrupt_mask.write(self.interrupt_mask.read() | mask),
			}
		}
	}

	pub fn get_pin_interrupt_enabled(&mut self, pin: usize) -> bool {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			return match self.interrupt_mask.read() & mask {
				0 => false,
				_ => true,
			};
		}
	}

	pub fn get_pin_interrupt_state(&mut self, pin: usize) -> bool {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			return match self.interrupt_status.read() & mask {
				0 => false,
				_ => true,
			};
		}
	}

	pub fn clear_pin_interrupt_state(&mut self, pin: usize) {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			self.interrupt_status.write(mask)
		}
	}

	pub fn set_pin_interrupt_edge_select_enabled(&mut self, pin: usize, enabled: bool) {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			match enabled {
				true => self.interrupt_edge_selection.write(self.interrupt_edge_selection.read() & !mask),
				false => self.interrupt_edge_selection.write(self.interrupt_edge_selection.read() | mask),
			}
		}
	}
	
	pub fn get_pin_interrupt_edge_select_enabled(&mut self, pin: usize) -> bool {
		unsafe {
			let mask: u32 = GPIO::get_pin_mask(pin);
			return match self.interrupt_edge_selection.read() & mask {
				0 => true,
				_ => false,
			};
		}
	}
}
