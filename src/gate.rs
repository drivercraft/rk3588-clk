use crate::generate_register_bitfields;
use tock_registers::{register_bitfields, registers::ReadWrite};

pub struct GateRegisters {
    pub gate_con0: ReadWrite<u32, CRU_GATE_CON00::Register>,
    pub gate_con1: ReadWrite<u32, CRU_GATE_CON01::Register>,
    pub gate_con2: ReadWrite<u32, CRU_GATE_CON02::Register>,
    pub gate_con3: ReadWrite<u32, CRU_GATE_CON03::Register>,
    pub gate_con4: ReadWrite<u32, CRU_GATE_CON04::Register>,
    pub gate_con5: ReadWrite<u32, CRU_GATE_CON05::Register>,
    pub gate_con6: ReadWrite<u32, CRU_GATE_CON06::Register>,
    pub gate_con7: ReadWrite<u32, CRU_GATE_CON07::Register>,
    pub gate_con8: ReadWrite<u32, CRU_GATE_CON08::Register>,
    pub gate_con9: ReadWrite<u32, CRU_GATE_CON09::Register>,
    pub gate_con10: ReadWrite<u32, CRU_GATE_CON10::Register>,
    pub gate_con11: ReadWrite<u32, CRU_GATE_CON11::Register>,
    pub gate_con12: ReadWrite<u32, CRU_GATE_CON12::Register>,
    pub gate_con13: ReadWrite<u32, CRU_GATE_CON13::Register>,
    pub gate_con14: ReadWrite<u32, CRU_GATE_CON14::Register>,
    pub gate_con15: ReadWrite<u32, CRU_GATE_CON15::Register>,
    pub gate_con16: ReadWrite<u32, CRU_GATE_CON16::Register>,
    pub gate_con17: ReadWrite<u32, CRU_GATE_CON17::Register>,
    pub gate_con18: ReadWrite<u32, CRU_GATE_CON18::Register>,
    pub gate_con19: ReadWrite<u32, CRU_GATE_CON19::Register>,
    pub gate_con20: ReadWrite<u32, CRU_GATE_CON20::Register>,
    pub gate_con21: ReadWrite<u32, CRU_GATE_CON21::Register>,
    pub gate_con22: ReadWrite<u32, CRU_GATE_CON22::Register>,
    pub gate_con23: ReadWrite<u32, CRU_GATE_CON23::Register>,
    pub gate_con24: ReadWrite<u32, CRU_GATE_CON24::Register>,
    pub gate_con25: ReadWrite<u32, CRU_GATE_CON25::Register>,
    pub gate_con26: ReadWrite<u32, CRU_GATE_CON26::Register>,
    pub gate_con27: ReadWrite<u32, CRU_GATE_CON27::Register>,
    pub gate_con28: ReadWrite<u32, CRU_GATE_CON28::Register>,
    pub gate_con29: ReadWrite<u32, CRU_GATE_CON29::Register>,
    pub gate_con30: ReadWrite<u32, CRU_GATE_CON30::Register>,
    pub gate_con31: ReadWrite<u32, CRU_GATE_CON31::Register>,
    pub gate_con32: ReadWrite<u32, CRU_GATE_CON32::Register>,
    pub gate_con33: ReadWrite<u32, CRU_GATE_CON33::Register>,
    pub gate_con34: ReadWrite<u32, CRU_GATE_CON34::Register>,
    pub gate_con35: ReadWrite<u32, CRU_GATE_CON35::Register>,
    _reserved0: u32,
    pub gate_con37: ReadWrite<u32, CRU_GATE_CON37::Register>,
    pub gate_con38: ReadWrite<u32, CRU_GATE_CON38::Register>,
    pub gate_con39: ReadWrite<u32, CRU_GATE_CON39::Register>,
    pub gate_con40: ReadWrite<u32, CRU_GATE_CON40::Register>,
    pub gate_con41: ReadWrite<u32, CRU_GATE_CON41::Register>,
    pub gate_con42: ReadWrite<u32, CRU_GATE_CON42::Register>,
    pub gate_con43: ReadWrite<u32, CRU_GATE_CON43::Register>,
    pub gate_con44: ReadWrite<u32, CRU_GATE_CON44::Register>,
    pub gate_con45: ReadWrite<u32, CRU_GATE_CON45::Register>,
    _reserved1: u32,
    pub gate_con47: ReadWrite<u32, CRU_GATE_CON47::Register>,
    pub gate_con48: ReadWrite<u32, CRU_GATE_CON48::Register>,
    pub gate_con49: ReadWrite<u32, CRU_GATE_CON49::Register>,
    pub gate_con50: ReadWrite<u32, CRU_GATE_CON50::Register>,
    pub gate_con51: ReadWrite<u32, CRU_GATE_CON51::Register>,
    pub gate_con52: ReadWrite<u32, CRU_GATE_CON52::Register>,
    pub gate_con53: ReadWrite<u32, CRU_GATE_CON53::Register>,
    _reserved2: u32,
    pub gate_con55: ReadWrite<u32, CRU_GATE_CON55::Register>,
    pub gate_con56: ReadWrite<u32, CRU_GATE_CON56::Register>,
    pub gate_con57: ReadWrite<u32, CRU_GATE_CON57::Register>,
    _reserved3: u32,
    pub gate_con59: ReadWrite<u32, CRU_GATE_CON59::Register>,
    pub gate_con60: ReadWrite<u32, CRU_GATE_CON60::Register>,
    pub gate_con61: ReadWrite<u32, CRU_GATE_CON61::Register>,
    pub gate_con62: ReadWrite<u32, CRU_GATE_CON62::Register>,
    pub gate_con63: ReadWrite<u32, CRU_GATE_CON63::Register>,
    pub gate_con64: ReadWrite<u32, CRU_GATE_CON64::Register>,
    pub gate_con65: ReadWrite<u32, CRU_GATE_CON65::Register>,
    pub gate_con66: ReadWrite<u32, CRU_GATE_CON66::Register>,
    pub gate_con67: ReadWrite<u32, CRU_GATE_CON67::Register>,
    pub gate_con68: ReadWrite<u32, CRU_GATE_CON68::Register>,
    pub gate_con69: ReadWrite<u32, CRU_GATE_CON69::Register>,
    pub gate_con70: ReadWrite<u32, CRU_GATE_CON70::Register>,
    _reserved4: u32,
    pub gate_con72: ReadWrite<u32, CRU_GATE_CON72::Register>,
    pub gate_con73: ReadWrite<u32, CRU_GATE_CON73::Register>,
    pub gate_con74: ReadWrite<u32, CRU_GATE_CON74::Register>,
    pub gate_con75: ReadWrite<u32, CRU_GATE_CON75::Register>,
    pub gate_con76: ReadWrite<u32, CRU_GATE_CON76::Register>,
}

