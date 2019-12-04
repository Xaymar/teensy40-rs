#![allow(dead_code)]

// Clock Controller Module

use volatile::ReadOnly;
use volatile::Volatile;

pub const CCR: *mut Volatile<u32> = (0x400F_C000) as *mut Volatile<u32>;
pub const CSR: *mut ReadOnly<u32> = (0x400F_C004) as *mut ReadOnly<u32>;
pub const CCSR: *mut Volatile<u32> = (0x400F_C008) as *mut Volatile<u32>;
pub const CACRR: *mut Volatile<u32> = (0x400F_C00C) as *mut Volatile<u32>;
pub const CBCDR: *mut Volatile<u32> = (0x400F_C010) as *mut Volatile<u32>;
pub const CBCMR: *mut Volatile<u32> = (0x400F_C014) as *mut Volatile<u32>;
pub const CSCMR1: *mut Volatile<u32> = (0x400F_C018) as *mut Volatile<u32>;
pub const CSCMR2: *mut Volatile<u32> = (0x400F_C01C) as *mut Volatile<u32>;
pub const CSCDR1: *mut Volatile<u32> = (0x400F_C020) as *mut Volatile<u32>;
pub const CS1CDR: *mut Volatile<u32> = (0x400F_C024) as *mut Volatile<u32>;
pub const CS2CDR: *mut Volatile<u32> = (0x400F_C028) as *mut Volatile<u32>;
pub const CDCDR: *mut Volatile<u32> = (0x400F_C02C) as *mut Volatile<u32>;
pub const CSCDR2: *mut Volatile<u32> = (0x400F_C030) as *mut Volatile<u32>;
pub const CSCDR3: *mut Volatile<u32> = (0x400F_C038) as *mut Volatile<u32>;
pub const CDHIPR: *mut Volatile<u32> = (0x400F_C048) as *mut Volatile<u32>;
pub const CLPCR: *mut Volatile<u32> = (0x400F_C054) as *mut Volatile<u32>;
pub const CISR: *mut Volatile<u32> = (0x400F_C058) as *mut Volatile<u32>;
pub const CIMR: *mut Volatile<u32> = (0x400F_C05C) as *mut Volatile<u32>;
pub const CCOSR: *mut Volatile<u32> = (0x400F_C060) as *mut Volatile<u32>;
pub const CGPR: *mut Volatile<u32> = (0x400F_C064) as *mut Volatile<u32>;
pub const CCGR0: *mut Volatile<u32> = (0x400F_C068) as *mut Volatile<u32>;
pub const CCGR1: *mut Volatile<u32> = (0x400F_C06C) as *mut Volatile<u32>;
pub const CCGR2: *mut Volatile<u32> = (0x400F_C070) as *mut Volatile<u32>;
pub const CCGR3: *mut Volatile<u32> = (0x400F_C074) as *mut Volatile<u32>;
pub const CCGR4: *mut Volatile<u32> = (0x400F_C078) as *mut Volatile<u32>;
pub const CCGR5: *mut Volatile<u32> = (0x400F_C07C) as *mut Volatile<u32>;
pub const CCGR6: *mut Volatile<u32> = (0x400F_C080) as *mut Volatile<u32>;
pub const CCGR7: *mut Volatile<u32> = (0x400F_C084) as *mut Volatile<u32>;
pub const CMEOR: *mut Volatile<u32> = (0x400F_C088) as *mut Volatile<u32>;
