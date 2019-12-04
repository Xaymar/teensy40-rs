#![allow(dead_code)]

// System Control Block

use volatile::ReadOnly;
use volatile::Volatile;
use volatile::WriteOnly;

// Auxiliary Control Register
pub const ACTLR: *mut Volatile<u32> = (0xE000_E008) as *mut Volatile<u32>;

// SysTick Control and Status Register
pub const SYST_CSR: *mut Volatile<u32> = (0xE000_E010) as *mut Volatile<u32>;
// SysTick Reload Value Register
pub const SYST_RVR: *mut Volatile<u32> = (0xE000_E014) as *mut Volatile<u32>;
// SysTick Current Value Register
pub const SYST_CVR: *mut Volatile<u32> = (0xE000_E018) as *mut Volatile<u32>;
// SysTick Calibration Value Register
pub const SYST_CALIB: *mut ReadOnly<u32> = (0xE000_E01C) as *mut ReadOnly<u32>;

// CPUID Base Register
pub const CPUID: *mut ReadOnly<u32> = (0xE000_ED00) as *mut ReadOnly<u32>;
// Interrupt Control and State Register
pub const ICSR: *mut Volatile<u32> = (0xE000_ED04) as *mut Volatile<u32>;
// Application Interrupt and Reset Control Register
pub const AIRCR: *mut Volatile<u32> = (0xE000_ED0C) as *mut Volatile<u32>;
// System Control Register
pub const SCR: *mut Volatile<u32> = (0xE000_ED10) as *mut Volatile<u32>;
// Configuration and Control Register
pub const CCR: *mut Volatile<u32> = (0xE000_ED14) as *mut Volatile<u32>;
// System Handler Priority Register 1
pub const SHPR1: *mut Volatile<u32> = (0xE000_ED18) as *mut Volatile<u32>;
// System Handler Priority Register 2
pub const SHPR2: *mut Volatile<u32> = (0xE000_ED1C) as *mut Volatile<u32>;
// System Handler Priority Register 3
pub const SHPR3: *mut Volatile<u32> = (0xE000_ED20) as *mut Volatile<u32>;
// System Handler Control and State Register
pub const SHCSR: *mut Volatile<u32> = (0xE000_ED24) as *mut Volatile<u32>;
// Configurable Fault Status Registers
pub const CFSR: *mut Volatile<u32> = (0xE000_ED28) as *mut Volatile<u32>;
// HardFault Status Register
pub const HFSR: *mut Volatile<u32> = (0xE000_ED2C) as *mut Volatile<u32>;
// Debug Fault Status Register
pub const DFSR: *mut Volatile<u32> = (0xE000_ED30) as *mut Volatile<u32>;
// MemManage Fault Address Register
pub const MMFAR: *mut Volatile<u32> = (0xE000_ED34) as *mut Volatile<u32>;
// BusFault Address Register
pub const BFAR: *mut Volatile<u32> = (0xE000_ED38) as *mut Volatile<u32>;
// Processor Feature Register 0
pub const ID_PFR0: *mut ReadOnly<u32> = (0xE000_ED40) as *mut ReadOnly<u32>;
// Processor Feature Register 1
pub const ID_PFR1: *mut ReadOnly<u32> = (0xE000_ED44) as *mut ReadOnly<u32>;
// Debug Feature Register 0
pub const ID_DFR0: *mut ReadOnly<u32> = (0xE000_ED48) as *mut ReadOnly<u32>;
// Auxiliary Featur Register 0
pub const ID_AFR0: *mut ReadOnly<u32> = (0xE000_ED4C) as *mut ReadOnly<u32>;
// Memory Model Feature Register 0
pub const ID_MMFR0: *mut ReadOnly<u32> = (0xE000_ED50) as *mut ReadOnly<u32>;
// Memory Model Feature Register 1
pub const ID_MMFR1: *mut ReadOnly<u32> = (0xE000_ED54) as *mut ReadOnly<u32>;
// Memory Model Feature Register 2
pub const ID_MMFR2: *mut ReadOnly<u32> = (0xE000_ED58) as *mut ReadOnly<u32>;
// Memory Model Feature Register 3
pub const ID_MMFR3: *mut ReadOnly<u32> = (0xE000_ED5C) as *mut ReadOnly<u32>;
// Instruction Set Attributes Register 0
pub const ID_ISAR0: *mut ReadOnly<u32> = (0xE000_ED60) as *mut ReadOnly<u32>;
// Instruction Set Attributes Register 1
pub const ID_ISAR1: *mut ReadOnly<u32> = (0xE000_ED64) as *mut ReadOnly<u32>;
// Instruction Set Attributes Register 2
pub const ID_ISAR2: *mut ReadOnly<u32> = (0xE000_ED68) as *mut ReadOnly<u32>;
// Instruction Set Attributes Register 3
pub const ID_ISAR3: *mut ReadOnly<u32> = (0xE000_ED6C) as *mut ReadOnly<u32>;
// Instruction Set Attributes Register 4
pub const ID_ISAR4: *mut ReadOnly<u32> = (0xE000_ED70) as *mut ReadOnly<u32>;
// Cache Level ID Register
pub const CLIDR: *mut ReadOnly<u32> = (0xE000_ED78) as *mut ReadOnly<u32>;
// Cache Type Register
pub const CTR: *mut ReadOnly<u32> = (0xE000_ED7C) as *mut ReadOnly<u32>;
// Cache Size ID Register
pub const CCSIDR: *mut ReadOnly<u32> = (0xE000_ED80) as *mut ReadOnly<u32>;
// Cache Size Selection Register
pub const CSSELR: *mut Volatile<u32> = (0xE000_ED84) as *mut Volatile<u32>;
// Coprocessor Access Control Register
pub const CPACR: *mut Volatile<u32> = (0xE000_ED88) as *mut Volatile<u32>;

