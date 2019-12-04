MEMORY
{
	ITCM (rwx):  ORIGIN = 0x00000000, LENGTH = 512K
	DTCM (rwx):  ORIGIN = 0x20000000, LENGTH = 512K
	RAM (rwx):   ORIGIN = 0x20200000, LENGTH = 512K
	FLASH (rwx): ORIGIN = 0x60000000, LENGTH = 1984K
}

ENTRY(_IMAGE_VECTOR_TABLE)

SECTIONS
{
	.text.progmem : {
		KEEP(*(.flashconfig))
		FILL(0xFF)
		. = ORIGIN(FLASH) + 0x1000;
		KEEP(*(.ivt))
		KEEP(*(.bootdata))
		KEEP(*(.vectors))
		KEEP(*(.startup))
		*(.progmem*)
                . = ALIGN(4);
                KEEP(*(.init))
                __preinit_array_start = .;
                KEEP (*(.preinit_array))
                __preinit_array_end = .;
                __init_array_start = .;
                KEEP (*(.init_array))
                __init_array_end = .;
		. = ALIGN(16);
	} > FLASH	

	.text.itcm : {
		. = . + 32; /* MPU to trap NULL pointer deref */
		*(.fastrun)
		*(.text*)
		. = ALIGN(16);
	} > ITCM  AT> FLASH

	.data : {
		*(.rodata*)
		*(.data*)
		. = ALIGN(16);
	} > DTCM  AT> FLASH

	.bss ALIGN(4) : {
		*(.bss*)
		*(COMMON)
		. = ALIGN(64);
		*(.interrupt)
		. = ALIGN(32);
		. = . + 32; /* MPU to trap stack overflow */
	} > DTCM

	.bss.dma (NOLOAD) : {
		*(.dmabuffers)
		. = ALIGN(16);
	} > RAM

	/DISCARD/ : {
		*(.ARM.exidx)
	}

	_FLASH_IMAGE_LENGTH = SIZEOF(.text.progmem);
	
	_ITCM_BLOCK_COUNT = (SIZEOF(.text.itcm) + 0x7FFE) >> 15;
	_FLEXRAM_BANK_CONFIG = 0xAAAAAAAA | ((1 << (_ITCM_BLOCK_COUNT * 2)) - 1);
	_STACK_END = ORIGIN(DTCM) + ((16 - _ITCM_BLOCK_COUNT) << 15);

	_TEXT_START = ADDR(.text.itcm);
	_TEXT_END = ADDR(.text.itcm) + SIZEOF(.text.itcm);
	_TEXT_LEN = SIZEOF(.text.itcm);
	_TEXT_LOAD = LOADADDR(.text.itcm);
	
	_DATA_START = ADDR(.data);
	_DATA_END = ADDR(.data) + SIZEOF(.data);
	_DATA_LEN = SIZEOF(.data);
	_DATA_LOAD = LOADADDR(.data);

	_BSS_START = ADDR(.bss);
	_BSS_END = ADDR(.bss) + SIZEOF(.bss);
	_BSS_LEN = SIZEOF(.bss);

	/* Defined here to avoid rough casting in Rust */
	_INITIAL_STACK = 0x20010000;

	/* From Teensyduino */
	_heap_start = ADDR(.bss.dma) + SIZEOF(.bss.dma);
	_heap_end = ORIGIN(RAM) + LENGTH(RAM);

	_flashimagelen = SIZEOF(.text.progmem) + SIZEOF(.text.itcm) + SIZEOF(.data);
	_teensy_model_identifier = 0x24;

	.debug_info     0 : { *(.debug_info) }
	.debug_abbrev   0 : { *(.debug_abbrev) }
	.debug_line     0 : { *(.debug_line) }
	.debug_frame    0 : { *(.debug_frame) }
	.debug_str      0 : { *(.debug_str) }
	.debug_loc      0 : { *(.debug_loc) }
}

