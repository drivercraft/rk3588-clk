use crate::generate_register_bitfields;
use tock_registers::{register_bitfields, registers::ReadWrite};

// Start Adderess: 0x0A00
#[repr(C)]
pub struct SoftReset {
    _reserved0: u32,
    pub cru_softrst_con01: ReadWrite<u32, CRU_SOFTRST_CON01::Register>,
    pub cru_softrst_con02: ReadWrite<u32, CRU_SOFTRST_CON02::Register>,
    pub cru_softrst_con03: ReadWrite<u32, CRU_SOFTRST_CON03::Register>,
    pub cru_softrst_con04: ReadWrite<u32, CRU_SOFTRST_CON04::Register>,
    pub cru_softrst_con05: ReadWrite<u32, CRU_SOFTRST_CON05::Register>,
    pub cru_softrst_con06: ReadWrite<u32, CRU_SOFTRST_CON06::Register>,
    pub cru_softrst_con07: ReadWrite<u32, CRU_SOFTRST_CON07::Register>,
    pub cru_softrst_con08: ReadWrite<u32, CRU_SOFTRST_CON08::Register>,
    pub cru_softrst_con09: ReadWrite<u32, CRU_SOFTRST_CON09::Register>,
    pub cru_softrst_con10: ReadWrite<u32, CRU_SOFTRST_CON10::Register>,
    pub cru_softrst_con11: ReadWrite<u32, CRU_SOFTRST_CON11::Register>,
    pub cru_softrst_con12: ReadWrite<u32, CRU_SOFTRST_CON12::Register>,
    pub cru_softrst_con13: ReadWrite<u32, CRU_SOFTRST_CON13::Register>,
    pub cru_softrst_con14: ReadWrite<u32, CRU_SOFTRST_CON14::Register>,
    pub cru_softrst_con15: ReadWrite<u32, CRU_SOFTRST_CON15::Register>,
    pub cru_softrst_con16: ReadWrite<u32, CRU_SOFTRST_CON16::Register>,
    pub cru_softrst_con17: ReadWrite<u32, CRU_SOFTRST_CON17::Register>,
    pub cru_softrst_con18: ReadWrite<u32, CRU_SOFTRST_CON18::Register>,
    pub cru_softrst_con19: ReadWrite<u32, CRU_SOFTRST_CON19::Register>,
    pub cru_softrst_con20: ReadWrite<u32, CRU_SOFTRST_CON20::Register>,
    pub cru_softrst_con21: ReadWrite<u32, CRU_SOFTRST_CON21::Register>,
    pub cru_softrst_con22: ReadWrite<u32, CRU_SOFTRST_CON22::Register>,
    pub cru_softrst_con23: ReadWrite<u32, CRU_SOFTRST_CON23::Register>,
    pub cru_softrst_con24: ReadWrite<u32, CRU_SOFTRST_CON24::Register>,
    pub cru_softrst_con25: ReadWrite<u32, CRU_SOFTRST_CON25::Register>,
    pub cru_softrst_con26: ReadWrite<u32, CRU_SOFTRST_CON26::Register>,
    pub cru_softrst_con27: ReadWrite<u32, CRU_SOFTRST_CON27::Register>,
    pub cru_softrst_con28: ReadWrite<u32, CRU_SOFTRST_CON28::Register>,
    pub cru_softrst_con29: ReadWrite<u32, CRU_SOFTRST_CON29::Register>,
    pub cru_softrst_con30: ReadWrite<u32, CRU_SOFTRST_CON30::Register>,
    pub cru_softrst_con31: ReadWrite<u32, CRU_SOFTRST_CON31::Register>,
    pub cru_softrst_con32: ReadWrite<u32, CRU_SOFTRST_CON32::Register>,
    pub cru_softrst_con33: ReadWrite<u32, CRU_SOFTRST_CON33::Register>,
    pub cru_softrst_con34: ReadWrite<u32, CRU_SOFTRST_CON34::Register>,
    pub cru_softrst_con35: ReadWrite<u32, CRU_SOFTRST_CON35::Register>,
    _reserved1: u32,
    // pub cru_softrst_con36: ReadWrite<u32, CRU_SOFTRST_CON36::Register>,
    pub cru_softrst_con37: ReadWrite<u32, CRU_SOFTRST_CON37::Register>,
    _reserved2: [u32; 2],
    // pub cru_softrst_con38: ReadWrite<u32, CRU_SOFTRST_CON38::Register>,
    // pub cru_softrst_con39: ReadWrite<u32, CRU_SOFTRST_CON39::Register>,
    pub cru_softrst_con40: ReadWrite<u32, CRU_SOFTRST_CON40::Register>,
    pub cru_softrst_con41: ReadWrite<u32, CRU_SOFTRST_CON41::Register>,
    pub cru_softrst_con42: ReadWrite<u32, CRU_SOFTRST_CON42::Register>,
    pub cru_softrst_con43: ReadWrite<u32, CRU_SOFTRST_CON43::Register>,
    pub cru_softrst_con44: ReadWrite<u32, CRU_SOFTRST_CON44::Register>,
    pub cru_softrst_con45: ReadWrite<u32, CRU_SOFTRST_CON45::Register>,
    _reserved3: u32,
    // pub cru_softrst_con46: ReadWrite<u32, CRU_SOFTRST_CON46::Register>,
    pub cru_softrst_con47: ReadWrite<u32, CRU_SOFTRST_CON47::Register>,
    pub cru_softrst_con48: ReadWrite<u32, CRU_SOFTRST_CON48::Register>,
    pub cru_softrst_con49: ReadWrite<u32, CRU_SOFTRST_CON49::Register>,
    pub cru_softrst_con50: ReadWrite<u32, CRU_SOFTRST_CON50::Register>,
    pub cru_softrst_con51: ReadWrite<u32, CRU_SOFTRST_CON51::Register>,
    pub cru_softrst_con52: ReadWrite<u32, CRU_SOFTRST_CON52::Register>,
    pub cru_softrst_con53: ReadWrite<u32, CRU_SOFTRST_CON53::Register>,
    _reserved4: u32,
    // pub cru_softrst_con54: ReadWrite<u32, CRU_SOFTRST_CON54::Register>,
    pub cru_softrst_con55: ReadWrite<u32, CRU_SOFTRST_CON55::Register>,
    pub cru_softrst_con56: ReadWrite<u32, CRU_SOFTRST_CON56::Register>,
    pub cru_softrst_con57: ReadWrite<u32, CRU_SOFTRST_CON57::Register>,
    _reserved5: u32,
    // pub cru_softrst_con58: ReadWrite<u32, CRU_SOFTRST_CON58::Register>,
    pub cru_softrst_con59: ReadWrite<u32, CRU_SOFTRST_CON59::Register>,
    pub cru_softrst_con60: ReadWrite<u32, CRU_SOFTRST_CON60::Register>,
    pub cru_softrst_con61: ReadWrite<u32, CRU_SOFTRST_CON61::Register>,
    pub cru_softrst_con62: ReadWrite<u32, CRU_SOFTRST_CON62::Register>,
    pub cru_softrst_con63: ReadWrite<u32, CRU_SOFTRST_CON63::Register>,
    pub cru_softrst_con64: ReadWrite<u32, CRU_SOFTRST_CON64::Register>,
    pub cru_softrst_con65: ReadWrite<u32, CRU_SOFTRST_CON65::Register>,
    pub cru_softrst_con66: ReadWrite<u32, CRU_SOFTRST_CON66::Register>,
    pub cru_softrst_con67: ReadWrite<u32, CRU_SOFTRST_CON67::Register>,
    pub cru_softrst_con68: ReadWrite<u32, CRU_SOFTRST_CON68::Register>,
    pub cru_softrst_con69: ReadWrite<u32, CRU_SOFTRST_CON69::Register>,
    pub cru_softrst_con70: ReadWrite<u32, CRU_SOFTRST_CON70::Register>,
    _reserved6: u32,
    // pub cru_softrst_con71: ReadWrite<u32, CRU_SOFTRST_CON71::Register>,
    pub cru_softrst_con72: ReadWrite<u32, CRU_SOFTRST_CON72::Register>,
    pub cru_softrst_con73: ReadWrite<u32, CRU_SOFTRST_CON73::Register>,
    pub cru_softrst_con74: ReadWrite<u32, CRU_SOFTRST_CON74::Register>,
    pub cru_softrst_con75: ReadWrite<u32, CRU_SOFTRST_CON75::Register>,
    pub cru_softrst_con76: ReadWrite<u32, CRU_SOFTRST_CON76::Register>,
    pub cru_softrst_con77: ReadWrite<u32, CRU_SOFTRST_CON77::Register>,
}

