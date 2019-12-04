#![feature(allocator)]
#![no_std]
//#![allocator]

/*
use core;

const BLOCK_SIZE: usize = 256;
const THRESHOLD: usize = 8192;

extern "C" {
	static __RAM_BASE: *mut usize;
	static __RAM_SIZE: usize;
}

fn get_block_count() -> usize {
	unsafe {
		let memory: usize = __RAM_SIZE;
		let blocks: usize = memory / BLOCK_SIZE / 8;
		return blocks;
	}
}

fn get_total_block_size() -> usize {
	let mut size: usize = get_block_count() * BLOCK_SIZE;
	if size % BLOCK_SIZE > 0 {
		size = ((size / BLOCK_SIZE) + 1) * BLOCK_SIZE;
	}
	return size;
}

fn get_block_offset(index: usize) -> usize {
	return (index / 8) as usize;
}

fn get_block_mask(index: usize) -> u8 {
	return (0b1 << (index % 8)) as u8;
}

fn get_block_ptr(offset: usize) -> *mut u8 {
	unsafe {
		return (__RAM_BASE as usize + offset) as *mut u8;
	}
}

unsafe fn read_block(offset: usize) -> u8 {
	return core::ptr::read_volatile(get_block_ptr(offset));
}

unsafe fn write_block(offset: usize, value: u8) {
	core::ptr::write_volatile(get_block_ptr(offset), value);
}

fn is_block_marked(index: usize) -> bool {
	let off = get_block_offset(index);
	let mask = get_block_mask(index);
	unsafe {
		let block = read_block(off);
		return (block & mask) != 0;
	}
}

fn set_block_marked(index: usize, marked: bool) {
	let off = get_block_offset(index);
	let mask = get_block_mask(index);
	unsafe {
		let mut block = read_block(off);
		block &= !mask;
		if marked {
			block |= mask;
		}
		write_block(off, block);
	}
}

fn initialize_allocator() {
	if is_block_marked(0) {
		return;
	}
	let _i: usize;
	for _i in 0..get_block_count() {
		unsafe { write_block(get_block_offset(_i), 0xFF); }
	}
}

fn get_size_in_blocks(size: usize) -> usize {
	let t0 = size / BLOCK_SIZE;
	if size % BLOCK_SIZE > 0 {
		return 1 + t0;
	} else {
		return t0;
	}
}

fn get_continuous_blocks(blocks: usize) -> usize {

}

fn allocate(size: usize) {
	let is_far: bool = size > THRESHOLD;

}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
	initialize_allocator();
	
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
	initialize_allocator();
	
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(
	ptr: *mut u8,
	_old_size: usize,
	size: usize,
	_align: usize,
) -> *mut u8 {
	initialize_allocator();
	
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(
	_ptr: *mut u8,
	old_size: usize,
	_size: usize,
	_align: usize,
) -> usize {
	// Not supported
	old_size
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
	size
}
*/