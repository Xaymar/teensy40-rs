#![allow(dead_code)]

use super::scb;

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn enable() {
	super::data_synchronization_barrier();
	super::instruction_synchronization_barrier();
	(*scb::ICIALLU).write(0);
	super::data_synchronization_barrier();
	super::instruction_synchronization_barrier();
	(*scb::CCR).write((*scb::CCR).read() | (1 << 16) | (1 << 17));
}

#[link_section = ".fastrun"]
#[inline(always)]
pub unsafe fn disable() {
	super::data_synchronization_barrier();
	super::instruction_synchronization_barrier();
	(*scb::ICIALLU).write(0);
	super::data_synchronization_barrier();
	super::instruction_synchronization_barrier();
	(*scb::CCR).write((*scb::CCR).read() | (1 << 16) | (1 << 17));
}