// CRU_SOFTRST_CON01  0x0A04;
generate_register_bitfields!(
    CRU_SOFTRST_CON01,
    u32,
    [
        (reserved0, 0, 3, []),
        (aresetn_top_biu, 3, 1, []),
        (presetn_top_biu, 4, 1, []),
        (reserved1, 5, 1, []),
        (presetn_csiphy0, 6, 1, []),
        (reserved2, 7, 1, []),
        (presetn_csiphy1, 8, 1, []),
        (reserved3, 9, 6, []),
        (aresetn_top_m500_biu, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON02  0x0A08;
generate_register_bitfields!(
    CRU_SOFTRST_CON02,
    u32,
    [
        (reseraresetn_top_m400_biuved, 0, 1, []),
        (aresetn_top_s200_biu, 1, 1, []),
        (aresetn_top_s400_biu, 2, 1, []),
        (aresetn_top_m300_biu, 3, 1, []),
        (reserved0, 4, 4, []),
        (resetn_usbdp_combo_phy0_init, 8, 1, []),
        (resetn_usbdp_combo_phy0_cmn, 9, 1, []),
        (resetn_usbdp_combo_phy0_lane, 10, 1, []),
        (resetn_usbdp_combo_phy0_pcs, 11, 1, []),
        (reserved1, 12, 3, []),
        (resetn_usbdp_combo_phy1_init, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON03  0x0A0C;
generate_register_bitfields!(
    CRU_SOFTRST_CON03,
    u32,
    [
        (resetn_usbdp_combo_phy1_cmn, 0, 1, []),
        (resetn_usbdp_combo_phy1_lane, 1, 1, []),
        (resetn_usbdp_combo_phy1_pcs, 2, 1, []),
        (reserved, 3, 11, []),
        (presetn_mipi_dcphy0, 14, 1, []),
        (presetn_mipi_dcphy0_grf, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON04  0x0A10;
generate_register_bitfields!(
    CRU_SOFTRST_CON04,
    u32,
    [
        (reserved0, 0, 3, []),
        (presetn_mipi_dcphy1, 3, 1, []),
        (presetn_mipi_dcphy1_grf, 4, 1, []),
        (presetn_apb2asb_slv_cdphy, 5, 1, []),
        (presetn_apb2asb_slv_csiphy, 6, 1, []),
        (presetn_apb2asb_slv_vccio3_5, 7, 1, []),
        (presetn_apb2asb_slv_vccio6, 8, 1, []),
        (presetn_apb2asb_slv_emmcio, 9, 1, []),
        (presetn_apb2asb_slv_ioc_top, 10, 1, []),
        (presetn_apb2asb_slv_ioc_right, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON05  0x0A14;
generate_register_bitfields!(
    CRU_SOFTRST_CON05,
    u32,
    [
        (presetn_cru, 0, 1, []),
        (reserved0, 1, 6, []),
        (aresetn_channel_secure2vo1usb, 7, 1, []),
        (aresetn_channel_secure2center, 8, 1, []),
        (reserved1, 9, 5, []),
        (hresetn_channel_secure2vo1usb, 14, 1, []),
        (hresetn_channel_secure2center, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON06  0x0A18;
generate_register_bitfields!(
    CRU_SOFTRST_CON06,
    u32,
    [
        (presetn_channel_secure2vo1usb, 0, 1, []),
        (presetn_channel_secure2center, 1, 1, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON07  0x0A1C;
generate_register_bitfields!(
    CRU_SOFTRST_CON07,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_audio_biu, 2, 1, []),
        (presetn_audio_biu, 3, 1, []),
        (hresetn_i2s0_8ch, 4, 1, []),
        (reserved1, 5, 2, []),
        (mresetn_i2s0_8ch_tx, 7, 1, []),
        (reserved2, 8, 2, []),
        (mresetn_i2s0_8ch_rx, 10, 1, []),
        (presetn_acdcdig, 11, 1, []),
        (hresetn_i2s2_2ch, 12, 1, []),
        (hresetn_i2s3_2ch, 13, 1, []),
        (reserved3, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON08  0x0A20;
generate_register_bitfields!(
    CRU_SOFTRST_CON08,
    u32,
    [
        (mresetn_i2s2_2ch, 0, 1, []),
        (reserved0, 1, 2, []),
        (mresetn_i2s3_2ch, 3, 1, []),
        (resetn_dac_acdcdig, 4, 1, []),
        (reserved1, 5, 9, []),
        (hresetn_spdif0, 14, 1, []),
        (reserved2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON09  0x0A24;
generate_register_bitfields!(
    CRU_SOFTRST_CON09,
    u32,
    [
        (reserved0, 0, 1, []),
        (mresetn_spdif0, 1, 1, []),
        (hresetn_spdif1, 2, 1, []),
        (reserved1, 3, 2, []),
        (mresetn_spdif1, 5, 1, []),
        (hresetn_pdm1, 6, 1, []),
        (resetn_pdm1, 7, 1, []),
        (reserved2, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON10  0x0A28;
generate_register_bitfields!(
    CRU_SOFTRST_CON10,
    u32,
    [
        (reserved, 0, 1, []),
        (aresetn_bus_biu, 1, 1, []),
        (presetn_bus_biu, 2, 1, []),
        (aresetn_gic, 3, 1, []),
        (aresetn_gic_dbg, 4, 1, []),
        (aresetn_dmac0, 5, 1, []),
        (aresetn_dmac1, 6, 1, []),
        (aresetn_dmac2, 7, 1, []),
        (presetn_i2c1, 8, 1, []),
        (presetn_i2c2, 9, 1, []),
        (presetn_i2c3, 10, 1, []),
        (presetn_i2c4, 11, 1, []),
        (presetn_i2c5, 12, 1, []),
        (presetn_i2c6, 13, 1, []),
        (presetn_i2c7, 14, 1, []),
        (presetn_i2c8, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON11  0x0A2C;
generate_register_bitfields!(
    CRU_SOFTRST_CON11,
    u32,
    [
        (resetn_i2c1, 0, 1, []),
        (resetn_i2c2, 1, 1, []),
        (resetn_i2c3, 2, 1, []),
        (resetn_i2c4, 3, 1, []),
        (resetn_i2c5, 4, 1, []),
        (resetn_i2c6, 5, 1, []),
        (resetn_i2c7, 6, 1, []),
        (resetn_i2c8, 7, 1, []),
        (presetn_can0, 8, 1, []),
        (resetn_can0, 9, 1, []),
        (presetn_can1, 10, 1, []),
        (resetn_can1, 11, 1, []),
        (presetn_can2, 12, 1, []),
        (resetn_can2, 13, 1, []),
        (presetn_saradc, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON12  0x0A30;
generate_register_bitfields!(
    CRU_SOFTRST_CON12,
    u32,
    [
        (presetn_tsadc, 0, 1, []),
        (resetn_tsadc, 1, 1, []),
        (presetn_uart1, 2, 1, []),
        (presetn_uart2, 3, 1, []),
        (presetn_uart3, 4, 1, []),
        (presetn_uart4, 5, 1, []),
        (presetn_uart5, 6, 1, []),
        (presetn_uart6, 7, 1, []),
        (presetn_uart7, 8, 1, []),
        (presetn_uart8, 9, 1, []),
        (presetn_uart9, 10, 1, []),
        (reserved0, 11, 2, []),
        (sresetn_uart1, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON13  0x0A34;
generate_register_bitfields!(
    CRU_SOFTRST_CON13,
    u32,
    [
        (sresetn_uart2, 0, 1, []),
        (reserved0, 1, 2, []),
        (sresetn_uart3, 3, 1, []),
        (reserved1, 4, 2, []),
        (sresetn_uart4, 6, 1, []),
        (reserved2, 7, 2, []),
        (sresetn_uart5, 9, 1, []),
        (reserved3, 10, 2, []),
        (sresetn_uart6, 12, 1, []),
        (reserved4, 13, 2, []),
        (sresetn_uart7, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON14  0x0A38;
generate_register_bitfields!(
    CRU_SOFTRST_CON14,
    u32,
    [
        (reserved0, 0, 2, []),
        (sresetn_uart8, 2, 1, []),
        (reserved1, 3, 2, []),
        (sresetn_uart9, 5, 1, []),
        (presetn_spi0, 6, 1, []),
        (presetn_spi1, 7, 1, []),
        (presetn_spi2, 8, 1, []),
        (presetn_spi3, 9, 1, []),
        (presetn_spi4, 10, 1, []),
        (resetn_spi0, 11, 1, []),
        (resetn_spi1, 12, 1, []),
        (resetn_spi2, 13, 1, []),
        (resetn_spi3, 14, 1, []),
        (resetn_spi4, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON15  0x0A3C;
generate_register_bitfields!(
    CRU_SOFTRST_CON15,
    u32,
    [
        (presetn_wdt0, 0, 1, []),
        (tresetn_wdt0, 1, 1, []),
        (presetn_sys_grf, 2, 1, []),
        (presetn_pwm1, 3, 1, []),
        (resetn_pwm1, 4, 1, []),
        (reserved0, 5, 1, []),
        (presetn_pwm2, 6, 1, []),
        (resetn_pwm2, 7, 1, []),
        (reserved1, 8, 1, []),
        (presetn_pwm3, 9, 1, []),
        (resetn_pwm3, 10, 1, []),
        (reserved2, 11, 1, []),
        (presetn_bustimer0, 12, 1, []),
        (presetn_bustimer1, 13, 1, []),
        (reserved3, 14, 1, []),
        (resetn_bustimer0, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON16  0x0A40;
generate_register_bitfields!(
    CRU_SOFTRST_CON16,
    u32,
    [
        (resetn_bustimer1, 0, 1, []),
        (resetn_bustimer2, 1, 1, []),
        (resetn_bustimer3, 2, 1, []),
        (resetn_bustimer4, 3, 1, []),
        (resetn_bustimer5, 4, 1, []),
        (resetn_bustimer6, 5, 1, []),
        (resetn_bustimer7, 6, 1, []),
        (resetn_bustimer8, 7, 1, []),
        (resetn_bustimer9, 8, 1, []),
        (resetn_bustimer10, 9, 1, []),
        (resetn_bustimer11, 10, 1, []),
        (presetn_mailbox0, 11, 1, []),
        (presetn_mailbox1, 12, 1, []),
        (presetn_mailbox2, 13, 1, []),
        (presetn_gpio1, 14, 1, []),
        (dbresetn_gpio1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON17  0x0A44;
generate_register_bitfields!(
    CRU_SOFTRST_CON17,
    u32,
    [
        (presetn_gpio2, 0, 1, []),
        (dbresetn_gpio2, 1, 1, []),
        (presetn_gpio3, 2, 1, []),
        (dbresetn_gpio3, 3, 1, []),
        (presetn_gpio4, 4, 1, []),
        (dbresetn_gpio4, 5, 1, []),
        (aresetn_decom, 6, 1, []),
        (presetn_decom, 7, 1, []),
        (dresetn_decom, 8, 1, []),
        (presetn_top, 9, 1, []),
        (reserved, 10, 1, []),
        (aresetn_gicadb_gic2core_bus, 11, 1, []),
        (presetn_dft2apb, 12, 1, []),
        (presetn_apb2asb_mst_top, 13, 1, []),
        (presetn_apb2asb_mst_cdphy, 14, 1, []),
        (presetn_apb2asb_mst_bot_right, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON18  0x0A48;
generate_register_bitfields!(
    CRU_SOFTRST_CON18,
    u32,
    [
        (presetn_apb2asb_mst_ioc_top, 0, 1, []),
        (presetn_apb2asb_mst_ioc_right, 1, 1, []),
        (presetn_apb2asb_mst_csiphy, 2, 1, []),
        (presetn_apb2asb_mst_vccio3_5, 3, 1, []),
        (presetn_apb2asb_mst_vccio6, 4, 1, []),
        (presetn_apb2asb_mst_emmcio, 5, 1, []),
        (aresetn_spinlock, 6, 1, []),
        (reserved0, 7, 2, []),
        (presetn_otpc_ns, 9, 1, []),
        (resetn_otpc_ns, 10, 1, []),
        (resetn_otpc_arb, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON19  0x0A4C;
generate_register_bitfields!(
    CRU_SOFTRST_CON19,
    u32,
    [
        (presetn_busioc, 0, 1, []),
        (reserved0, 1, 3, []),
        (presetn_pmucm0_intmux, 4, 1, []),
        (presetn_ddrcm0_intmux, 5, 1, []),
        (reserved1, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON20  0x0A50;
generate_register_bitfields!(
    CRU_SOFTRST_CON20,
    u32,
    [
        (presetn_ddr_dfictl_ch0, 0, 1, []),
        (presetn_ddr_mon_ch0, 1, 1, []),
        (presetn_ddr_standby_ch0, 2, 1, []),
        (presetn_ddr_upctl_ch0, 3, 1, []),
        (tmresetn_ddr_mon_ch0, 4, 1, []),
        (presetn_ddr_grf_ch01, 5, 1, []),
        (resetn_dfi_ch0, 6, 1, []),
        (resetn_sbr_ch0, 7, 1, []),
        (resetn_ddr_upctl_ch0, 8, 1, []),
        (resetn_ddr_dfictl_ch0, 9, 1, []),
        (resetn_ddr_mon_ch0, 10, 1, []),
        (resetn_ddr_standby_ch0, 11, 1, []),
        (aresetn_ddr_upctl_ch0, 12, 1, []),
        (presetn_ddr_dfictl_ch1, 13, 1, []),
        (presetn_ddr_mon_ch1, 14, 1, []),
        (presetn_ddr_standby_ch1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON21  0x0A54;
generate_register_bitfields!(
    CRU_SOFTRST_CON21,
    u32,
    [
        (presetn_ddr_upctl_ch1, 0, 1, []),
        (tmresetn_ddr_mon_ch1, 1, 1, []),
        (resetn_dfi_ch1, 2, 1, []),
        (resetn_sbr_ch1, 3, 1, []),
        (resetn_ddr_upctl_ch1, 4, 1, []),
        (resetn_ddr_dfictl_ch1, 5, 1, []),
        (resetn_ddr_mon_ch1, 6, 1, []),
        (resetn_ddr_standby_ch1, 7, 1, []),
        (aresetn_ddr_upctl_ch1, 8, 1, []),
        (reserved, 9, 4, []),
        (aresetn_ddr_ddrsch0, 13, 1, []),
        (aresetn_ddr_rs_ddrsch0, 14, 1, []),
        (aresetn_ddr_frs_ddrsch0, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON22  0x0A58;
generate_register_bitfields!(
    CRU_SOFTRST_CON22,
    u32,
    [
        (aresetn_ddr_scramble0, 0, 1, []),
        (aresetn_ddr_frs_scramble0, 1, 1, []),
        (aresetn_ddr_ddrsch1, 2, 1, []),
        (aresetn_ddr_rs_ddrsch1, 3, 1, []),
        (aresetn_ddr_frs_ddrsch1, 4, 1, []),
        (aresetn_ddr_scramble1, 5, 1, []),
        (aresetn_ddr_frs_scramble1, 6, 1, []),
        (presetn_ddr_ddrsch0, 7, 1, []),
        (presetn_ddr_ddrsch1, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON23  0x0A5C;
generate_register_bitfields!(
    CRU_SOFTRST_CON23,
    u32,
    [
        (presetn_ddr_dfictl_ch2, 0, 1, []),
        (presetn_ddr_mon_ch2, 1, 1, []),
        (presetn_ddr_standby_ch2, 2, 1, []),
        (presetn_ddr_upctl_ch2, 3, 1, []),
        (tmresetn_ddr_mon_ch2, 4, 1, []),
        (presetn_ddr_grf_ch23, 5, 1, []),
        (resetn_dfi_ch2, 6, 1, []),
        (resetn_sbr_ch2, 7, 1, []),
        (resetn_ddr_upctl_ch2, 8, 1, []),
        (resetn_ddr_dfictl_ch2, 9, 1, []),
        (resetn_ddr_mon_ch2, 10, 1, []),
        (resetn_ddr_standby_ch2, 11, 1, []),
        (aresetn_ddr_upctl_ch2, 12, 1, []),
        (presetn_ddr_dfictl_ch3, 13, 1, []),
        (presetn_ddr_mon_ch3, 14, 1, []),
        (presetn_ddr_standby_ch3, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON24  0x0A60;
generate_register_bitfields!(
    CRU_SOFTRST_CON24,
    u32,
    [
        (presetn_ddr_upctl_ch3, 0, 1, []),
        (tmresetn_ddr_mon_ch3, 1, 1, []),
        (resetn_dfi_ch3, 2, 1, []),
        (resetn_sbr_ch3, 3, 1, []),
        (resetn_ddr_upctl_ch3, 4, 1, []),
        (resetn_ddr_dfictl_ch3, 5, 1, []),
        (resetn_ddr_mon_ch3, 6, 1, []),
        (resetn_ddr_standby_ch3, 7, 1, []),
        (aresetn_ddr_upctl_ch3, 8, 1, []),
        (reserved, 9, 4, []),
        (aresetn_ddr_ddrsch2, 13, 1, []),
        (aresetn_ddr_rs_ddrsch2, 14, 1, []),
        (aresetn_ddr_frs_ddrsch2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON25  0x0A64;
generate_register_bitfields!(
    CRU_SOFTRST_CON25,
    u32,
    [
        (aresetn_ddr_scramble2, 0, 1, []),
        (aresetn_ddr_frs_scramble2, 1, 1, []),
        (aresetn_ddr_ddrsch3, 2, 1, []),
        (reseraresetn_ddr_rs_ddrsch3ved, 3, 1, []),
        (aresetn_ddr_frs_ddrsch3, 4, 1, []),
        (aresetn_ddr_scramble3, 5, 1, []),
        (aresetn_ddr_frs_scramble3, 6, 1, []),
        (presetn_ddr_ddrsch2, 7, 1, []),
        (presetn_ddr_ddrsch3, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON26  0x0A68;
generate_register_bitfields!(
    CRU_SOFTRST_CON26,
    u32,
    [
        (reserved0, 0, 3, []),
        (resetn_isp1, 3, 1, []),
        (resetn_isp1_vicap, 4, 1, []),
        (reserved1, 5, 1, []),
        (aresetn_isp1_biu, 6, 1, []),
        (reserved2, 7, 1, []),
        (hresetn_isp1_biu, 8, 1, []),
        (reserved3, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON27  0x0A6C;
generate_register_bitfields!(
    CRU_SOFTRST_CON27,
    u32,
    [
        (aresetn_rknn1, 0, 1, []),
        (aresetn_rknn1_biu, 1, 1, []),
        (hresetn_rknn1, 2, 1, []),
        (hresetn_rknn1_biu, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON28  0x0A70;
generate_register_bitfields!(
    CRU_SOFTRST_CON28,
    u32,
    [
        (aresetn_rknn2, 0, 1, []),
        (aresetn_rknn2_biu, 1, 1, []),
        (hresetn_rknn2, 2, 1, []),
        (hresetn_rknn2_biu, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON29  0x0A74;
generate_register_bitfields!(
    CRU_SOFTRST_CON29,
    u32,
    [
        (reserved0, 0, 3, []),
        (aresetn_rknn_dsu0, 3, 1, []),
        (reserved1, 4, 1, []),
        (presetn_nputop_biu, 5, 1, []),
        (presetn_npu_timer, 6, 1, []),
        (reserved2, 7, 1, []),
        (resetn_nputimer0, 8, 1, []),
        (resetn_nputimer1, 9, 1, []),
        (presetn_npu_wdt, 10, 1, []),
        (tresetn_npu_wdt, 11, 1, []),
        (presetn_pvtm1, 12, 1, []),
        (presetn_npu_grf, 13, 1, []),
        (resetn_pvtm1, 14, 1, []),
        (reserved3, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON30  0x0A78;
generate_register_bitfields!(
    CRU_SOFTRST_CON30,
    u32,
    [
        (resetn_npu_pvtpll, 0, 1, []),
        (reserved0, 1, 1, []),
        (hresetn_npu_cm0_biu, 2, 1, []),
        (fresetn_npu_cm0_core, 3, 1, []),
        (tresetn_npu_cm0_jtag, 4, 1, []),
        (reserved1, 5, 1, []),
        (aresetn_rknn0, 6, 1, []),
        (aresetn_rknn0_biu, 7, 1, []),
        (hresetn_rknn0, 8, 1, []),
        (hresetn_rknn0_biu, 9, 1, []),
        (reserved2, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON31  0x0A7C;
generate_register_bitfields!(
    CRU_SOFTRST_CON31,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_nvm_biu, 2, 1, []),
        (aresetn_nvm_biu, 3, 1, []),
        (hresetn_emmc, 4, 1, []),
        (aresetn_emmc, 5, 1, []),
        (cresetn_emmc, 6, 1, []),
        (bresetn_emmc, 7, 1, []),
        (tresetn_emmc, 8, 1, []),
        (sresetn_sfc, 9, 1, []),
        (hresetn_sfc, 10, 1, []),
        (hresetn_sfc_xip, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON32  0x0A80;
generate_register_bitfields!(
    CRU_SOFTRST_CON32,
    u32,
    [
        (reserved0, 0, 1, []),
        (presetn_grf, 1, 1, []),
        (presetn_dec_biu, 2, 1, []),
        (reserved1, 3, 2, []),
        (presetn_php_biu, 5, 1, []),
        (reserved2, 6, 2, []),
        (aresetn_pcie_bridge, 8, 1, []),
        (aresetn_php_biu, 9, 1, []),
        (aresetn_gmac0, 10, 1, []),
        (aresetn_gmac1, 11, 1, []),
        (aresetn_pcie_biu, 12, 1, []),
        (resetn_pcie_4l_power_up, 13, 1, []),
        (resetn_pcie_2l_power_up, 14, 1, []),
        (resetn_pcie_1l0_power_up, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON33  0x0A84;
generate_register_bitfields!(
    CRU_SOFTRST_CON33,
    u32,
    [
        (resetn_pcie_1l1_power_up, 0, 1, []),
        (resetn_pcie_1l2_power_up, 1, 1, []),
        (reserved, 2, 10, []),
        (presetn_pcie_4l, 12, 1, []),
        (presetn_pcie_2l, 13, 1, []),
        (presetn_pcie_1l0, 14, 1, []),
        (presetn_pcie_1l1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON34  0x0A88;
generate_register_bitfields!(
    CRU_SOFTRST_CON34,
    u32,
    [
        (presetn_pcie_1l2, 0, 1, []),
        (reserved0, 1, 5, []),
        (aresetn_php_gic_its, 6, 1, []),
        (aresetn_mmu_pcie, 7, 1, []),
        (aresetn_mmu_php, 8, 1, []),
        (aresetn_mmu_biu, 9, 1, []),
        (reserved1, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON35  0x0A8C;
generate_register_bitfields!(
    CRU_SOFTRST_CON35,
    u32,
    [
        (reserved0, 0, 7, []),
        (aresetn_usb3otg2, 7, 1, []),
        (reserved1, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON37  0x0A94;
generate_register_bitfields!(
    CRU_SOFTRST_CON37,
    u32,
    [
        (reserved, 0, 4, []),
        (resetn_pmalive0, 4, 1, []),
        (resetn_pmalive1, 5, 1, []),
        (resetn_pmalive2, 6, 1, []),
        (aresetn_sata0, 7, 1, []),
        (aresetn_sata1, 8, 1, []),
        (aresetn_sata2, 9, 1, []),
        (resetn_rxoob0, 10, 1, []),
        (resetn_rxoob1, 11, 1, []),
        (resetn_rxoob2, 12, 1, []),
        (resetn_asic0, 13, 1, []),
        (resetn_asic1, 14, 1, []),
        (resetn_asic2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON40  0x0AA0;
generate_register_bitfields!(
    CRU_SOFTRST_CON40,
    u32,
    [
        (reserved0, 0, 2, []),
        (aresetn_rkvdec_ccu, 2, 1, []),
        (hresetn_rkvdec0, 3, 1, []),
        (aresetn_rkvdec0, 4, 1, []),
        (hresetn_rkvdec0_biu, 5, 1, []),
        (aresetn_rkvdec0_biu, 6, 1, []),
        (resetn_rkvdec0_ca, 7, 1, []),
        (resetn_rkvdec0_hevc_c, 8, 1, []),
        (resetn_rkvdec0_core, 9, 1, []),
        (reserved1, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON41  0x0AA4;
generate_register_bitfields!(
    CRU_SOFTRST_CON41,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_rkvdec1, 2, 1, []),
        (rearesetn_rkvdec1served, 3, 1, []),
        (hresetn_rkvdec1_biu, 4, 1, []),
        (aresetn_rkvdec1_biu, 5, 1, []),
        (resetn_rkvdec1_ca, 6, 1, []),
        (resetn_rkvdec1_hevc_ca, 7, 1, []),
        (resetn_rkvdec1_core, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON42  0x0AA8;
generate_register_bitfields!(
    CRU_SOFTRST_CON42,
    u32,
    [
        (reserved0, 0, 2, []),
        (aresetn_usb_biu, 2, 1, []),
        (hresetn_usb_biu, 3, 1, []),
        (aresetn_usb3otg0, 4, 1, []),
        (reserved1, 5, 2, []),
        (aresetn_usb3otg1, 7, 1, []),
        (reserved2, 8, 2, []),
        (hresetn_host0, 10, 1, []),
        (hresetn_host_arb0, 11, 1, []),
        (hresetn_host1, 12, 1, []),
        (hresetn_host_arb1, 13, 1, []),
        (aresetn_usb_grf, 14, 1, []),
        (cresetn_usb2p0_host0, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON43  0x0AAC;
generate_register_bitfields!(
    CRU_SOFTRST_CON43,
    u32,
    [
        (cresetn_usb2p0_host1, 0, 1, []),
        (resetn_host_utmi0, 1, 1, []),
        (resetn_host_utmi1, 2, 1, []),
        (reserved, 3, 13, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON44  0x0AB0;
generate_register_bitfields!(
    CRU_SOFTRST_CON44,
    u32,
    [
        (reserved, 0, 4, []),
        (aresetn_vdpu_biu, 4, 1, []),
        (aresetn_vdpu_low_biu, 5, 1, []),
        (reserhresetn_vdpu_biuved, 6, 1, []),
        (aresetn_jpeg_decoder_biu, 7, 1, []),
        (aresetn_vpu, 8, 1, []),
        (hresetn_vpu, 9, 1, []),
        (aresetn_jpeg_encoder0, 10, 1, []),
        (hresetn_jpeg_encoder0, 11, 1, []),
        (aresetn_jpeg_encoder1, 12, 1, []),
        (hresetn_jpeg_encoder1, 13, 1, []),
        (aresetn_jpeg_encoder2, 14, 1, []),
        (hresetn_jpeg_encoder2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON45  0x0AB4;
generate_register_bitfields!(
    CRU_SOFTRST_CON45,
    u32,
    [
        (aresetn_jpeg_encoder3, 0, 1, []),
        (hresetn_jpeg_encoder3, 1, 1, []),
        (aresetn_jpeg_decoder, 2, 1, []),
        (hresetn_jpeg_decoder, 3, 1, []),
        (hresetn_iep2p0, 4, 1, []),
        (aresetn_iep2p0, 5, 1, []),
        (resetn_iep2p0_core, 6, 1, []),
        (hresetn_rga2, 7, 1, []),
        (aresetn_rga2, 8, 1, []),
        (resetn_rga2_core, 9, 1, []),
        (hresetn_rga3_0, 10, 1, []),
        (aresetn_rga3_0, 11, 1, []),
        (resetn_rga3_0_core, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON47  0x0ABC;
generate_register_bitfields!(
    CRU_SOFTRST_CON47,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_rkvenc0_biu, 2, 1, []),
        (aresetn_rkvenc0_biu, 3, 1, []),
        (hresetn_rkvenc0, 4, 1, []),
        (aresetn_rkvenc0, 5, 1, []),
        (resetn_rkvenc0_core, 6, 1, []),
        (reserved1, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON48  0x0AC0;
generate_register_bitfields!(
    CRU_SOFTRST_CON48,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_rkvenc1_biu, 2, 1, []),
        (aresetn_rkvenc1_biu, 3, 1, []),
        (hresetn_rkvenc1, 4, 1, []),
        (aresetn_rkvenc1, 5, 1, []),
        (resetn_rkvenc1_core, 6, 1, []),
        (reserved1, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON49  0x0AC4;
generate_register_bitfields!(
    CRU_SOFTRST_CON49,
    u32,
    [
        (reserved0, 0, 3, []),
        (aresetn_vi_biu, 3, 1, []),
        (hresetn_vi_biu, 4, 1, []),
        (presetn_vi_biu, 5, 1, []),
        (dresetn_vicap, 6, 1, []),
        (aresetn_vicap, 7, 1, []),
        (hresetn_vicap, 8, 1, []),
        (reserved1, 9, 1, []),
        (resetn_isp0, 10, 1, []),
        (resetn_isp0_vicap, 11, 1, []),
        (reserved2, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON50  0x0AC8;
generate_register_bitfields!(
    CRU_SOFTRST_CON50,
    u32,
    [
        (resetn_fisheye0, 0, 1, []),
        (reserved0, 1, 2, []),
        (resetn_fisheye1, 3, 1, []),
        (presetn_csi_host_0, 4, 1, []),
        (presetn_csi_host_1, 5, 1, []),
        (presetn_csi_host_2, 6, 1, []),
        (presetn_csi_host_3, 7, 1, []),
        (presetn_csi_host_4, 8, 1, []),
        (presetn_csi_host_5, 9, 1, []),
        (reserved1, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON51  0x0ACC;
generate_register_bitfields!(
    CRU_SOFTRST_CON51,
    u32,
    [
        (reserved0, 0, 4, []),
        (resetn_csihost0_vicap, 4, 1, []),
        (resetn_csihost1_vicap, 5, 1, []),
        (resetn_csihost2_vicap, 6, 1, []),
        (resetn_csihost3_vicap, 7, 1, []),
        (resetn_csihost4_vicap, 8, 1, []),
        (resetn_csihost5_vicap, 9, 1, []),
        (reserved1, 10, 3, []),
        (resetn_cifin, 13, 1, []),
        (reserved2, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON52  0x0AD0;
generate_register_bitfields!(
    CRU_SOFTRST_CON52,
    u32,
    [
        (reserved0, 0, 4, []),
        (aresetn_vop_biu, 4, 1, []),
        (aresetn_vop_low_biu, 5, 1, []),
        (hresetn_vop_biu, 6, 1, []),
        (presetn_vop_biu, 7, 1, []),
        (hresetn_vop, 8, 1, []),
        (aresetn_vop, 9, 1, []),
        (reserved1, 10, 3, []),
        (dresetn_vp0, 13, 1, []),
        (dresetn_vp2hdmi_bridge0, 14, 1, []),
        (dresetn_vp2hdmi_bridge1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON53  0x0AD4;
generate_register_bitfields!(
    CRU_SOFTRST_CON53,
    u32,
    [
        (dresetn_vp1, 0, 1, []),
        (dresetn_vp2, 1, 1, []),
        (dresetn_vp3, 2, 1, []),
        (presetn_vopgrf, 3, 1, []),
        (presetn_dsihost0, 4, 1, []),
        (presetn_dsihost1, 5, 1, []),
        (resetn_dsihost0, 6, 1, []),
        (resetn_dsihost1, 7, 1, []),
        (resetn_vop_pmu, 8, 1, []),
        (presetn_vop_channel_biu, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON55  0x0ADC;
generate_register_bitfields!(
    CRU_SOFTRST_CON55,
    u32,
    [
        (reserved0, 0, 1, []),
        (hresetn_vo0_biu, 5, 1, []),
        (hresetn_vo0_s_biu, 6, 1, []),
        (presetn_vo0_biu, 7, 1, []),
        (presetn_vo0_s_biu, 8, 1, []),
        (aresetn_hdcp0_biu, 9, 1, []),
        (presetn_vo0grf, 10, 1, []),
        (hresetn_hdcp_key0, 11, 1, []),
        (aresetn_hdcp0, 12, 1, []),
        (hresetn_hdcp0, 13, 1, []),
        (reserved1, 14, 1, []),
        (resetn_hdcp0, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON56  0x0AE0;
generate_register_bitfields!(
    CRU_SOFTRST_CON56,
    u32,
    [
        (reserved0, 0, 1, []),
        (presetn_trng0, 1, 1, []),
        (reserved1, 2, 6, []),
        (resetn_dp0, 8, 1, []),
        (resetn_dp1, 9, 1, []),
        (hresetn_i2s4_8ch, 10, 1, []),
        (reserved2, 11, 2, []),
        (mresetn_i2s4_8ch_tx, 13, 1, []),
        (hresetn_i2s8_8ch, 14, 1, []),
        (reserved3, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON57  0x0AE4;
generate_register_bitfields!(
    CRU_SOFTRST_CON57,
    u32,
    [
        (reserved0, 0, 1, []),
        (mresetn_i2s8_8ch_tx, 1, 1, []),
        (hresetn_spdif2_dp0, 2, 1, []),
        (reserved1, 3, 3, []),
        (mresetn_spdif2_dp0, 6, 1, []),
        (hresetn_spdif5_dp1, 7, 1, []),
        (reserved2, 8, 3, []),
        (mresetn_spdif5_dp1, 11, 1, []),
        (reserved3, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON59  0x0AEC;
generate_register_bitfields!(
    CRU_SOFTRST_CON59,
    u32,
    [
        (reserved0, 0, 6, []),
        (aresetn_hdcp1_biu, 6, 1, []),
        (reserved1, 7, 1, []),
        (aresetn_vo1_biu, 8, 1, []),
        (hresetn_vo1_biu0, 9, 1, []),
        (hresetn_vo1_s_biu, 10, 1, []),
        (hresetn_vo1_biu1, 11, 1, []),
        (presetn_vo1grf, 12, 1, []),
        (presetn_vo1_s_biu, 13, 1, []),
        (reserved2, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON60  0x0AF0;
generate_register_bitfields!(
    CRU_SOFTRST_CON60,
    u32,
    [
        (hresetn_i2s7_8ch, 0, 1, []),
        (reserved0, 1, 2, []),
        (mresetn_i2s7_8ch_rx, 3, 1, []),
        (hresetn_hdcp_key1, 4, 1, []),
        (aresetn_hdcp1, 5, 1, []),
        (hresetn_hdcp1, 6, 1, []),
        (reserved1, 7, 1, []),
        (resetn_hdcp1, 8, 1, []),
        (reserved2, 9, 1, []),
        (presetn_trng1, 10, 1, []),
        (presetn_hdmitx0, 11, 1, []),
        (reserved3, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON61  0x0AF4;
generate_register_bitfields!(
    CRU_SOFTRST_CON61,
    u32,
    [
        (resetn_hdmitx0_ref, 0, 1, []),
        (reserved0, 1, 1, []),
        (presetn_hdmitx1, 2, 1, []),
        (reserved1, 3, 4, []),
        (resetn_hdmitx1_ref, 7, 1, []),
        (reserved2, 8, 1, []),
        (aresetn_hdmirx, 9, 1, []),
        (presetn_hdmirx, 10, 1, []),
        (resetn_hdmirx_ref, 11, 1, []),
        (reserved3, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON62  0x0AF8;
generate_register_bitfields!(
    CRU_SOFTRST_CON62,
    u32,
    [
        (presetn_edp0, 0, 1, []),
        (resetn_edp0_24m, 1, 1, []),
        (reserved0, 2, 1, []),
        (presetn_edp1, 3, 1, []),
        (resetn_edp1_24m, 4, 1, []),
        (reserved1, 5, 3, []),
        (mresetn_i2s5_8ch_tx, 8, 1, []),
        (reserved2, 9, 3, []),
        (hresetn_i2s5_8ch, 12, 1, []),
        (reserved3, 13, 2, []),
        (mresetn_i2s6_8ch_tx, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON63  0x0AFC;
generate_register_bitfields!(
    CRU_SOFTRST_CON63,
    u32,
    [
        (reserved0, 0, 1, []),
        (mresetn_i2s6_8ch_rx, 2, 1, []),
        (hresetn_i2s6_8ch, 3, 1, []),
        (hresetn_spdif3, 4, 1, []),
        (reserved1, 5, 2, []),
        (mresetn_spdif3, 7, 1, []),
        (hresetn_spdif4, 8, 1, []),
        (reserved2, 9, 2, []),
        (mresetn_spdif4, 11, 1, []),
        (hresetn_spdifrx0, 12, 1, []),
        (mresetn_spdifrx0, 13, 1, []),
        (hresetn_spdifrx1, 14, 1, []),
        (mresetn_spdifrx1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON64  0x0B00;
generate_register_bitfields!(
    CRU_SOFTRST_CON64,
    u32,
    [
        (hresetn_spdifrx2, 0, 1, []),
        (mresetn_spdifrx2, 1, 1, []),
        (reserved, 2, 10, []),
        (resetn_linksym_hdmitxphy0, 12, 1, []),
        (resetn_linksym_hdmitxphy1, 13, 1, []),
        (resetn_vo1_bridge0, 14, 1, []),
        (resetn_vo1_bridge1, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON65  0x0B04;
generate_register_bitfields!(
    CRU_SOFTRST_CON65,
    u32,
    [
        (hresetn_i2s9_8ch, 0, 1, []),
        (reserved0, 1, 2, []),
        (mresetn_i2s9_8ch_rx, 3, 1, []),
        (hresetn_i2s10_8ch, 4, 1, []),
        (reserved1, 5, 2, []),
        (mresetn_i2s10_8ch_rx, 7, 1, []),
        (presetn_s_hdmirx, 8, 1, []),
        (reserved2, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON66  0x0B08;
generate_register_bitfields!(
    CRU_SOFTRST_CON66,
    u32,
    [
        (reserved0, 0, 4, []),
        (resetn_gpu, 4, 1, []),
        (sysresetn_gpu, 5, 1, []),
        (reserved1, 6, 2, []),
        (aresetn_s_gpu_biu, 8, 1, []),
        (aresetn_m0_gpu_biu, 9, 1, []),
        (aresetn_m1_gpu_biu, 10, 1, []),
        (aresetn_m2_gpu_biu, 11, 1, []),
        (aresetn_m3_gpu_biu, 12, 1, []),
        (reserved2, 13, 1, []),
        (presetn_gpu_biu, 14, 1, []),
        (presetn_pvtm2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON67  0x0B0C;
generate_register_bitfields!(
    CRU_SOFTRST_CON67,
    u32,
    [
        (resetn_pvtm2, 0, 1, []),
        (reserved0, 1, 1, []),
        (presetn_gpu_grf, 2, 1, []),
        (resetn_gpu_pvtpll, 3, 1, []),
        (poresetn_gpu_jtag, 4, 1, []),
        (reserved1, 5, 11, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON68  0x0B10;
generate_register_bitfields!(
    CRU_SOFTRST_CON68,
    u32,
    [
        (reserved0, 0, 1, []),
        (aresetn_av1_biu, 1, 1, []),
        (aresetn_av1, 2, 1, []),
        (reserved1, 3, 1, []),
        (presetn_av1_biu, 4, 1, []),
        (presetn_av1, 5, 1, []),
        (reserved2, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON69  0x0B14;
generate_register_bitfields!(
    CRU_SOFTRST_CON69,
    u32,
    [
        (reserved0, 0, 4, []),
        (aresetn_ddr_biu, 4, 1, []),
        (aresetn_dma2ddr, 5, 1, []),
        (aresetn_ddr_sharemem, 6, 1, []),
        (aresetn_ddr_sharemem_biu, 7, 1, []),
        (reserved1, 8, 2, []),
        (aresetn_center_s200_biu, 10, 1, []),
        (aresetn_center_s400_biu, 11, 1, []),
        (hresetn_ahb2apb, 12, 1, []),
        (hresetn_center_biu, 13, 1, []),
        (fresetn_ddr_cm0_core, 14, 1, []),
        (reserved2, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON70  0x0B18;
generate_register_bitfields!(
    CRU_SOFTRST_CON70,
    u32,
    [
        (resetn_ddr_timer0, 0, 1, []),
        (resetn_ddr_timer1, 1, 1, []),
        (tresetn_wdt_ddr, 2, 1, []),
        (tresetn_ddr_cm0_jtag, 3, 1, []),
        (reserved0, 4, 1, []),
        (presetn_center_grf, 5, 1, []),
        (presetn_ahb2apb, 6, 1, []),
        (presetn_wdt, 7, 1, []),
        (presetn_timer, 8, 1, []),
        (presetn_dma2ddr, 9, 1, []),
        (presetn_sharemem, 10, 1, []),
        (presetn_center_biu, 11, 1, []),
        (presetn_center_channel_biu, 12, 1, []),
        (reserved1, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON72  0x0B20;
generate_register_bitfields!(
    CRU_SOFTRST_CON72,
    u32,
    [
        (reserved0, 0, 1, []),
        (presetn_usbdpgrf0, 1, 1, []),
        (presetn_usbdpphy0, 2, 1, []),
        (presetn_usbdpgrf1, 3, 1, []),
        (presetn_usbdpphy1, 4, 1, []),
        (presetn_hdptx0, 5, 1, []),
        (presetn_hdptx1, 6, 1, []),
        (presetn_apb2asb_slv_bot_right, 7, 1, []),
        (presetn_usb2phy_u3_0_grf0, 8, 1, []),
        (presetn_usb2phy_u3_1_grf0, 9, 1, []),
        (presetn_usb2phy_u2_0_grf0, 10, 1, []),
        (presetn_usb2phy_u2_1_grf0, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON73  0x0B24;
generate_register_bitfields!(
    CRU_SOFTRST_CON73,
    u32,
    [
        (reserved0, 0, 12, []),
        (resetn_hdmihdp0, 12, 1, []),
        (resetn_hdmihdp1, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON74  0x0B28;
generate_register_bitfields!(
    CRU_SOFTRST_CON74,
    u32,
    [
        (reserved0, 0, 1, []),
        (aresetn_vo1usb_top_biu, 1, 1, []),
        (reserved1, 2, 1, []),
        (hresetn_vo1usb_top_biu, 3, 1, []),
        (reserved2, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON75  0x0B2C;
generate_register_bitfields!(
    CRU_SOFTRST_CON75,
    u32,
    [
        (reserved0, 0, 1, []),
        (hresetn_sdio_biu, 1, 1, []),
        (hresetn_sdio, 2, 1, []),
        (resetn_sdio, 3, 1, []),
        (reserved1, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON76  0x0B30;
generate_register_bitfields!(
    CRU_SOFTRST_CON76,
    u32,
    [
        (reserved0, 0, 2, []),
        (hresetn_rga3_biu, 2, 1, []),
        (aresetn_rga3_biu, 3, 1, []),
        (hresetn_rga3_1, 4, 1, []),
        (aresetn_rga3_1, 5, 1, []),
        (resetn_rga3_1_core, 6, 1, []),
        (reserved1, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SOFTRST_CON77  0x0B34;
generate_register_bitfields!(
    CRU_SOFTRST_CON77,
    u32,
    [
        (reserved0, 0, 6, []),
        (resetn_ref_pipe_phy0, 6, 1, []),
        (resetn_ref_pipe_phy1, 7, 1, []),
        (resetn_ref_pipe_phy2, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);
