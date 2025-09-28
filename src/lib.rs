#![no_std]

extern crate alloc;

pub mod autocs;
pub mod clksel;
pub mod gate;
pub mod pll;
pub mod regs_gen;
pub mod softrst;
pub mod tools;

use log::debug;
use tock_registers::interfaces::{Readable, Writeable};

use crate::{
    autocs::ModeRegisters,
    clksel::ClkSelRegisters,
    pll::{AupllRegisters, CpllRegisters, GpllRegisters, NpllRegisters, V0pllRegisters},
    tools::{div_round_up, div_to_rate},
};
use core::ptr::NonNull;

pub const OFFSET: usize = 0x160;

pub const OSC_HZ: usize = 24 * 1000 * 1000;
pub const APLL_L_HZ: usize = 800 * 1000 * 1000;
pub const APLL_B_HZ: usize = 816 * 1000 * 1000;
pub const GPLL_HZ: usize = 1188 * 1000 * 1000;
pub const CPLL_HZ: usize = 1500 * 1000 * 1000;

pub const CCLK_EMMC_DIV_SHIFT: u32 = 8;
pub const CCLK_EMMC_DIV_MASK: u32 = 0x3f << CCLK_EMMC_DIV_SHIFT;
pub const CCLK_EMMC_SEL_SHIFT: u32 = 14;
pub const CCLK_EMMC_SEL_MASK: u32 = 3 << CCLK_EMMC_SEL_SHIFT;

pub const CCLK_EMMC_SEL_GPLL: u32 = 0;
pub const CCLK_EMMC_SEL_CPLL: u32 = 1;

pub const SCLK_SFC_SEL_CPLL: u32 = 0;
pub const SCLK_SFC_SEL_GPLL: u32 = 1;
pub const SCLK_SFC_SEL_24M: u32 = 2;

pub const CCLK_SRC_SDIO: u32 = 395;
pub const CCLK_EMMC: u32 = 300;
pub const BCLK_EMMC: u32 = 301;
pub const SCLK_SFC: u32 = 303;
pub const DCLK_DECOM: u32 = 150;

pub struct Rk3588Cru {
    addr: NonNull<u8>,
    cpll_hz: usize,
    gpll_hz: usize,
}

impl Rk3588Cru {
    pub fn new(addr: NonNull<u8>) -> Self {
        Self {
            addr,
            cpll_hz: CPLL_HZ,
            gpll_hz: GPLL_HZ,
        }
    }

    pub fn init(&self) {
        // Initialize the CRU if needed
    }

    pub fn registers(&self) -> &Rk3588CruRegisters {
        unsafe { &*(self.addr.as_ptr().add(OFFSET) as *const Rk3588CruRegisters) }
    }

    pub fn mmc_get_clk(&self, clk_id: u32) -> Result<usize, ()> {
        debug!("Getting clk_id {}", clk_id);

        let clksel = &self.registers().clksel;

        match clk_id {
            CCLK_SRC_SDIO => {
                todo!("Implement mmc_get_clk for CCLK_SRC_SDIO");
            }
            CCLK_EMMC => {
                let config = clksel.cru_clksel_con77.get();
                let div = (config & CCLK_EMMC_DIV_MASK) >> CCLK_EMMC_DIV_SHIFT;
                let sel = (config & CCLK_EMMC_SEL_MASK) >> CCLK_EMMC_SEL_SHIFT;
                let prate = if sel == CCLK_EMMC_SEL_GPLL {
                    self.gpll_hz
                } else if sel == CCLK_EMMC_SEL_CPLL {
                    self.cpll_hz
                } else {
                    OSC_HZ
                };

                return Ok(div_to_rate(prate, div));
            }
            BCLK_EMMC => {
                todo!("Implement mmc_get_clk for BCLK_EMMC");
            }
            SCLK_SFC => {
                todo!("Implement mmc_get_clk for SCLK_SFC");
            }
            DCLK_DECOM => {
                todo!("Implement mmc_get_clk for DCLK_DECOM");
            }
            _ => {
                panic!("Unsupported clk_id: {}", clk_id);
            }
        }
    }

    pub fn mmc_set_clk(&self, clk_id: u32, rate: usize) -> Result<usize, ()> {
        debug!("Setting clk_id {} to rate {}", clk_id, rate);

        let clksel = &self.registers().clksel;

        let (src_clk, div) = match clk_id {
            CCLK_SRC_SDIO => {
                todo!("Implement mmc_set_clk for CCLK_SRC_SDIO");
            }
            CCLK_EMMC => {
                if OSC_HZ % rate == 0 {
                    let div = div_round_up(OSC_HZ, rate);
                    (SCLK_SFC_SEL_24M, div)
                } else if self.cpll_hz % rate == 0 {
                    let div = div_round_up(self.cpll_hz, rate);
                    (SCLK_SFC_SEL_CPLL, div)
                } else {
                    let div = div_round_up(self.gpll_hz, rate);
                    (SCLK_SFC_SEL_GPLL, div)
                }
            }
            BCLK_EMMC => {
                todo!("Implement mmc_set_clk for BCLK_EMMC");
            }
            SCLK_SFC => {
                todo!("Implement mmc_set_clk for SCLK_SFC");
            }
            DCLK_DECOM => {
                todo!("Implement mmc_set_clk for DCLK_DECOM");
            }
            _ => {
                return Err(());
            }
        };

        match clk_id {
            CCLK_EMMC => {
                let new_value =
                    (src_clk << CCLK_EMMC_SEL_SHIFT) | (((div as u32) - 1) << CCLK_EMMC_DIV_SHIFT);
                let mask = CCLK_EMMC_SEL_MASK | CCLK_EMMC_DIV_MASK;
                let final_value = (mask | new_value) << 16 | new_value;

                debug!(
                    "CCLK_EMMC: src_clk {}, div {}, new_value {:#x}, final_value {:#x}",
                    src_clk, div, new_value, final_value
                );

                clksel.cru_clksel_con77.set(final_value);
            }
            _ => {
                return Err(());
            }
        }

        match self.mmc_get_clk(clk_id) {
            Ok(freq) => Ok(freq),
            Err(_) => Err(()),
        }
    }
}

#[repr(C)]
pub struct Rk3588CruRegisters {
    v0pll: V0pllRegisters,
    aupll: AupllRegisters,
    cpll: CpllRegisters,
    gpll: GpllRegisters,
    npll: NpllRegisters,
    _reserved0: [u8; 0x80],
    mode: ModeRegisters,
    clksel: ClkSelRegisters,
}