// Instruction Cache Invalidate All to Point Of Unification (PoU)
pub const ICIALLU: *mut WriteOnly<u32> = (0xE000_EF50) as *mut WriteOnly<u32>;
// Instruction cache invalidate by address to PoU
pub const ICIMVAU: *mut WriteOnly<u32> = (0xE000_EF58) as *mut WriteOnly<u32>;
// Data cache invalidate by address to Point of Coherency (PoC)
pub const DCIMVAC: *mut WriteOnly<u32> = (0xE000_EF5C) as *mut WriteOnly<u32>;
// Data cache clean by set/way
pub const DCISW: *mut WriteOnly<u32> = (0xE000_EF60) as *mut WriteOnly<u32>;
// Data cache by address to PoU
pub const DCCMVAU: *mut WriteOnly<u32> = (0xE000_EF64) as *mut WriteOnly<u32>;
// Data cache clean by address to PoC
pub const DCCMVAC: *mut WriteOnly<u32> = (0xE000_EF68) as *mut WriteOnly<u32>;
// Data cache clean by set/way
pub const DCCSW: *mut WriteOnly<u32> = (0xE000_EF6C) as *mut WriteOnly<u32>;
// Data cache clean and invalidate by address to PoC
pub const DCCIMVAC: *mut WriteOnly<u32> = (0xE000_EF70) as *mut WriteOnly<u32>;
// Data cache clean and invalidate by set/way
pub const DCCISW: *mut WriteOnly<u32> = (0xE000_EF74) as *mut WriteOnly<u32>;
// Not Implemented, Unknown Access (Read As Zero, Write Ignored)
//pub const BPIALL: *mut Volatile<u32> = (0xE000_EF78) as *mut Volatile<u32>;

// Instruction Tightly-Coupled Memory Control Register
pub const CM7_ITCMCR: *mut Volatile<u32> = (0xE000_EF90) as *mut Volatile<u32>;
// Data Tightly-Coupled Memory Control Register
pub const CM7_DTCMCR: *mut Volatile<u32> = (0xE000_EF94) as *mut Volatile<u32>;
// AHBP Control Register
pub const CM7_AHBPCR: *mut Volatile<u32> = (0xE000_EF98) as *mut Volatile<u32>;
// L1 Cache Control Register
pub const CM7_CACR: *mut Volatile<u32> = (0xE000_EF9C) as *mut Volatile<u32>;
// AHB Slave Control Register
pub const CM7_AHBSCR: *mut Volatile<u32> = (0xE000_EFA0) as *mut Volatile<u32>;
// Auxiliary Bus Fault Status Register
pub const CM7_ABFSR: *mut Volatile<u32> = (0xE000_EFA8) as *mut Volatile<u32>;

// Instruction Error Bank Register 0
pub const IEBR0: *mut Volatile<u32> = (0xE000_EFB0) as *mut Volatile<u32>;
// Instruction Error Bank Register 1
pub const IEBR1: *mut Volatile<u32> = (0xE000_EFB4) as *mut Volatile<u32>;
// Data Error Bank Register 0
pub const DEBR0: *mut Volatile<u32> = (0xE000_EFB8) as *mut Volatile<u32>;
// Data Error Bank Register 1
pub const DEBR1: *mut Volatile<u32> = (0xE000_EFBC) as *mut Volatile<u32>;

// Periphal ID 4
pub const PID4: *mut Volatile<u32> = (0xE000_EFD0) as *mut Volatile<u32>;
// Periphal ID 5
pub const PID5: *mut Volatile<u32> = (0xE000_EFD4) as *mut Volatile<u32>;
// Periphal ID 6
pub const PID6: *mut Volatile<u32> = (0xE000_EFD8) as *mut Volatile<u32>;
// Periphal ID 7
pub const PID7: *mut Volatile<u32> = (0xE000_EFDC) as *mut Volatile<u32>;
// Periphal ID 0
pub const PID0: *mut Volatile<u32> = (0xE000_EFE0) as *mut Volatile<u32>;
// Periphal ID 1
pub const PID1: *mut Volatile<u32> = (0xE000_EFE4) as *mut Volatile<u32>;
// Periphal ID 2
pub const PID2: *mut Volatile<u32> = (0xE000_EFE8) as *mut Volatile<u32>;
// Periphal ID 3
pub const PID3: *mut Volatile<u32> = (0xE000_EFEC) as *mut Volatile<u32>;

// Component ID 0
pub const CID0: *mut Volatile<u32> = (0xE000_EFF0) as *mut Volatile<u32>;
// Component ID 1
pub const CID1: *mut Volatile<u32> = (0xE000_EFF4) as *mut Volatile<u32>;
// Component ID 2
pub const CID2: *mut Volatile<u32> = (0xE000_EFF8) as *mut Volatile<u32>;
// Component ID 3
pub const CID3: *mut Volatile<u32> = (0xE000_EFFC) as *mut Volatile<u32>;