// CRU_GATE_CON00  0x0800;
generate_register_bitfields!(
    CRU_GATE_CON00,
    u32,
    [
        (clk_matrix_50m_src_en, 0, 1, []),
        (clk_matrix_100m_src_en, 1, 1, []),
        (clk_matrix_150m_src_en, 2, 1, []),
        (clk_matrix_200m_src_en, 3, 1, []),
        (clk_matrix_250m_src_en, 4, 1, []),
        (clk_matrix_300m_src_en, 5, 1, []),
        (clk_matrix_350m_src_en, 6, 1, []),
        (clk_matrix_400m_src_en, 7, 1, []),
        (clk_matrix_450m_src_en, 8, 1, []),
        (clk_matrix_500m_src_en, 9, 1, []),
        (clk_matrix_600m_src_en, 10, 1, []),
        (clk_matrix_650m_src_en, 11, 1, []),
        (clk_matrix_700m_src_en, 12, 1, []),
        (clk_matrix_800m_src_en, 13, 1, []),
        (clk_matrix_1000m_src_en, 14, 1, []),
        (clk_matrix_1200m_src_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON01  0x0804;
generate_register_bitfields!(
    CRU_GATE_CON01,
    u32,
    [
        (aclk_top_root_en, 0, 1, []),
        (pclk_top_root_en, 1, 1, []),
        (aclk_low_top_root_en, 2, 1, []),
        (aclk_top_biu_en, 3, 1, []),
        (pclk_top_biu_en, 4, 1, []),
        (reserved0, 5, 1, []),
        (pclk_csiphy0_en, 6, 1, []),
        (reserved1, 7, 1, []),
        (pclk_csiphy1_en, 8, 1, []),
        (reserved2, 9, 1, []),
        (aclk_top_m300_root_en, 10, 1, []),
        (aclk_top_m500_root_en, 11, 1, []),
        (aclk_top_m400_root_en, 12, 1, []),
        (aclk_top_s200_root_en, 13, 1, []),
        (aclk_top_s400_root_en, 14, 1, []),
        (aclk_top_m500_biu_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON02  0x0808;
generate_register_bitfields!(
    CRU_GATE_CON02,
    u32,
    [
        (aclk_top_m400_biu_en, 0, 1, []),
        (aclk_top_s200_biu_en, 1, 1, []),
        (aclk_top_s400_biu_en, 2, 1, []),
        (aclk_top_m300_biu_en, 3, 1, []),
        (clk_testout_top_en, 4, 1, []),
        (reserved0, 5, 1, []),
        (clk_testout_grp0_en, 6, 1, []),
        (reserved1, 7, 1, []),
        (clk_usbdp_combo_phy0_immortal_en, 8, 1, []),
        (reserved2, 9, 6, []),
        (clk_usbdp_combo_phy1_immortal_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON03  0x080C;
generate_register_bitfields!(
    CRU_GATE_CON03,
    u32,
    [
        (reserved, 0, 14, []),
        (pclk_mipi_dcphy0_en, 14, 1, []),
        (pclk_mipi_dcphy0_grf_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON04  0x0810;
generate_register_bitfields!(
    CRU_GATE_CON04,
    u32,
    [
        (reserved0, 0, 3, []),
        (pclk_mipi_dcphy1_en, 3, 1, []),
        (pclk_mipi_dcphy1_grf_en, 4, 1, []),
        (pclk_apb2asb_slv_cdphy_en, 5, 1, []),
        (pclk_apb2asb_slv_csiphy_en, 6, 1, []),
        (pclk_apb2asb_slv_vccio3_5_en, 7, 1, []),
        (pclk_apb2asb_slv_vccio6_en, 8, 1, []),
        (pclk_apb2asb_slv_emmcio_en, 9, 1, []),
        (pclk_apb2asb_slv_ioc_top_en, 10, 1, []),
        (pclk_apb2asb_slv_ioc_right_en, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON05  0x0814;
generate_register_bitfields!(
    CRU_GATE_CON05,
    u32,
    [
        (pclk_cru_en, 0, 1, []),
        (reserved, 1, 2, []),
        (mclk_gmac0_out_en, 3, 1, []),
        (refclko25m_eth0_out_en, 4, 1, []),
        (refclko25m_eth1_out_en, 5, 1, []),
        (clk_cifout_out_en, 6, 1, []),
        (aclk_channel_secure2vo1usb_en, 7, 1, []),
        (aclk_channel_secure2center_en, 8, 1, []),
        (clk_mipi_cameraout_m0_en, 9, 1, []),
        (clk_mipi_cameraout_m1_en, 10, 1, []),
        (clk_mipi_cameraout_m2_en, 11, 1, []),
        (clk_mipi_cameraout_m3_en, 12, 1, []),
        (clk_mipi_cameraout_m4_en, 13, 1, []),
        (hclk_channel_secure2vo1usb_en, 14, 1, []),
        (hclk_channel_secure2center_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON06  0x0818;
generate_register_bitfields!(
    CRU_GATE_CON06,
    u32,
    [
        (pclk_channel_secure2vo1usb_en, 0, 1, []),
        (pclk_channel_secure2center_en, 1, 1, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON07  0x081C;
generate_register_bitfields!(
    CRU_GATE_CON07,
    u32,
    [
        (hclk_audio_root_en, 0, 1, []),
        (pclk_audio_root_en, 1, 1, []),
        (hclk_audio_biu_en, 2, 1, []),
        (pclk_audio_biu_en, 3, 1, []),
        (hclk_i2s0_8ch_en, 4, 1, []),
        (clk_i2s0_8ch_tx_en, 5, 1, []),
        (clk_i2s0_8ch_frac_tx_en, 6, 1, []),
        (mclk_i2s0_8ch_tx_en, 7, 1, []),
        (clk_i2s0_8ch_rx_en, 8, 1, []),
        (clk_i2s0_8ch_frac_rx_en, 9, 1, []),
        (mclk_i2s0_8ch_rx_en, 10, 1, []),
        (pclk_acdcdig_en, 11, 1, []),
        (hclk_i2s2_2ch_en, 12, 1, []),
        (hclk_i2s3_2ch_en, 13, 1, []),
        (clk_i2s2_2ch_en, 14, 1, []),
        (clk_i2s2_2ch_frac_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON08  0x0820;
generate_register_bitfields!(
    CRU_GATE_CON08,
    u32,
    [
        (mclk_i2s2_2ch_en, 0, 1, []),
        (clk_i2s3_2ch_en, 1, 1, []),
        (clk_i2s3_2ch_frac_en, 2, 1, []),
        (mclk_i2s3_2ch_en, 3, 1, []),
        (clk_dac_acdcdig_en, 4, 1, []),
        (reserved, 5, 9, []),
        (hclk_spdif0_en, 14, 1, []),
        (clk_spdif0_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON09  0x0824;
generate_register_bitfields!(
    CRU_GATE_CON09,
    u32,
    [
        (clk_spdif0_frac_en, 0, 1, []),
        (mclk_spdif0_en, 1, 1, []),
        (hclk_spdif1_en, 2, 1, []),
        (clk_spdif1_en, 3, 1, []),
        (clk_spdif1_frac_en, 4, 1, []),
        (mclk_spdif1_en, 5, 1, []),
        (hclk_pdm1_en, 6, 1, []),
        (mclk_pdm1_en, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON10  0x0828;
generate_register_bitfields!(
    CRU_GATE_CON10,
    u32,
    [
        (aclk_bus_root_en, 0, 1, []),
        (aclk_bus_biu_en, 1, 1, []),
        (pclk_bus_biu_en, 2, 1, []),
        (aclk_gic_en, 3, 1, []),
        (reserved, 4, 1, []),
        (aclk_dmac0_en, 5, 1, []),
        (aclk_dmac1_en, 6, 1, []),
        (aclk_dmac2_en, 7, 1, []),
        (pclk_i2c1_en, 8, 1, []),
        (pclk_i2c2_en, 9, 1, []),
        (pclk_i2c3_en, 10, 1, []),
        (pclk_i2c4_en, 11, 1, []),
        (pclk_i2c5_en, 12, 1, []),
        (pclk_i2c6_en, 13, 1, []),
        (pclk_i2c7_en, 14, 1, []),
        (pclk_i2c8_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON11  0x082C;
generate_register_bitfields!(
    CRU_GATE_CON11,
    u32,
    [
        (clk_i2c1_en, 0, 1, []),
        (clk_i2c2_en, 1, 1, []),
        (clk_i2c3_en, 2, 1, []),
        (clk_i2c4_en, 3, 1, []),
        (clk_i2c5_en, 4, 1, []),
        (clk_i2c6_en, 5, 1, []),
        (clk_i2c7_en, 6, 1, []),
        (clk_i2c8_en, 7, 1, []),
        (pclk_can0_en, 8, 1, []),
        (clk_can0_en, 9, 1, []),
        (pclk_can1_en, 10, 1, []),
        (clk_can1_en, 11, 1, []),
        (pclk_can2_en, 12, 1, []),
        (clk_can2_en, 13, 1, []),
        (pclk_saradc_en, 14, 1, []),
        (clk_saradc_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON12  0x0830;
generate_register_bitfields!(
    CRU_GATE_CON12,
    u32,
    [
        (pclk_tsadc_en, 0, 1, []),
        (clk_tsadc_en, 1, 1, []),
        (pclk_uart1_en, 2, 1, []),
        (pclk_uart2_en, 3, 1, []),
        (pclk_uart3_en, 4, 1, []),
        (pclk_uart4_en, 5, 1, []),
        (pclk_uart5_en, 6, 1, []),
        (pclk_uart6_en, 7, 1, []),
        (pclk_uart7_en, 8, 1, []),
        (pclk_uart8_en, 9, 1, []),
        (pclk_uart9_en, 10, 1, []),
        (clk_uart1_en, 11, 1, []),
        (clk_uart1_frac_en, 12, 1, []),
        (sclk_uart1_en, 13, 1, []),
        (clk_uart2_en, 14, 1, []),
        (clk_uart2_frac_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON13  0x0834;
generate_register_bitfields!(
    CRU_GATE_CON13,
    u32,
    [
        (sclk_uart2_en, 0, 1, []),
        (clk_uart3_en, 1, 1, []),
        (clk_uart3_frac_en, 2, 1, []),
        (sclk_uart3_en, 3, 1, []),
        (clk_uart4_en, 4, 1, []),
        (clk_uart4_frac_en, 5, 1, []),
        (sclk_uart4_en, 6, 1, []),
        (clk_uart5_en, 7, 1, []),
        (clk_uart5_frac_en, 8, 1, []),
        (sclk_uart5_en, 9, 1, []),
        (clk_uart6_en, 10, 1, []),
        (clk_uart6_frac_en, 11, 1, []),
        (sclk_uart6_en, 12, 1, []),
        (clk_uart7_en, 13, 1, []),
        (clk_uart7_frac_en, 14, 1, []),
        (sclk_uart7_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON14  0x0838;
generate_register_bitfields!(
    CRU_GATE_CON14,
    u32,
    [
        (clk_uart8_en, 0, 1, []),
        (clk_uart8_frac_en, 1, 1, []),
        (sclk_uart8_en, 2, 1, []),
        (clk_uart9_en, 3, 1, []),
        (clk_uart9_frac_en, 4, 1, []),
        (sclk_uart9_en, 5, 1, []),
        (pclk_spi0_en, 6, 1, []),
        (pclk_spi1_en, 7, 1, []),
        (pclk_spi2_en, 8, 1, []),
        (pclk_spi3_en, 9, 1, []),
        (pclk_spi4_en, 10, 1, []),
        (clk_spi0_en, 11, 1, []),
        (clk_spi1_en, 12, 1, []),
        (clk_spi2_en, 13, 1, []),
        (clk_spi3_en, 14, 1, []),
        (clk_spi4_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON15  0x083C;
generate_register_bitfields!(
    CRU_GATE_CON15,
    u32,
    [
        (pclk_wdt0_en, 0, 1, []),
        (tclk_wdt0_en, 1, 1, []),
        (pclk_sys_grf_en, 2, 1, []),
        (pclk_pwm1_en, 3, 1, []),
        (clk_pwm1_en, 4, 1, []),
        (clk_pwm1_capture_en, 5, 1, []),
        (pclk_pwm2_en, 6, 1, []),
        (clk_pwm2_en, 7, 1, []),
        (clk_pwm2_capture_en, 8, 1, []),
        (pclk_pwm3_en, 9, 1, []),
        (clk_pwm3_en, 10, 1, []),
        (clk_pwm3_capture_en, 11, 1, []),
        (pclk_bustimer0_en, 12, 1, []),
        (pclk_bustimer1_en, 13, 1, []),
        (clk_bustimer_root_en, 14, 1, []),
        (clk_bustimer0_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON16  0x0840;
generate_register_bitfields!(
    CRU_GATE_CON16,
    u32,
    [
        (clk_bustimer1_en, 0, 1, []),
        (clk_bustimer2_en, 1, 1, []),
        (clk_bustimer3_en, 2, 1, []),
        (clk_bustimer4_en, 3, 1, []),
        (clk_bustimer5_en, 4, 1, []),
        (clk_bustimer6_en, 5, 1, []),
        (clk_bustimer7_en, 6, 1, []),
        (clk_bustimer8_en, 7, 1, []),
        (clk_bustimer9_en, 8, 1, []),
        (clk_bustimer10_en, 9, 1, []),
        (clk_bustimer11_en, 10, 1, []),
        (pclk_mailbox0_en, 11, 1, []),
        (pclk_mailbox1_en, 12, 1, []),
        (pclk_mailbox2_en, 13, 1, []),
        (pclk_gpio1_en, 14, 1, []),
        (dbclk_gpio1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON17  0x0844;
generate_register_bitfields!(
    CRU_GATE_CON17,
    u32,
    [
        (pclk_gpio2_en, 0, 1, []),
        (dbclk_gpio2_en, 1, 1, []),
        (pclk_gpio3_en, 2, 1, []),
        (dbclk_gpio3_en, 3, 1, []),
        (pclk_gpio4_en, 4, 1, []),
        (dbclk_gpio4_en, 5, 1, []),
        (aclk_decom_en, 6, 1, []),
        (pclk_decom_en, 7, 1, []),
        (dclk_decom_en, 8, 1, []),
        (pclk_top_en, 9, 1, []),
        (reserved, 10, 1, []),
        (aclk_gicadb_gic2core_bus_en, 11, 1, []),
        (pclk_dft2apb_en, 12, 1, []),
        (pclk_apb2asb_mst_top_en, 13, 1, []),
        (pclk_apb2asb_mst_cdphy_en, 14, 1, []),
        (pclk_apb2asb_mst_bot_right_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON18  0x0848;
generate_register_bitfields!(
    CRU_GATE_CON18,
    u32,
    [
        (pclk_apb2asb_mst_ioc_top_en, 0, 1, []),
        (pclk_apb2asb_mst_ioc_right_en, 1, 1, []),
        (pclk_apb2asb_mst_csiphy_en, 2, 1, []),
        (pclk_apb2asb_mst_vccio3_5_en, 3, 1, []),
        (pclk_apb2asb_mst_vccio6_en, 4, 1, []),
        (pclk_apb2asb_mst_emmcio_en, 5, 1, []),
        (aclk_spinlock_en, 6, 1, []),
        (reserved0, 7, 2, []),
        (pclk_otpc_ns_en, 9, 1, []),
        (clk_otpc_ns_en, 10, 1, []),
        (clk_otpc_arb_en, 11, 1, []),
        (clk_otpc_auto_rd_en, 12, 1, []),
        (clk_otp_phy_en, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON19  0x084C;
generate_register_bitfields!(
    CRU_GATE_CON19,
    u32,
    [
        (pclk_busioc_en, 0, 1, []),
        (clk_bisrintf_pllsrc_en, 1, 1, []),
        (clk_bisrintf_en, 2, 1, []),
        (pclk_pmu2_en, 3, 1, []),
        (pclk_pmucm0_intmux_en, 4, 1, []),
        (pclk_ddrcm0_intmux_en, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON20  0x0850;
generate_register_bitfields!(
    CRU_GATE_CON20,
    u32,
    [
        (pclk_ddr_dfictl_ch0_en, 0, 1, []),
        (pclk_ddr_mon_ch0_en, 1, 1, []),
        (pclk_ddr_standby_ch0_en, 2, 1, []),
        (pclk_ddr_upctl_ch0_en, 3, 1, []),
        (tmclk_ddr_mon_ch0_en, 4, 1, []),
        (pclk_ddr_grf_ch01_en, 5, 1, []),
        (clk_dfi_ch0_en, 6, 1, []),
        (clk_sbr_ch0_en, 7, 1, []),
        (clk_ddr_upctl_ch0_en, 8, 1, []),
        (clk_ddr_dfictl_ch0_en, 9, 1, []),
        (clk_ddr_mon_ch0_en, 10, 1, []),
        (clk_ddr_standby_ch0_en, 11, 1, []),
        (aclk_ddr_upctl_ch0_en, 12, 1, []),
        (pclk_ddr_dfictl_ch1_en, 13, 1, []),
        (pclk_ddr_mon_ch1_en, 14, 1, []),
        (pclk_ddr_standby_ch1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON21  0x0854;
generate_register_bitfields!(
    CRU_GATE_CON21,
    u32,
    [
        (pclk_ddr_upctl_ch1_en, 0, 1, []),
        (tmclk_ddr_mon_ch1_en, 1, 1, []),
        (clk_dfi_ch1_en, 2, 1, []),
        (clk_sbr_ch1_en, 3, 1, []),
        (clk_ddr_upctl_ch1_en, 4, 1, []),
        (clk_ddr_dfictl_ch1_en, 5, 1, []),
        (clk_ddr_mon_ch1_en, 6, 1, []),
        (clk_ddr_standby_ch1_en, 7, 1, []),
        (aclk_ddr_upctl_ch1_en, 8, 1, []),
        (reserved, 9, 4, []),
        (aclk_ddr_ddrsch0_en, 13, 1, []),
        (aclk_ddr_rs_ddrsch0_en, 14, 1, []),
        (aclk_ddr_frs_ddrsch0_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON22  0x0858;
generate_register_bitfields!(
    CRU_GATE_CON22,
    u32,
    [
        (aclk_ddr_scramble0_en, 0, 1, []),
        (aclk_ddr_frs_scramble0_en, 1, 1, []),
        (aclk_ddr_ddrsch1_en, 2, 1, []),
        (aclk_ddr_rs_ddrsch1_en, 3, 1, []),
        (aclk_ddr_frs_ddrsch1_en, 4, 1, []),
        (aclk_ddr_scramble1_en, 5, 1, []),
        (aclk_ddr_frs_scramble1_en, 6, 1, []),
        (pclk_ddr_ddrsch0_en, 7, 1, []),
        (pclk_ddr_ddrsch1_en, 8, 1, []),
        (clk_testout_ddr01_en, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON23  0x085C;
generate_register_bitfields!(
    CRU_GATE_CON23,
    u32,
    [
        (pclk_ddr_dfictl_ch2_en, 0, 1, []),
        (pclk_ddr_mon_ch2_en, 1, 1, []),
        (pclk_ddr_standby_ch2_en, 2, 1, []),
        (pclk_ddr_upctl_ch2_en, 3, 1, []),
        (tmclk_ddr_mon_ch2_en, 4, 1, []),
        (pclk_ddr_grf_ch23_en, 5, 1, []),
        (clk_dfi_ch2_en, 6, 1, []),
        (clk_sbr_ch2_en, 7, 1, []),
        (clk_ddr_upctl_ch2_en, 8, 1, []),
        (clk_ddr_dfictl_ch2_en, 9, 1, []),
        (clk_ddr_mon_ch2_en, 10, 1, []),
        (clk_ddr_standby_ch2_en, 11, 1, []),
        (aclk_ddr_upctl_ch2_en, 12, 1, []),
        (pclk_ddr_dfictl_ch3_en, 13, 1, []),
        (pclk_ddr_mon_ch3_en, 14, 1, []),
        (pclk_ddr_standby_ch3_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON24  0x0860;
generate_register_bitfields!(
    CRU_GATE_CON24,
    u32,
    [
        (pclk_ddr_upctl_ch3_en, 0, 1, []),
        (tmclk_ddr_mon_ch3_en, 1, 1, []),
        (clk_dfi_ch3_en, 2, 1, []),
        (clk_sbr_ch3_en, 3, 1, []),
        (clk_ddr_upctl_ch3_en, 4, 1, []),
        (clk_ddr_dfictl_ch3_en, 5, 1, []),
        (clk_ddr_mon_ch3_en, 6, 1, []),
        (clk_ddr_standby_ch3_en, 7, 1, []),
        (aclk_ddr_upctl_ch3_en, 8, 1, []),
        (reserved, 9, 4, []),
        (aclk_ddr_ddrsch2_en, 13, 1, []),
        (aclk_ddr_rs_ddrsch2_en, 14, 1, []),
        (aclk_ddr_frs_ddrsch2_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON25  0x0864;
generate_register_bitfields!(
    CRU_GATE_CON25,
    u32,
    [
        (aclk_ddr_scramble2_en, 0, 1, []),
        (aclk_ddr_frs_scramble2_en, 1, 1, []),
        (aclk_ddr_ddrsch3_en, 2, 1, []),
        (aclk_ddr_rs_ddrsch3_en, 3, 1, []),
        (aclk_ddr_frs_ddrsch3_en, 4, 1, []),
        (aclk_ddr_scramble3_en, 5, 1, []),
        (aclk_ddr_frs_scramble3_en, 6, 1, []),
        (pclk_ddr_ddrsch2_en, 7, 1, []),
        (pclk_ddr_ddrsch3_en, 8, 1, []),
        (clk_testout_ddr23_en, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON26  0x0868;
generate_register_bitfields!(
    CRU_GATE_CON26,
    u32,
    [
        (aclk_isp1_root_en, 0, 1, []),
        (hclk_isp1_root_en, 1, 1, []),
        (clk_isp1_core_en, 2, 1, []),
        (clk_isp1_core_marvin_en, 3, 1, []),
        (clk_isp1_core_vicap_en, 4, 1, []),
        (aclk_isp1_en, 5, 1, []),
        (aclk_isp1_biu_en, 6, 1, []),
        (hclk_isp1_en, 7, 1, []),
        (hclk_isp1_biu_en, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON27  0x086C;
generate_register_bitfields!(
    CRU_GATE_CON27,
    u32,
    [
        (aclk_rknn1_en, 0, 1, []),
        (aclk_rknn1_biu_en, 1, 1, []),
        (hclk_rknn1_en, 2, 1, []),
        (hclk_rknn1_biu_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON28  0x0870;
generate_register_bitfields!(
    CRU_GATE_CON28,
    u32,
    [
        (aclk_rknn2_en, 0, 1, []),
        (aclk_rknn2_biu_en, 1, 1, []),
        (hclk_rknn2_en, 2, 1, []),
        (hclk_rknn2_biu_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON29  0x0874;
generate_register_bitfields!(
    CRU_GATE_CON29,
    u32,
    [
        (hclk_rknn_root_en, 0, 1, []),
        (clk_rknn_dsu0_df_en, 1, 1, []),
        (clk_testout_npu_en, 2, 1, []),
        (clk_rknn_dsu0_en, 3, 1, []),
        (pclk_nputop_root_en, 4, 1, []),
        (pclk_nputop_biu_en, 5, 1, []),
        (pclk_npu_timer_en, 6, 1, []),
        (clk_nputimer_root_en, 7, 1, []),
        (clk_nputimer0_en, 8, 1, []),
        (clk_nputimer1_en, 9, 1, []),
        (pclk_npu_wdt_en, 10, 1, []),
        (tclk_npu_wdt_en, 11, 1, []),
        (pclk_pvtm1_en, 12, 1, []),
        (pclk_npu_grf_en, 13, 1, []),
        (clk_pvtm1_en, 14, 1, []),
        (clk_npu_pvtm_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON30  0x0878;
generate_register_bitfields!(
    CRU_GATE_CON30,
    u32,
    [
        (clk_npu_pvtpll_en, 0, 1, []),
        (hclk_npu_cm0_root_en, 1, 1, []),
        (hclk_npu_cm0_biu_en, 2, 1, []),
        (fclk_npu_cm0_core_en, 3, 1, []),
        (reserved0, 4, 1, []),
        (clk_npu_cm0_rtc_en, 5, 1, []),
        (aclk_rknn0_en, 6, 1, []),
        (aclk_rknn0_biu_en, 7, 1, []),
        (hclk_rknn0_en, 8, 1, []),
        (hclk_rknn0_biu_en, 9, 1, []),
        (reserved1, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON31  0x087C;
generate_register_bitfields!(
    CRU_GATE_CON31,
    u32,
    [
        (hclk_nvm_root_en, 0, 1, []),
        (aclk_nvm_root_en, 1, 1, []),
        (hclk_nvm_biu_en, 2, 1, []),
        (aclk_nvm_biu_en, 3, 1, []),
        (hclk_emmc_en, 4, 1, []),
        (aclk_emmc_en, 5, 1, []),
        (cclk_emmc_en, 6, 1, []),
        (bclk_emmc_en, 7, 1, []),
        (tmclk_emmc_en, 8, 1, []),
        (sclk_sfc_en, 9, 1, []),
        (hclk_sfc_en, 10, 1, []),
        (hclk_sfc_xip_en, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON32  0x0880;
generate_register_bitfields!(
    CRU_GATE_CON32,
    u32,
    [
        (pclk_php_root_en, 0, 1, []),
        (pclk_grf_en, 1, 1, []),
        (pclk_dec_biu_en, 2, 1, []),
        (pclk_gmac0_en, 3, 1, []),
        (pclk_gmac1_en, 4, 1, []),
        (pclk_php_biu_en, 5, 1, []),
        (aclk_pcie_root_en, 6, 1, []),
        (aclk_php_root_en, 7, 1, []),
        (aclk_pcie_bridge_en, 8, 1, []),
        (aclk_php_biu_en, 9, 1, []),
        (aclk_gmac0_en, 10, 1, []),
        (aclk_gmac1_en, 11, 1, []),
        (aclk_pcie_biu_en, 12, 1, []),
        (aclk_pcie_4l_dbi_en, 13, 1, []),
        (aclk_pcie_2l_dbi_en, 14, 1, []),
        (aclk_pcie_1l0_dbi_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON33  0x0884;
generate_register_bitfields!(
    CRU_GATE_CON33,
    u32,
    [
        (aclk_pcie_1l1_dbi_en, 0, 1, []),
        (aclk_pcie_1l2_dbi_en, 1, 1, []),
        (aclk_pcie_4l_mstr_en, 2, 1, []),
        (aclk_pcie_2l_mstr_en, 3, 1, []),
        (aclk_pcie_1l0_mstr_en, 4, 1, []),
        (aclk_pcie_1l1_mstr_en, 5, 1, []),
        (aclk_pcie_1l2_mstr_en, 6, 1, []),
        (aclk_pcie_4l_slv_en, 7, 1, []),
        (aclk_pcie_2l_slv_en, 8, 1, []),
        (aclk_pcie_1l0_slv_en, 9, 1, []),
        (aclk_pcie_1l1_slv_en, 10, 1, []),
        (aclk_pcie_1l2_slv_en, 11, 1, []),
        (pclk_pcie_4l_en, 12, 1, []),
        (pclk_pcie_2l_en, 13, 1, []),
        (pclk_pcie_1l0_en, 14, 1, []),
        (pclk_pcie_1l1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON34  0x0888;
generate_register_bitfields!(
    CRU_GATE_CON34,
    u32,
    [
        (pclk_pcie_1l2_en, 0, 1, []),
        (clk_pcie_4l_aux_en, 1, 1, []),
        (clk_pcie_2l_aux_en, 2, 1, []),
        (clk_pcie_1l0_aux_en, 3, 1, []),
        (clk_pcie_1l1_aux_en, 4, 1, []),
        (clk_pcie_1l2_aux_en, 5, 1, []),
        (aclk_php_gic_its_en, 6, 1, []),
        (aclk_mmu_pcie_en, 7, 1, []),
        (aclk_mmu_php_en, 8, 1, []),
        (aclk_mmu_biu_en, 9, 1, []),
        (clk_gmac0_ptp_ref_en, 10, 1, []),
        (clk_gmac1_ptp_ref_en, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON35  0x088C;
generate_register_bitfields!(
    CRU_GATE_CON35,
    u32,
    [
        (reserved0, 0, 5, []),
        (clk_gmac_125m_cru_en, 5, 1, []),
        (clk_gmac_50m_cru_en, 6, 1, []),
        (aclk_usb3otg2_en, 7, 1, []),
        (suspend_clk_usb3otg2_en, 8, 1, []),
        (ref_clk_usb3otg2_en, 9, 1, []),
        (clk_utmi_otg2_en, 10, 1, []),
        (reserved1, 11, 5, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON37  0x0894;
generate_register_bitfields!(
    CRU_GATE_CON37,
    u32,
    [
        (clk_pipephy0_ref_en, 0, 1, []),
        (clk_pipephy1_ref_en, 1, 1, []),
        (clk_pipephy2_ref_en, 2, 1, []),
        (reserved0, 3, 1, []),
        (clk_pmalive0_en, 4, 1, []),
        (clk_pmalive1_en, 5, 1, []),
        (clk_pmalive2_en, 6, 1, []),
        (aclk_sata0_en, 7, 1, []),
        (aclk_sata1_en, 8, 1, []),
        (aclk_sata2_en, 9, 1, []),
        (clk_rxoob0_en, 10, 1, []),
        (clk_rxoob1_en, 11, 1, []),
        (clk_rxoob2_en, 12, 1, []),
        (reserved1, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON38  0x0898;
generate_register_bitfields!(
    CRU_GATE_CON38,
    u32,
    [
        (reserved0, 0, 3, []),
        (clk_pipephy0_pipe_g_en, 3, 1, []),
        (clk_pipephy1_pipe_g_en, 4, 1, []),
        (clk_pipephy2_pipe_g_en, 5, 1, []),
        (clk_pipephy0_pipe_asic_g_en, 6, 1, []),
        (clk_pipephy1_pipe_asic_g_en, 7, 1, []),
        (clk_pipephy2_pipe_asic_g_en, 8, 1, []),
        (clk_pipephy2_pipe_u3_g_en, 9, 1, []),
        (reserved1, 10, 3, []),
        (clk_pcie_1l2_pipe_en, 13, 1, []),
        (clk_pcie_1l0_pipe_en, 14, 1, []),
        (clk_pcie_1l1_pipe_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON39  0x089C;
generate_register_bitfields!(
    CRU_GATE_CON39,
    u32,
    [
        (clk_pcie_4l_pipe_en, 0, 1, []),
        (clk_pcie_2l_pipe_en, 1, 1, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON40  0x08A0;
generate_register_bitfields!(
    CRU_GATE_CON40,
    u32,
    [
        (hclk_rkvdec0_root_en, 0, 1, []),
        (aclk_rkvdec0_root_en, 1, 1, []),
        (aclk_rkvdec_ccu_en, 2, 1, []),
        (hclk_rkvdec0_en, 3, 1, []),
        (aclk_rkvdec0_en, 4, 1, []),
        (hclk_rkvdec0_biu_en, 5, 1, []),
        (aclk_rkvdec0_biu_en, 6, 1, []),
        (clk_rkvdec0_ca_en, 7, 1, []),
        (clk_rkvdec0_hevc_ca_en, 8, 1, []),
        (clk_rkvdec0_core_en, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON41  0x08A4;
generate_register_bitfields!(
    CRU_GATE_CON41,
    u32,
    [
        (hclk_rkvdec1_root_en, 0, 1, []),
        (aclk_rkvdec1_root_en, 1, 1, []),
        (hclk_rkvdec1_en, 2, 1, []),
        (aclk_rkvdec1_en, 3, 1, []),
        (hclk_rkvdec1_biu_en, 4, 1, []),
        (aclk_rkvdec1_biu_en, 5, 1, []),
        (clk_rkvdec1_ca_en, 6, 1, []),
        (clk_rkvdec1_hevc_ca_en, 7, 1, []),
        (clk_rkvdec1_core_en, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON42  0x08A8;
generate_register_bitfields!(
    CRU_GATE_CON42,
    u32,
    [
        (aclk_usb_root_en, 0, 1, []),
        (hclk_usb_root_en, 1, 1, []),
        (aclk_usb_biu_en, 2, 1, []),
        (hclk_usb_biu_en, 3, 1, []),
        (aclk_usb3otg0_en, 4, 1, []),
        (suspend_clk_usb3otg0_en, 5, 1, []),
        (ref_clk_usb3otg0_en, 6, 1, []),
        (aclk_usb3otg1_en, 7, 1, []),
        (suspend_clk_usb3otg1_en, 8, 1, []),
        (ref_clk_usb3otg1_en, 9, 1, []),
        (hclk_host0_en, 10, 1, []),
        (hclk_host_arb0_en, 11, 1, []),
        (hclk_host1_en, 12, 1, []),
        (hclk_host_arb1_en, 13, 1, []),
        (aclk_usb_grf_en, 14, 1, []),
        (utmi_ohci_clk48_host0_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON43  0x08AC;
generate_register_bitfields!(
    CRU_GATE_CON43,
    u32,
    [
        (utmi_ohci_clk48_host1_en, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON44  0x08B0;
generate_register_bitfields!(
    CRU_GATE_CON44,
    u32,
    [
        (aclk_vdpu_root_en, 0, 1, []),
        (aclk_vdpu_low_root_en, 1, 1, []),
        (hclk_vdpu_root_en, 2, 1, []),
        (aclk_jpeg_decoder_root_en, 3, 1, []),
        (aclk_vdpu_biu_en, 4, 1, []),
        (aclk_vdpu_low_biu_en, 5, 1, []),
        (hclk_vdpu_biu_en, 6, 1, []),
        (aclk_jpeg_decoder_biu_en, 7, 1, []),
        (aclk_vpu_en, 8, 1, []),
        (hclk_vpu_en, 9, 1, []),
        (aclk_jpeg_encoder0_en, 10, 1, []),
        (hclk_jpeg_encoder0_en, 11, 1, []),
        (aclk_jpeg_encoder1_en, 12, 1, []),
        (hclk_jpeg_encoder1_en, 13, 1, []),
        (aclk_jpeg_encoder2_en, 14, 1, []),
        (hclk_jpeg_encoder2_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON45  0x08B4;
generate_register_bitfields!(
    CRU_GATE_CON45,
    u32,
    [
        (aclk_jpeg_encoder3_en, 0, 1, []),
        (hclk_jpeg_encoder3_en, 1, 1, []),
        (aclk_jpeg_decoder_en, 2, 1, []),
        (hclk_jpeg_decoder_en, 3, 1, []),
        (hclk_iep2p0_en, 4, 1, []),
        (aclk_iep2p0_en, 5, 1, []),
        (clk_iep2p0_core_en, 6, 1, []),
        (hclk_rga2_en, 7, 1, []),
        (aclk_rga2_en, 8, 1, []),
        (clk_rga2_core_en, 9, 1, []),
        (hclk_rga3_0_en, 10, 1, []),
        (aclk_rga3_0_en, 11, 1, []),
        (clk_rga3_0_core_en, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON47  0x08BC;
generate_register_bitfields!(
    CRU_GATE_CON47,
    u32,
    [
        (hclk_rkvenc0_root_en, 0, 1, []),
        (aclk_rkvenc0_root_en, 1, 1, []),
        (hclk_rkvenc0_biu_en, 2, 1, []),
        (aclk_rkvenc0_biu_en, 3, 1, []),
        (hclk_rkvenc0_en, 4, 1, []),
        (aclk_rkvenc0_en, 5, 1, []),
        (clk_rkvenc0_core_en, 6, 1, []),
        (reserved, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON48  0x08C0;
generate_register_bitfields!(
    CRU_GATE_CON48,
    u32,
    [
        (hclk_rkvenc1_root_en, 0, 1, []),
        (aclk_rkvenc1_root_en, 1, 1, []),
        (hclk_rkvenc1_biu_en, 2, 1, []),
        (aclk_rkvenc1_biu_en, 3, 1, []),
        (hclk_rkvenc1_en, 4, 1, []),
        (aclk_rkvenc1_en, 5, 1, []),
        (clk_rkvenc1_core_en, 6, 1, []),
        (reserved, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON49  0x08C4;
generate_register_bitfields!(
    CRU_GATE_CON49,
    u32,
    [
        (aclk_vi_root_en, 0, 1, []),
        (hclk_vi_root_en, 1, 1, []),
        (pclk_vi_root_en, 2, 1, []),
        (aclk_vi_biu_en, 3, 1, []),
        (hclk_vi_biu_en, 4, 1, []),
        (pclk_vi_biu_en, 5, 1, []),
        (dclk_vicap_en, 6, 1, []),
        (aclk_vicap_en, 7, 1, []),
        (hclk_vicap_en, 8, 1, []),
        (clk_isp0_core_en, 9, 1, []),
        (clk_isp0_core_marvin_en, 10, 1, []),
        (clk_isp0_core_vicap_en, 11, 1, []),
        (aclk_isp0_en, 12, 1, []),
        (hclk_isp0_en, 13, 1, []),
        (aclk_fisheye0_en, 14, 1, []),
        (hclk_fisheye0_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON50  0x08C8;
generate_register_bitfields!(
    CRU_GATE_CON50,
    u32,
    [
        (clk_fisheye0_core_en, 0, 1, []),
        (aclk_fisheye1_en, 1, 1, []),
        (hclk_fisheye1_en, 2, 1, []),
        (clk_fisheye1_core_en, 3, 1, []),
        (pclk_csi_host_0_en, 4, 1, []),
        (pclk_csi_host_1_en, 5, 1, []),
        (pclk_csi_host_2_en, 6, 1, []),
        (pclk_csi_host_3_en, 7, 1, []),
        (pclk_csi_host_4_en, 8, 1, []),
        (pclk_csi_host_5_en, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON51  0x08CC;
generate_register_bitfields!(
    CRU_GATE_CON51,
    u32,
    [
        (reserved0, 0, 4, []),
        (clk_csihost0_vicap_en, 4, 1, []),
        (clk_csihost1_vicap_en, 5, 1, []),
        (clk_csihost2_vicap_en, 6, 1, []),
        (clk_csihost3_vicap_en, 7, 1, []),
        (clk_csihost4_vicap_en, 8, 1, []),
        (clk_csihost5_vicap_en, 9, 1, []),
        (iclk_csihost01_en, 10, 1, []),
        (iclk_csihost0_en, 11, 1, []),
        (iclk_csihost1_en, 12, 1, []),
        (reserved1, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON52  0x08D0;
generate_register_bitfields!(
    CRU_GATE_CON52,
    u32,
    [
        (aclk_vop_root_en, 0, 1, []),
        (aclk_vop_low_root_en, 1, 1, []),
        (hclk_vop_root_en, 2, 1, []),
        (pclk_vop_root_en, 3, 1, []),
        (aclk_vop_biu_en, 4, 1, []),
        (aclk_vop_low_biu_en, 5, 1, []),
        (hclk_vop_biu_en, 6, 1, []),
        (pclk_vop_biu_en, 7, 1, []),
        (hclk_vop_en, 8, 1, []),
        (aclk_vop_en, 9, 1, []),
        (dclk_vp0_src_en, 10, 1, []),
        (dclk_vp1_src_en, 11, 1, []),
        (dclk_vp2_src_en, 12, 1, []),
        (dclk_vp0_en, 13, 1, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON53  0x08D4;
generate_register_bitfields!(
    CRU_GATE_CON53,
    u32,
    [
        (dclk_vp1_en, 0, 1, []),
        (dclk_vp2_en, 1, 1, []),
        (dclk_vp3_en, 2, 1, []),
        (pclk_vopgrf_en, 3, 1, []),
        (pclk_dsihost0_en, 4, 1, []),
        (pclk_dsihost1_en, 5, 1, []),
        (clk_dsihost0_en, 6, 1, []),
        (clk_dsihost1_en, 7, 1, []),
        (clk_vop_pmu_en, 8, 1, []),
        (pclk_vop_channel_biu_en, 9, 1, []),
        (aclk_vop_doby_en, 10, 1, []),
        (reserved, 11, 5, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON55  0x08DC;
generate_register_bitfields!(
    CRU_GATE_CON55,
    u32,
    [
        (aclk_vo0_root_en, 0, 1, []),
        (hclk_vo0_root_en, 1, 1, []),
        (hclk_vo0_s_root_en, 2, 1, []),
        (pclk_vo0_root_en, 3, 1, []),
        (pclk_vo0_s_root_en, 4, 1, []),
        (hclk_vo0_biu_en, 5, 1, []),
        (hclk_vo0_s_biu_en, 6, 1, []),
        (pclk_vo0_biu_en, 7, 1, []),
        (pclk_vo0_s_biu_en, 8, 1, []),
        (aclk_hdcp0_biu_en, 9, 1, []),
        (pclk_vo0grf_en, 10, 1, []),
        (hclk_hdcp_key0_en, 11, 1, []),
        (aclk_hdcp0_en, 12, 1, []),
        (hclk_hdcp0_en, 13, 1, []),
        (pclk_hdcp0_en, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON56  0x08E0;
generate_register_bitfields!(
    CRU_GATE_CON56,
    u32,
    [
        (aclk_trng0_en, 0, 1, []),
        (pclk_trng0_en, 1, 1, []),
        (clk_aux16mhz_0_en, 2, 1, []),
        (clk_aux16mhz_1_en, 3, 1, []),
        (pclk_dp0_en, 4, 1, []),
        (pclk_dp1_en, 5, 1, []),
        (pclk_s_dp0_en, 6, 1, []),
        (pclk_s_dp1_en, 7, 1, []),
        (clk_dp0_en, 8, 1, []),
        (clk_dp1_en, 9, 1, []),
        (hclk_i2s4_8ch_en, 10, 1, []),
        (clk_i2s4_8ch_tx_en, 11, 1, []),
        (clk_i2s4_8ch_frac_tx_en, 12, 1, []),
        (mclk_i2s4_8ch_tx_en, 13, 1, []),
        (hclk_i2s8_8ch_en, 14, 1, []),
        (clk_i2s8_8ch_tx_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON57  0x08E4;
generate_register_bitfields!(
    CRU_GATE_CON57,
    u32,
    [
        (clk_i2s8_8ch_frac_tx_en, 0, 1, []),
        (mclk_i2s8_8ch_tx_en, 1, 1, []),
        (hclk_spdif2_dp0_en, 2, 1, []),
        (clk_spdif2_dp0_en, 3, 1, []),
        (clk_spdif2_dp0_frac_en, 4, 1, []),
        (mclk_spdif2_dp0_en, 5, 1, []),
        (mclk_spdif2_en, 6, 1, []),
        (hclk_spdif5_dp1_en, 7, 1, []),
        (clk_spdif5_dp1_en, 8, 1, []),
        (clk_spdif5_dp1_frac_en, 9, 1, []),
        (mclk_spdif5_dp1_en, 10, 1, []),
        (mclk_spdif5_en, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON59  0x08EC;
generate_register_bitfields!(
    CRU_GATE_CON59,
    u32,
    [
        (aclk_hdcp1_root_en, 0, 1, []),
        (aclk_hdmirx_root_en, 1, 1, []),
        (hclk_vo1_root_en, 2, 1, []),
        (hclk_vo1_s_root_en, 3, 1, []),
        (pclk_vo1_root_en, 4, 1, []),
        (pclk_vo1_s_root_en, 5, 1, []),
        (aclk_hdcp1_biu_en, 6, 1, []),
        (reserved, 7, 1, []),
        (aclk_vo1_biu_en, 8, 1, []),
        (hclk_vo1_biu_en, 9, 1, []),
        (hclk_vo1_s_biu_en, 10, 1, []),
        (pclk_vo1_biu_en, 11, 1, []),
        (pclk_vo1grf_en, 12, 1, []),
        (pclk_vo1_s_biu_en, 13, 1, []),
        (pclk_s_edp0_en, 14, 1, []),
        (pclk_s_edp1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON60  0x08F0;
generate_register_bitfields!(
    CRU_GATE_CON60,
    u32,
    [
        (hclk_i2s7_8ch_en, 0, 1, []),
        (clk_i2s7_8ch_rx_en, 1, 1, []),
        (clk_i2s7_8ch_frac_rx_en, 2, 1, []),
        (mclk_i2s7_8ch_rx_en, 3, 1, []),
        (hclk_hdcp_key1_en, 4, 1, []),
        (aclk_hdcp1_en, 5, 1, []),
        (hclk_hdcp1_en, 6, 1, []),
        (pclk_hdcp1_en, 7, 1, []),
        (reserved0, 8, 1, []),
        (aclk_trng1_en, 9, 1, []),
        (pclk_trng1_en, 10, 1, []),
        (pclk_hdmitx0_en, 11, 1, []),
        (reserved1, 12, 3, []),
        (clk_hdmitx0_earc_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON61  0x08F4;
generate_register_bitfields!(
    CRU_GATE_CON61,
    u32,
    [
        (clk_hdmitx0_ref_en, 0, 1, []),
        (reserved0, 1, 1, []),
        (pclk_hdmitx1_en, 2, 1, []),
        (reserved1, 3, 3, []),
        (clk_hdmitx1_earc_en, 6, 1, []),
        (clk_hdmitx1_ref_en, 6, 1, []),
        (reserved2, 8, 1, []),
        (aclk_hdmirx_en, 9, 1, []),
        (pclk_hdmirx_en, 10, 1, []),
        (clk_hdmirx_ref_en, 11, 1, []),
        (clk_hdmirx_aud_src_en, 12, 1, []),
        (clk_hdmirx_aud_frac_en, 13, 1, []),
        (clk_hdmirx_aud_en, 14, 1, []),
        (clk_hdmirx_tmdsqp_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON62  0x08F8;
generate_register_bitfields!(
    CRU_GATE_CON62,
    u32,
    [
        (pclk_edp0_en, 0, 1, []),
        (clk_edp0_24m_en, 1, 1, []),
        (clk_edp0_200m_en, 2, 1, []),
        (pclk_edp1_en, 3, 1, []),
        (clk_edp1_24m_en, 4, 1, []),
        (clk_edp1_200m_en, 5, 1, []),
        (clk_i2s5_8ch_tx_en, 6, 1, []),
        (clk_i2s5_8ch_frac_tx_en, 7, 1, []),
        (mclk_i2s5_8ch_tx_en, 8, 1, []),
        (reserved, 9, 3, []),
        (hclk_i2s5_8ch_en, 12, 1, []),
        (clk_i2s6_8ch_tx_en, 13, 1, []),
        (clk_i2s6_8ch_frac_tx_en, 14, 1, []),
        (mclk_i2s6_8ch_tx_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON63 0x08FC;
generate_register_bitfields!(
    CRU_GATE_CON63,
    u32,
    [
        (clk_i2s6_8ch_rx_en, 0, 1, []),
        (clk_i2s6_8ch_frac_rx_en, 1, 1, []),
        (mclk_i2s6_8ch_rx_en, 2, 1, []),
        (hclk_i2s6_8ch_en, 3, 1, []),
        (hclk_spdif3_en, 4, 1, []),
        (clk_spdif3_en, 5, 1, []),
        (clk_spdif3_frac_en, 6, 1, []),
        (mclk_spdif3_en, 7, 1, []),
        (hclk_spdif4_en, 8, 1, []),
        (clk_spdif4_en, 9, 1, []),
        (clk_spdif4_frac_en, 10, 1, []),
        (mclk_spdif4_en, 11, 1, []),
        (hclk_spdifrx0_en, 12, 1, []),
        (mclk_spdifrx0_en, 13, 1, []),
        (hclk_spdifrx1_en, 14, 1, []),
        (mclk_spdifrx1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON64 0x0900;
generate_register_bitfields!(
    CRU_GATE_CON64,
    u32,
    [
        (hclk_spdifrx2_en, 0, 1, []),
        (mclk_spdifrx2_en, 1, 1, []),
        (reserved, 2, 12, []),
        (dclk_vp2hdmi_bridge0_vo1_en, 14, 1, []),
        (dclk_vp2hdmi_bridge1_vo1_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON65 0x0904;
generate_register_bitfields!(
    CRU_GATE_CON65,
    u32,
    [
        (hclk_i2s9_8ch_en, 0, 1, []),
        (clk_i2s9_8ch_rx_en, 1, 1, []),
        (clk_i2s9_8ch_frac_rx_en, 2, 1, []),
        (mclk_i2s9_8ch_rx_en, 3, 1, []),
        (hclk_i2s10_8ch_en, 4, 1, []),
        (clk_i2s10_8ch_rx_en, 5, 1, []),
        (clk_i2s10_8ch_frac_rx_en, 6, 1, []),
        (mclk_i2s10_8ch_rx_en, 7, 1, []),
        (pclk_s_hdmirx_en, 8, 1, []),
        (clk_hdmitrx_refsrc_en, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON66 0x0908;
generate_register_bitfields!(
    CRU_GATE_CON66,
    u32,
    [
        (reserved0, 0, 1, []),
        (clk_gpu_src_df_en, 1, 1, []),
        (clk_testout_gpu_en, 2, 1, []),
        (clk_gpu_src_en, 3, 1, []),
        (clk_gpu_en, 4, 1, []),
        (reserved1, 5, 1, []),
        (clk_gpu_coregroup_en, 6, 1, []),
        (clk_gpu_stacks_en, 7, 1, []),
        (aclk_s_gpu_biu_en, 8, 1, []),
        (aclk_m0_gpu_biu_en, 9, 1, []),
        (aclk_m1_gpu_biu_en, 10, 1, []),
        (aclk_m2_gpu_biu_en, 11, 1, []),
        (aclk_m3_gpu_biu_en, 12, 1, []),
        (pclk_gpu_root_en, 13, 1, []),
        (pclk_gpu_biu_en, 14, 1, []),
        (pclk_pvtm2_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON67 0x090C;
generate_register_bitfields!(
    CRU_GATE_CON67,
    u32,
    [
        (clk_pvtm2_en, 0, 1, []),
        (clk_gpu_pvtm_en, 1, 1, []),
        (pclk_gpu_grf_en, 2, 1, []),
        (clk_gpu_pvtpll_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON68 0x0910;
generate_register_bitfields!(
    CRU_GATE_CON68,
    u32,
    [
        (aclk_av1_root_en, 0, 1, []),
        (aclk_av1_biu_en, 1, 1, []),
        (aclk_av1_en, 2, 1, []),
        (pclk_av1_root_en, 3, 1, []),
        (pclk_av1_biu_en, 4, 1, []),
        (pclk_av1_en, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON69 0x0914;
generate_register_bitfields!(
    CRU_GATE_CON69,
    u32,
    [
        (aclk_center_root_en, 0, 1, []),
        (aclk_center_low_root_en, 1, 1, []),
        (hclk_center_root_en, 2, 1, []),
        (pclk_center_root_en, 3, 1, []),
        (aclk_ddr_biu_en, 4, 1, []),
        (aclk_dma2ddr_en, 5, 1, []),
        (aclk_ddr_sharemem_en, 6, 1, []),
        (aclk_ddr_sharemem_biu_en, 7, 1, []),
        (aclk_center_s200_root_en, 8, 1, []),
        (aclk_center_s400_root_en, 9, 1, []),
        (aclk_center_s200_biu_en, 10, 1, []),
        (aclk_center_s400_biu_en, 11, 1, []),
        (hclk_ahb2apb_en, 12, 1, []),
        (hclk_center_biu_en, 13, 1, []),
        (fclk_ddr_cm0_core_en, 14, 1, []),
        (clk_ddr_timer_root_en, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON70 0x0918;
generate_register_bitfields!(
    CRU_GATE_CON70,
    u32,
    [
        (clk_ddr_timer0_en, 0, 1, []),
        (clk_ddr_timer1_en, 1, 1, []),
        (tclk_wdt_ddr_en, 2, 1, []),
        (reserved0, 3, 1, []),
        (clk_ddr_cm0_rtc_en, 4, 1, []),
        (pclk_center_grf_en, 5, 1, []),
        (pclk_ahb2apb_en, 6, 1, []),
        (pclk_wdt_en, 7, 1, []),
        (pclk_timer_en, 8, 1, []),
        (pclk_dma2ddr_en, 9, 1, []),
        (pclk_sharemem_en, 10, 1, []),
        (pclk_center_biu_en, 11, 1, []),
        (pclk_center_channel_biu_en, 12, 1, []),
        (reserved1, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON72 0x0920;
generate_register_bitfields!(
    CRU_GATE_CON72,
    u32,
    [
        (reserved0, 0, 1, []),
        (pclk_usbdpgrf0_en, 1, 1, []),
        (pclk_usbdpphy0_en, 2, 1, []),
        (pclk_usbdpgrf1_en, 3, 1, []),
        (pclk_usbdpphy1_en, 4, 1, []),
        (pclk_hdptx0_en, 5, 1, []),
        (pclk_hdptx1_en, 6, 1, []),
        (pclk_apb2asb_slv_bot_right_en, 7, 1, []),
        (pclk_usb2phy_u3_0_grf0_en, 8, 1, []),
        (pclk_usb2phy_u3_1_grf0_en, 9, 1, []),
        (pclk_usb2phy_u2_0_grf0_en, 10, 1, []),
        (pclk_usb2phy_u2_1_grf0_en, 11, 1, []),
        (reserved1, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON73 0x0924;
generate_register_bitfields!(
    CRU_GATE_CON73,
    u32,
    [
        (reserved0, 0, 12, []),
        (clk_hdmihdp0_en, 12, 1, []),
        (clk_hdmihdp1_en, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON74 0x0928;
generate_register_bitfields!(
    CRU_GATE_CON74,
    u32,
    [
        (aclk_vo1usb_top_root_en, 0, 1, []),
        (aclk_vo1usb_top_biu_en, 1, 1, []),
        (hclk_vo1usb_top_root_en, 2, 1, []),
        (hclk_vo1usb_top_biu_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON75 0x092C;
generate_register_bitfields!(
    CRU_GATE_CON75,
    u32,
    [
        (hclk_sdio_root_en, 0, 1, []),
        (hclk_sdio_biu_en, 1, 1, []),
        (hclk_sdio_en, 2, 1, []),
        (cclk_src_sdio_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON76 0x0930;
generate_register_bitfields!(
    CRU_GATE_CON76,
    u32,
    [
        (aclk_rga3_root_en, 0, 1, []),
        (hclk_rga3_root_en, 1, 1, []),
        (hclk_rga3_biu_en, 2, 1, []),
        (aclk_rga3_biu_en, 3, 1, []),
        (hclk_rga3_1_en, 4, 1, []),
        (aclk_rga3_1_en, 5, 1, []),
        (clk_rga3_1_core_en, 6, 1, []),
        (reserved, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_GATE_CON77 0x0934;
generate_register_bitfields!(
    CRU_GATE_CON77,
    u32,
    [
        (clk_ref_pipe_phy0_osc_src_en, 0, 1, []),
        (clk_ref_pipe_phy1_osc_src_en, 1, 1, []),
        (clk_ref_pipe_phy2_osc_src_en, 2, 1, []),
        (clk_ref_pipe_phy0_pll_src_en, 3, 1, []),
        (clk_ref_pipe_phy1_pll_src_en, 4, 1, []),
        (clk_ref_pipe_phy2_pll_src_en, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);
