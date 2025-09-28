use crate::generate_register_bitfields;
use tock_registers::{register_bitfields, registers::ReadWrite};

#[repr(C)]
pub struct ModeRegisters {
    pub mode_con00: ReadWrite<u32, CRU_MODE_CON00::Register>,
    _reserved: [u32; 31],
}

// CRU_MODE_CON00  0x0280;
generate_register_bitfields!(
    CRU_MODE_CON00,
    u32,
    [
        (clk_npll_mode, 0, 2, []),
        (clk_gpll_mode, 2, 2, []),
        (clk_v0pll_mode, 4, 2, []),
        (clk_aupll_mode, 6, 2, []),
        (clk_cpll_mode, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// const CRU_GLB_CNT_TH: u32 = 0x0C00;
generate_register_bitfields!(
    CRU_GLB_CNT_TH,
    u32,
    [
        (global_reset_counter_threshold, 0, 10, []),
        (reserved, 10, 22, []),
    ]
);

// const CRU_GLBRST_ST: u32 = 0x0C04;
generate_register_bitfields!(
    CRU_GLBRST_ST,
    u32,
    [
        (first_glbrst_register_rst, 0, 1, []),
        (second_glbrst_register_rst, 1, 1, []),
        (first_glbrst_tsadc_rst, 2, 1, []),
        (second_glbrst_tsadc_rst, 3, 1, []),
        (first_glbrst_wdt_rst, 4, 1, []),
        (second_glbrst_wdt_rst, 5, 1, []),
        (glbrst_wdt_rst, 6, 1, []),
        (glbrst_osc_chk_rst, 7, 1, []),
        (glbrst_pmusgrf_crc_chk_rst, 8, 1, []),
        (glbrst_dsusgrf_crc_chk_rst, 9, 1, []),
        (glbrst_sgrf_crc_chk_rst, 10, 1, []),
        (glbrst_wdt0_rst, 11, 1, []),
        (glbrst_wdt1_rst, 12, 1, []),
        (glbrst_wdt2_rst, 13, 1, []),
        (glbrst_wdt3_rst, 14, 1, []),
        (glbrst_wdt4_rst, 15, 1, []),
        (reserved, 16, 16, []),
    ]
);

// const CRU_GLB_SRST_FST_VALUE: u32 = 0x0C08;
generate_register_bitfields!(
    CRU_GLB_SRST_FST_VALUE,
    u32,
    [(glb_srsc_first_value, 0, 16, []), (reserved, 16, 16, []),]
);

// CRU_GLB_SRST_SND_VALUE  0x0C0C
generate_register_bitfields!(
    CRU_GLB_SRST_SND_VALUE,
    u32,
    [(glb_srsc_second_value, 0, 16, []), (reserved, 16, 16, []),]
);

// CRU_GLB_RST_CON  0x0C10
generate_register_bitfields!(
    CRU_GLB_RST_CON,
    u32,
    [
        (tsadc_trig_glbrst_sel, 0, 1, []),
        (tsadc_trig_glbrst_en, 1, 1, []),
        (glbrst_trig_pmu_sel, 2, 1, []),
        (glbrst_trig_pmu_en, 3, 1, []),
        (wdt_trig_pmu_en, 4, 1, []),
        (reserved0, 5, 1, []),
        (wdt_trig_glbrst_en, 6, 1, []),
        (osc_chk_trig_glbrst_en, 7, 1, []),
        (crc_pmusgrf_chk_trig_glbrst_en, 8, 1, []),
        (crc_dsusgrf_chk_trig_glbrst_en, 9, 1, []),
        (crc_sgrf_chk_trig_glbrst_en, 10, 1, []),
        (wdt_trig_glbrst_sel, 11, 1, []),
        (osc_chk_trig_glbrst_sel, 12, 1, []),
        (crc_pmusgrf_chk_trig_glbrst_sel, 13, 1, []),
        (crc_dsusgrf_chk_trig_glbrst_sel, 14, 1, []),
        (crc_sgrf_chk_trig_glbrst_sel, 15, 1, []),
        (reserved1, 16, 16, []),
    ]
);

// CRU_SDIO_CON0  0x0C24
generate_register_bitfields!(
    CRU_SDIO_CON0,
    u32,
    [(sdio_con0, 0, 16, []), (write_enable, 16, 16, []),]
);

// CRU_SDIO_CON1  0x0C28
generate_register_bitfields!(
    CRU_SDIO_CON1,
    u32,
    [(sdio_con1, 0, 16, []), (write_enable, 16, 16, []),]
);

// CRU_SDMMC_CON0  0x0C30
generate_register_bitfields!(
    CRU_SDMMC_CON0,
    u32,
    [(sdmmc_con0, 0, 16, []), (write_enable, 16, 16, []),]
);

// CRU_SDMMC_CON1  0x0C34
generate_register_bitfields!(
    CRU_SDMMC_CON1,
    u32,
    [(sdmmc_con1, 0, 16, []), (write_enable, 16, 16, []),]
);

// CRU_PHYREF_ALT_GATE_CON  0x0C38
generate_register_bitfields!(
    CRU_PHYREF_ALT_GATE_CON,
    u32,
    [
        (phy0_ref_alt_clk_p_en, 0, 1, []),
        (phy0_ref_alt_clk_m_en, 1, 1, []),
        (phy1_ref_alt_clk_p_en, 2, 1, []),
        (phy1_ref_alt_clk_m_en, 3, 1, []),
        (reserved, 4, 12, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CM0_GATEMASK_CON  0x0C3C
generate_register_bitfields!(
    CRU_CM0_GATEMASK_CON,
    u32,
    [
        (npucm0_dclk_cm0s_en, 0, 1, []),
        (npucm0_hclk_cm0s_en, 1, 1, []),
        (npucm0_sclk_cm0s_en, 2, 1, []),
        (ddrcm0_dclk_cm0s_en, 3, 1, []),
        (ddrcm0_hclk_cm0s_en, 4, 1, []),
        (ddrcm0_sclk_cm0s_en, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_QCHANNEL_CON01  0x0CA4
generate_register_bitfields!(
    CRU_QCHANNEL_CON01,
    u32,
    [
        (aclk_gic_qc_en, 0, 1, []),
        (aclk_gic_qc_gate_en, 1, 1, []),
        (aclk_gicadb_gic2core_bus_qc_en, 2, 1, []),
        (aclk_gicadb_gic2core_bus_qc_gate_en, 3, 1, []),
        (aclk_php_gic_its_qc_en, 4, 1, []),
        (aclk_php_gic_its_qc_gate_en, 5, 12, []),
        (clk_gpu_qc_en, 6, 1, []),
        (clk_gpu_qc_gate_en, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_SMOTH_DIVFREE_CON08  0x0CC0
generate_register_bitfields!(
    CRU_SMOTH_DIVFREE_CON08,
    u32,
    [
        (aclk_m0_gpu_step, 0, 5, []),
        (reserved, 5, 8, []),
        (aclk_m0_gpu_smdiv_clk_off, 13, 1, []),
        (aclk_m0_gpu_gate_smth_en, 14, 1, []),
        (aclk_m0_gpu_bypass, 15, 1, []),
        (aclk_m0_gpu_freq_keep, 16, 16, []),
    ]
);

// CRU_SMOTH_DIVFREE_CON09  0x0CC4
generate_register_bitfields!(
    CRU_SMOTH_DIVFREE_CON09,
    u32,
    [
        (aclk_m1_gpu_step, 0, 5, []),
        (reserved, 5, 8, []),
        (aclk_m1_gpu_smdiv_clk_off, 13, 1, []),
        (aclk_m1_gpu_gate_smth_en, 14, 1, []),
        (aclk_m1_gpu_bypass, 15, 1, []),
        (aclk_m1_gpu_freq_keep, 16, 16, []),
    ]
);

// CRU_SMOTH_DIVFREE_CON10  0x0CC8
generate_register_bitfields!(
    CRU_SMOTH_DIVFREE_CON10,
    u32,
    [
        (aclk_m2_gpu_step, 0, 5, []),
        (reserved, 5, 8, []),
        (aclk_m2_gpu_smdiv_clk_off, 13, 1, []),
        (aclk_m2_gpu_gate_smth_en, 14, 1, []),
        (aclk_m2_gpu_bypass, 15, 1, []),
        (aclk_m2_gpu_freq_keep, 16, 16, []),
    ]
);

// CRU_SMOTH_DIVFREE_CON11  0x0CCC
generate_register_bitfields!(
    CRU_SMOTH_DIVFREE_CON11,
    u32,
    [
        (aclk_m3_gpu_step, 0, 5, []),
        (reserved, 5, 8, []),
        (aclk_m3_gpu_smdiv_clk_off, 13, 1, []),
        (aclk_m3_gpu_gate_smth_en, 14, 1, []),
        (aclk_m3_gpu_bypass, 15, 1, []),
        (aclk_m3_gpu_freq_keep, 16, 16, []),
    ]
);

// CRU_SMOTH_DIVFREE_CON12  0x0CD0
generate_register_bitfields!(
    CRU_SMOTH_DIVFREE_CON12,
    u32,
    [
        (clk_rknn_dsu0_src_step, 0, 5, []),
        (reserved, 5, 8, []),
        (clk_rknn_dsu0_src_smdiv_clk_off, 13, 1, []),
        (clk_rknn_dsu0_src_gate_smth_en, 14, 1, []),
        (clk_rknn_dsu0_src_bypass, 15, 1, []),
        (clk_rknn_dsu0_src_freq_keep, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_ROOT_CON0  0x0D00
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_ROOT_CON0,
    u32,
    [
        (aclk_top_root_idle_th, 0, 16, []),
        (aclk_top_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_ROOT_CON1  0x0D04
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_ROOT_CON1,
    u32,
    [
        (aclk_top_root_autocs_ctrl, 0, 12, []),
        (aclk_top_root_autocs_en, 12, 1, []),
        (aclk_top_root_switch_en, 13, 1, []),
        (aclk_top_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_LOW_TOP_ROOT_CON0  0x0D08
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_LOW_TOP_ROOT_CON0,
    u32,
    [
        (aclk_low_top_root_idle_th, 0, 16, []),
        (aclk_low_top_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_LOW_TOP_ROOT_CON1  0x0D0C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_LOW_TOP_ROOT_CON1,
    u32,
    [
        (aclk_low_top_root_autocs_ctrl, 0, 12, []),
        (aclk_low_top_root_autocs_en, 12, 1, []),
        (aclk_low_top_root_switch_en, 13, 1, []),
        (aclk_low_top_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M400_ROOT_CON0  0x0D10
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M400_ROOT_CON0,
    u32,
    [
        (aclk_top_m400_root_idle_th, 0, 16, []),
        (aclk_top_m400_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M400_ROOT_CON1  0x0D14
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M400_ROOT_CON1,
    u32,
    [
        (aclk_top_m400_root_autocs_ctrl, 0, 12, []),
        (aclk_top_m400_root_autocs_en, 12, 1, []),
        (aclk_top_m400_root_switch_en, 13, 1, []),
        (aclk_top_m400_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_S400_ROOT_CON0  0x0D18
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_S400_ROOT_CON0,
    u32,
    [
        (aclk_top_s400_root_idle_th, 0, 16, []),
        (aclk_top_s400_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_S400_ROOT_CON1  0x0D1C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_S400_ROOT_CON1,
    u32,
    [
        (aclk_top_s400_root_autocs_ctrl, 0, 12, []),
        (aclk_top_s400_root_autocs_en, 12, 1, []),
        (aclk_top_s400_root_switch_en, 13, 1, []),
        (aclk_top_s400_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_BUS_ROOT_CON0  0x0D20
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_BUS_ROOT_CON0,
    u32,
    [
        (aclk_bus_root_idle_th, 0, 16, []),
        (aclk_bus_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_BUS_ROOT_CON1  0x0D24
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_BUS_ROOT_CON1,
    u32,
    [
        (aclk_bus_root_autocs_ctrl, 0, 12, []),
        (aclk_bus_root_autocs_en, 12, 1, []),
        (aclk_bus_root_switch_en, 13, 1, []),
        (aclk_bus_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_ISP1_ROOT_CON0  0x0D28
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_ISP1_ROOT_CON0,
    u32,
    [
        (aclk_isp1_root_idle_th, 0, 16, []),
        (aclk_isp1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_ISP1_ROOT_CON1  0x0D2C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_ISP1_ROOT_CON1,
    u32,
    [
        (aclk_isp1_root_autocs_ctrl, 0, 12, []),
        (aclk_isp1_root_autocs_en, 12, 1, []),
        (aclk_isp1_root_switch_en, 13, 1, []),
        (aclk_isp1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_RKNN_DSU0_CON0  0x0D30
generate_register_bitfields!(
    CRU_AUTOCS_CLK_RKNN_DSU0_CON0,
    u32,
    [
        (clk_rknn_dsu0_idle_th, 0, 16, []),
        (clk_rknn_dsu0_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_RKNN_DSU0_CON1  0x0D34
generate_register_bitfields!(
    CRU_AUTOCS_CLK_RKNN_DSU0_CON1,
    u32,
    [
        (clk_rknn_dsu0_autocs_ctrl, 0, 12, []),
        (clk_rknn_dsu0_autocs_en, 12, 1, []),
        (clk_rknn_dsu0_switch_en, 13, 1, []),
        (clk_rknn_dsu0_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKNN_ROOT_CON0  0x0D38
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKNN_ROOT_CON0,
    u32,
    [
        (hclk_rknn_root_idle_th, 0, 16, []),
        (hclk_rknn_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKNN_ROOT_CON1  0x0D3C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKNN_ROOT_CON1,
    u32,
    [
        (hclk_rknn_root_autocs_ctrl, 0, 12, []),
        (hclk_rknn_root_autocs_en, 12, 1, []),
        (hclk_rknn_root_switch_en, 13, 1, []),
        (hclk_rknn_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_NVM_ROOT_CON0  0x0D40
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_NVM_ROOT_CON0,
    u32,
    [
        (aclk_nvm_root_idle_th, 0, 16, []),
        (aclk_nvm_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_NVM_ROOT_CON1  0x0D44
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_NVM_ROOT_CON1,
    u32,
    [
        (aclk_nvm_root_autocs_ctrl, 0, 12, []),
        (aclk_nvm_root_autocs_en, 12, 1, []),
        (aclk_nvm_root_switch_en, 13, 1, []),
        (aclk_nvm_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_PHP_ROOT_CON0  0x0D48
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_PHP_ROOT_CON0,
    u32,
    [
        (aclk_php_root_idle_th, 0, 16, []),
        (aclk_php_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_PHP_ROOT_CON1  0x0D4C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_PHP_ROOT_CON1,
    u32,
    [
        (aclk_php_root_autocs_ctrl, 0, 12, []),
        (aclk_php_root_autocs_en, 12, 1, []),
        (aclk_php_root_switch_en, 13, 1, []),
        (aclk_php_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC0_ROOT_CON0  0x0D50
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC0_ROOT_CON0,
    u32,
    [
        (aclk_rkvdec0_root_idle_th, 0, 16, []),
        (aclk_rkvdec0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC0_ROOT_CON1  0x0D54
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC0_ROOT_CON1,
    u32,
    [
        (aclk_rkvdec0_root_autocs_ctrl, 0, 12, []),
        (aclk_rkvdec0_root_autocs_en, 12, 1, []),
        (aclk_rkvdec0_root_switch_en, 13, 1, []),
        (aclk_rkvdec0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC_CCU_CON0  0x0D58
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC_CCU_CON0,
    u32,
    [
        (aclk_rkvdec_ccu_idle_th, 0, 16, []),
        (aclk_rkvdec_ccu_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC_CCU_CON1  0x0D5C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC_CCU_CON1,
    u32,
    [
        (aclk_rkvdec_ccu_autocs_ctrl, 0, 12, []),
        (aclk_rkvdec_ccu_autocs_en, 12, 1, []),
        (aclk_rkvdec_ccu_switch_en, 13, 1, []),
        (aclk_rkvdec_ccu_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC1_ROOT_CON0  0x0D60
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC1_ROOT_CON0,
    u32,
    [
        (aclk_rkvdec1_root_idle_th, 0, 16, []),
        (aclk_rkvdec1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVDEC1_ROOT_CON1  0x0D64
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVDEC1_ROOT_CON1,
    u32,
    [
        (aclk_rkvdec1_root_autocs_ctrl, 0, 12, []),
        (aclk_rkvdec1_root_autocs_en, 12, 1, []),
        (aclk_rkvdec1_root_switch_en, 13, 1, []),
        (aclk_rkvdec1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_USB_ROOT_CON0  0x0D68
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_USB_ROOT_CON0,
    u32,
    [
        (aclk_usb_root_idle_th, 0, 16, []),
        (aclk_usb_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_USB_ROOT_CON1  0x0D6C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_USB_ROOT_CON1,
    u32,
    [
        (aclk_usb_root_autocs_ctrl, 0, 12, []),
        (aclk_usb_root_autocs_en, 12, 1, []),
        (aclk_usb_root_switch_en, 13, 1, []),
        (aclk_usb_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VDPU_ROOT_CON0  0x0D70
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VDPU_ROOT_CON0,
    u32,
    [
        (aclk_vdpu_root_idle_th, 0, 16, []),
        (aclk_vdpu_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VDPU_ROOT_CON1  0x0D74
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VDPU_ROOT_CON1,
    u32,
    [
        (aclk_vdpu_root_autocs_ctrl, 0, 12, []),
        (aclk_vdpu_root_autocs_en, 12, 1, []),
        (aclk_vdpu_root_switch_en, 13, 1, []),
        (aclk_vdpu_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VDPU_LOW_ROOT_CON0  0x0D78
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VDPU_LOW_ROOT_CON0,
    u32,
    [
        (aclk_vdpu_low_root_idle_th, 0, 16, []),
        (aclk_vdpu_low_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VDPU_LOW_ROOT_CON1  0x0D7C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VDPU_LOW_ROOT_CON1,
    u32,
    [
        (aclk_vdpu_low_root_autocs_ctrl, 0, 12, []),
        (aclk_vdpu_low_root_autocs_en, 12, 1, []),
        (aclk_vdpu_low_root_switch_en, 13, 1, []),
        (aclk_vdpu_low_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_JPEG_DECODER_ROOT_CON0  0x0D80
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_JPEG_DECODER_ROOT_CON0,
    u32,
    [
        (aclk_jpeg_decoder_root_idle_th, 0, 16, []),
        (aclk_jpeg_decoder_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_JPEG_DECODER_ROOT_CON1  0x0D84
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_JPEG_DECODER_ROOT_CON1,
    u32,
    [
        (aclk_jpeg_decoder_root_autocs_ctrl, 0, 12, []),
        (aclk_jpeg_decoder_root_autocs_en, 12, 1, []),
        (aclk_jpeg_decoder_root_switch_en, 13, 1, []),
        (aclk_jpeg_decoder_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVENC0_ROOT_CON0  0x0D88
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVENC0_ROOT_CON0,
    u32,
    [
        (aclk_rkvenc0_root_idle_th, 0, 16, []),
        (aclk_rkvenc0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVENC0_ROOT_CON1  0x0D8C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVENC0_ROOT_CON1,
    u32,
    [
        (aclk_rkvenc0_root_autocs_ctrl, 0, 12, []),
        (aclk_rkvenc0_root_autocs_en, 12, 1, []),
        (aclk_rkvenc0_root_switch_en, 13, 1, []),
        (aclk_rkvenc0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVENC1_ROOT_CON0  0x0D90
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVENC1_ROOT_CON0,
    u32,
    [
        (aclk_rkvenc1_root_idle_th, 0, 16, []),
        (aclk_rkvenc1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RKVENC1_ROOT_CON1  0x0D94
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RKVENC1_ROOT_CON1,
    u32,
    [
        (aclk_rkvenc1_root_autocs_ctrl, 0, 12, []),
        (aclk_rkvenc1_root_autocs_en, 12, 1, []),
        (aclk_rkvenc1_root_switch_en, 13, 1, []),
        (aclk_rkvenc1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VI_ROOT_CON0  0x0D98
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VI_ROOT_CON0,
    u32,
    [
        (aclk_vi_root_idle_th, 0, 16, []),
        (aclk_vi_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VI_ROOT_CON1  0x0D9C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VI_ROOT_CON1,
    u32,
    [
        (aclk_vi_root_autocs_ctrl, 0, 12, []),
        (aclk_vi_root_autocs_en, 12, 1, []),
        (aclk_vi_root_switch_en, 13, 1, []),
        (aclk_vi_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VOP_ROOT_CON0  0x0DA0
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VOP_ROOT_CON0,
    u32,
    [
        (aclk_vop_root_idle_th, 0, 16, []),
        (aclk_vop_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VOP_ROOT_CON1  0x0DA4
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VOP_ROOT_CON1,
    u32,
    [
        (aclk_vop_root_autocs_ctrl, 0, 12, []),
        (aclk_vop_root_autocs_en, 12, 1, []),
        (aclk_vop_root_switch_en, 13, 1, []),
        (aclk_vop_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VO0_ROOT_CON0  0x0DA8
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VO0_ROOT_CON0,
    u32,
    [
        (aclk_vo0_root_idle_th, 0, 16, []),
        (aclk_vo0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VO0_ROOT_CON1  0x0DAC
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VO0_ROOT_CON1,
    u32,
    [
        (aclk_vo0_root_autocs_ctrl, 0, 12, []),
        (aclk_vo0_root_autocs_en, 12, 1, []),
        (aclk_vo0_root_switch_en, 13, 1, []),
        (aclk_vo0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_HDCP1_ROOT_CON0  0x0DB0
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_HDCP1_ROOT_CON0,
    u32,
    [
        (aclk_hdcp1_root_idle_th, 0, 16, []),
        (aclk_hdcp1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_HDCP1_ROOT_CON1  0x0DB4
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_HDCP1_ROOT_CON1,
    u32,
    [
        (aclk_hdcp1_root_autocs_ctrl, 0, 12, []),
        (aclk_hdcp1_root_autocs_en, 12, 1, []),
        (aclk_hdcp1_root_switch_en, 13, 1, []),
        (aclk_hdcp1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_HDMIRX_ROOT_CON0  0x0DB8
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_HDMIRX_ROOT_CON0,
    u32,
    [
        (aclk_hdmirx_root_idle_th, 0, 16, []),
        (aclk_hdmirx_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_HDMIRX_ROOT_CON1  0x0DBC
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_HDMIRX_ROOT_CON1,
    u32,
    [
        (aclk_hdmirx_root_autocs_ctrl, 0, 12, []),
        (aclk_hdmirx_root_autocs_en, 12, 1, []),
        (aclk_hdmirx_root_switch_en, 13, 1, []),
        (aclk_hdmirx_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_GPU_COREGROUP_CON0  0x0DC0
generate_register_bitfields!(
    CRU_AUTOCS_CLK_GPU_COREGROUP_CON0,
    u32,
    [
        (clk_gpu_coregroup_idle_th, 0, 16, []),
        (clk_gpu_coregroup_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_GPU_COREGROUP_CON1  0x0DC4
generate_register_bitfields!(
    CRU_AUTOCS_CLK_GPU_COREGROUP_CON1,
    u32,
    [
        (clk_gpu_coregroup_autocs_ctrl, 0, 12, []),
        (clk_gpu_coregroup_autocs_en, 12, 1, []),
        (clk_gpu_coregroup_switch_en, 13, 1, []),
        (clk_gpu_coregroup_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_AV1_ROOT_CON0  0x0DE0
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_AV1_ROOT_CON0,
    u32,
    [
        (aclk_av1_root_idle_th, 0, 16, []),
        (aclk_av1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_AV1_ROOT_CON1  0x0DE4
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_AV1_ROOT_CON1,
    u32,
    [
        (aclk_av1_root_autocs_ctrl, 0, 12, []),
        (aclk_av1_root_autocs_en, 12, 1, []),
        (aclk_av1_root_switch_en, 13, 1, []),
        (aclk_av1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_ROOT_CON0  0x0DE8
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_ROOT_CON0,
    u32,
    [
        (aclk_center_root_idle_th, 0, 16, []),
        (aclk_center_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_ROOT_CON1  0x0DEC
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_ROOT_CON1,
    u32,
    [
        (aclk_center_root_autocs_ctrl, 0, 12, []),
        (aclk_center_root_autocs_en, 12, 1, []),
        (aclk_center_root_switch_en, 13, 1, []),
        (aclk_center_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_LOW_ROOT_CON0  0x0DF0
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_LOW_ROOT_CON0,
    u32,
    [
        (aclk_center_low_root_idle_th, 0, 16, []),
        (aclk_center_low_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_LOW_ROOT_CON1  0x0DF4
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_LOW_ROOT_CON1,
    u32,
    [
        (aclk_center_low_root_autocs_ctrl, 0, 12, []),
        (aclk_center_low_root_autocs_en, 12, 1, []),
        (aclk_center_low_root_switch_en, 13, 1, []),
        (aclk_center_low_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_S400_ROOT_CON0  0x0DF8
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_S400_ROOT_CON0,
    u32,
    [
        (aclk_center_s400_root_idle_th, 0, 16, []),
        (aclk_center_s400_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_S400_ROOT_CON1  0x0DFC
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_S400_ROOT_CON1,
    u32,
    [
        (aclk_center_s400_root_autocs_ctrl, 0, 12, []),
        (aclk_center_s400_root_autocs_en, 12, 1, []),
        (aclk_center_s400_root_switch_en, 13, 1, []),
        (aclk_center_s400_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VO1USB_TOP_ROOT_CON0  0x0E00
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VO1USB_TOP_ROOT_CON0,
    u32,
    [
        (aclk_vo1usb_top_root_idle_th, 0, 16, []),
        (aclk_vo1usb_top_root_wait_th, 16, 16, []),
    ]
);
// CRU_AUTOCS_ACLK_VO1USB_TOP_ROOT_CON1  0x0E04
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VO1USB_TOP_ROOT_CON1,
    u32,
    [
        (aclk_vo1usb_top_root_autocs_ctrl, 0, 12, []),
        (aclk_vo1usb_top_root_autocs_en, 12, 1, []),
        (aclk_vo1usb_top_root_switch_en, 13, 1, []),
        (aclk_vo1usb_top_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RGA3_ROOT_CON0  0x0E08
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RGA3_ROOT_CON0,
    u32,
    [
        (aclk_rga3_root_idle_th, 0, 16, []),
        (aclk_rga3_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_RGA3_ROOT_CON1  0x0E0C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_RGA3_ROOT_CON1,
    u32,
    [
        (aclk_rga3_root_autocs_ctrl, 0, 12, []),
        (aclk_rga3_root_autocs_en, 12, 1, []),
        (aclk_rga3_root_switch_en, 13, 1, []),
        (aclk_rga3_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_AV1_ROOT_CON0  0x0E10
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_AV1_ROOT_CON0,
    u32,
    [
        (pclk_av1_root_idle_th, 0, 16, []),
        (pclk_av1_root_wait_th, 16, 16, []),
    ]
);
// CRU_AUTOCS_PCLK_AV1_ROOT_CON1  0x0E14
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_AV1_ROOT_CON1,
    u32,
    [
        (pclk_av1_root_autocs_ctrl, 0, 12, []),
        (pclk_av1_root_autocs_en, 12, 1, []),
        (pclk_av1_root_switch_en, 13, 1, []),
        (pclk_av1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_ISP1_ROOT_CON0  0x0E18
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_ISP1_ROOT_CON0,
    u32,
    [
        (hclk_isp1_root_idle_th, 0, 16, []),
        (hclk_isp1_root_wait_th, 16, 16, []),
    ]
);
// CRU_AUTOCS_HCLK_ISP1_ROOT_CON1  0x0E1C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_ISP1_ROOT_CON1,
    u32,
    [
        (hclk_isp1_root_autocs_ctrl, 0, 12, []),
        (hclk_isp1_root_autocs_en, 12, 1, []),
        (hclk_isp1_root_switch_en, 13, 1, []),
        (hclk_isp1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_NPUTOP_ROOT_CON0  0x0E20
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_NPUTOP_ROOT_CON0,
    u32,
    [
        (pclk_nputop_root_idle_th, 0, 16, []),
        (pclk_nputop_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_NPUTOP_ROOT_CON1  0x0E24
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_NPUTOP_ROOT_CON1,
    u32,
    [
        (pclk_nputop_root_autocs_ctrl, 0, 12, []),
        (pclk_nputop_root_autocs_en, 12, 1, []),
        (pclk_nputop_root_switch_en, 13, 1, []),
        (pclk_nputop_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_NPU_CM0_ROOT_CON0  0x0E28
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_NPU_CM0_ROOT_CON0,
    u32,
    [
        (hclk_npu_cm0_root_idle_th, 0, 16, []),
        (hclk_npu_cm0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_NPU_CM0_ROOT_CON1  0x0E2C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_NPU_CM0_ROOT_CON1,
    u32,
    [
        (hclk_npu_cm0_root_autocs_ctrl, 0, 12, []),
        (hclk_npu_cm0_root_autocs_en, 12, 1, []),
        (hclk_npu_cm0_root_switch_en, 13, 1, []),
        (hclk_npu_cm0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_NVM_ROOT_CON0  0x0E30
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_NVM_ROOT_CON0,
    u32,
    [
        (hclk_nvm_root_idle_th, 0, 16, []),
        (hclk_nvm_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_NVM_ROOT_CON1  0x0E34
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_NVM_ROOT_CON1,
    u32,
    [
        (hclk_nvm_root_autocs_ctrl, 0, 12, []),
        (hclk_nvm_root_autocs_en, 12, 1, []),
        (hclk_nvm_root_switch_en, 13, 1, []),
        (hclk_nvm_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_PHP_ROOT_CON0  0x0E38
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_PHP_ROOT_CON0,
    u32,
    [
        (pclk_php_root_idle_th, 0, 16, []),
        (pclk_php_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_PHP_ROOT_CON1  0x0E3C
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_PHP_ROOT_CON1,
    u32,
    [
        (pclk_php_root_autocs_ctrl, 0, 12, []),
        (pclk_php_root_autocs_en, 12, 1, []),
        (pclk_php_root_switch_en, 13, 1, []),
        (pclk_php_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_PCIE_ROOT_CON0  0x0E40
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_PCIE_ROOT_CON0,
    u32,
    [
        (aclk_pcie_root_idle_th, 0, 16, []),
        (aclk_pcie_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_PCIE_ROOT_CON1  0x0E44
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_PCIE_ROOT_CON1,
    u32,
    [
        (aclk_pcie_root_autocs_ctrl, 0, 12, []),
        (aclk_pcie_root_autocs_en, 12, 1, []),
        (aclk_pcie_root_switch_en, 13, 1, []),
        (aclk_pcie_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVDEC0_ROOT_CON0  0x0E48
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVDEC0_ROOT_CON0,
    u32,
    [
        (hclk_rkvdec0_root_idle_th, 0, 16, []),
        (hclk_rkvdec0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVDEC0_ROOT_CON1  0x0E4C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVDEC0_ROOT_CON1,
    u32,
    [
        (hclk_rkvdec0_root_autocs_ctrl, 0, 12, []),
        (hclk_rkvdec0_root_autocs_en, 12, 1, []),
        (hclk_rkvdec0_root_switch_en, 13, 1, []),
        (hclk_rkvdec0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVDEC1_ROOT_CON0  0x0E50
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVDEC1_ROOT_CON0,
    u32,
    [
        (hclk_rkvdec1_root_idle_th, 0, 16, []),
        (hclk_rkvdec1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVDEC1_ROOT_CON1  0x0E54
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVDEC1_ROOT_CON1,
    u32,
    [
        (hclk_rkvdec1_root_autocs_ctrl, 0, 12, []),
        (hclk_rkvdec1_root_autocs_en, 12, 1, []),
        (hclk_rkvdec1_root_switch_en, 13, 1, []),
        (hclk_rkvdec1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_TOP_ROOT_CON0  0x0E58
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_TOP_ROOT_CON0,
    u32,
    [
        (pclk_top_root_idle_th, 0, 16, []),
        (pclk_top_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_TOP_ROOT_CON1  0x0E5C
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_TOP_ROOT_CON1,
    u32,
    [
        (pclk_top_root_autocs_ctrl, 0, 12, []),
        (pclk_top_root_autocs_en, 12, 1, []),
        (pclk_top_root_switch_en, 13, 1, []),
        (pclk_top_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M500_ROOT_CON0  0x0E60
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M500_ROOT_CON0,
    u32,
    [
        (aclk_top_m500_root_idle_th, 0, 16, []),
        (aclk_top_m500_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M500_ROOT_CON1  0x0E64
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M500_ROOT_CON1,
    u32,
    [
        (aclk_top_m500_root_autocs_ctrl, 0, 12, []),
        (aclk_top_m500_root_autocs_en, 12, 1, []),
        (aclk_top_m500_root_switch_en, 13, 1, []),
        (aclk_top_m500_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_S200_ROOT_CON0  0x0E68
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_S200_ROOT_CON0,
    u32,
    [
        (aclk_top_s200_root_idle_th, 0, 16, []),
        (aclk_top_s200_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_S200_ROOT_CON1  0x0E6C
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_S200_ROOT_CON1,
    u32,
    [
        (aclk_top_s200_root_autocs_ctrl, 0, 12, []),
        (aclk_top_s200_root_autocs_en, 12, 1, []),
        (aclk_top_s200_root_switch_en, 13, 1, []),
        (aclk_top_s200_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_USB_ROOT_CON0  0x0E70
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_USB_ROOT_CON0,
    u32,
    [
        (hclk_usb_root_idle_th, 0, 16, []),
        (hclk_usb_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_USB_ROOT_CON1  0x0E74
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_USB_ROOT_CON1,
    u32,
    [
        (hclk_usb_root_autocs_ctrl, 0, 12, []),
        (hclk_usb_root_autocs_en, 12, 1, []),
        (hclk_usb_root_switch_en, 13, 1, []),
        (hclk_usb_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VDPU_ROOT_CON0  0x0E78
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VDPU_ROOT_CON0,
    u32,
    [
        (hclk_vdpu_root_idle_th, 0, 16, []),
        (hclk_vdpu_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VDPU_ROOT_CON1  0x0E7C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VDPU_ROOT_CON1,
    u32,
    [
        (hclk_vdpu_root_autocs_ctrl, 0, 12, []),
        (hclk_vdpu_root_autocs_en, 12, 1, []),
        (hclk_vdpu_root_switch_en, 13, 1, []),
        (hclk_vdpu_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVENC0_ROOT_CON0  0x0E80
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVENC0_ROOT_CON0,
    u32,
    [
        (hclk_rkvenc0_root_idle_th, 0, 16, []),
        (hclk_rkvenc0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVENC0_ROOT_CON1  0x0E84
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVENC0_ROOT_CON1,
    u32,
    [
        (hclk_rkvenc0_root_autocs_ctrl, 0, 12, []),
        (hclk_rkvenc0_root_autocs_en, 12, 1, []),
        (hclk_rkvenc0_root_switch_en, 13, 1, []),
        (hclk_rkvenc0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVENC1_ROOT_CON0  0x0E88
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVENC1_ROOT_CON0,
    u32,
    [
        (hclk_rkvenc1_root_idle_th, 0, 16, []),
        (hclk_rkvenc1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RKVENC1_ROOT_CON1  0x0E8C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RKVENC1_ROOT_CON1,
    u32,
    [
        (hclk_rkvenc1_root_autocs_ctrl, 0, 12, []),
        (hclk_rkvenc1_root_autocs_en, 12, 1, []),
        (hclk_rkvenc1_root_switch_en, 13, 1, []),
        (hclk_rkvenc1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VI_ROOT_CON0  0x0E90
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VI_ROOT_CON0,
    u32,
    [
        (hclk_vi_root_idle_th, 0, 16, []),
        (hclk_vi_root_wait_th, 16, 16, []),
    ]
);
// CRU_AUTOCS_HCLK_VI_ROOT_CON1  0x0E94
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VI_ROOT_CON1,
    u32,
    [
        (hclk_vi_root_autocs_ctrl, 0, 12, []),
        (hclk_vi_root_autocs_en, 12, 1, []),
        (hclk_vi_root_switch_en, 13, 1, []),
        (hclk_vi_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VI_ROOT_CON0  0x0E98
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VI_ROOT_CON0,
    u32,
    [
        (pclk_vi_root_idle_th, 0, 16, []),
        (pclk_vi_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VI_ROOT_CON1  0x0E9C
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VI_ROOT_CON1,
    u32,
    [
        (pclk_vi_root_autocs_ctrl, 0, 12, []),
        (pclk_vi_root_autocs_en, 12, 1, []),
        (pclk_vi_root_switch_en, 13, 1, []),
        (pclk_vi_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VOP_LOW_ROOT_CON0  0x0EA0
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VOP_LOW_ROOT_CON0,
    u32,
    [
        (aclk_vop_low_root_idle_th, 0, 16, []),
        (aclk_vop_low_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_VOP_LOW_ROOT_CON1  0x0EA4
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_VOP_LOW_ROOT_CON1,
    u32,
    [
        (aclk_vop_low_root_autocs_ctrl, 0, 12, []),
        (aclk_vop_low_root_autocs_en, 12, 1, []),
        (aclk_vop_low_root_switch_en, 13, 1, []),
        (aclk_vop_low_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VOP_ROOT_CON0  0x0EA8
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VOP_ROOT_CON0,
    u32,
    [
        (hclk_vop_root_idle_th, 0, 16, []),
        (hclk_vop_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VOP_ROOT_CON1  0x0EAC
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VOP_ROOT_CON1,
    u32,
    [
        (hclk_vop_root_autocs_ctrl, 0, 12, []),
        (hclk_vop_root_autocs_en, 12, 1, []),
        (hclk_vop_root_switch_en, 13, 1, []),
        (hclk_vop_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VOP_ROOT_CON0  0x0EB0
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VOP_ROOT_CON0,
    u32,
    [
        (pclk_vop_root_idle_th, 0, 16, []),
        (pclk_vop_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VOP_ROOT_CON1  0x0EB4
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VOP_ROOT_CON1,
    u32,
    [
        (pclk_vop_root_autocs_ctrl, 0, 12, []),
        (pclk_vop_root_autocs_en, 12, 1, []),
        (pclk_vop_root_switch_en, 13, 1, []),
        (pclk_vop_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO0_ROOT_CON0  0x0EB8
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO0_ROOT_CON0,
    u32,
    [
        (hclk_vo0_root_idle_th, 0, 16, []),
        (hclk_vo0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO0_ROOT_CON1  0x0EBC
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO0_ROOT_CON1,
    u32,
    [
        (hclk_vo0_root_autocs_ctrl, 0, 12, []),
        (hclk_vo0_root_autocs_en, 12, 1, []),
        (hclk_vo0_root_switch_en, 13, 1, []),
        (hclk_vo0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO0_S_ROOT_CON0  0x0EC0
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO0_S_ROOT_CON0,
    u32,
    [
        (hclk_vo0_s_root_idle_th, 0, 16, []),
        (hclk_vo0_s_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO0_S_ROOT_CON1  0x0EC4
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO0_S_ROOT_CON1,
    u32,
    [
        (hclk_vo0_s_root_autocs_ctrl, 0, 12, []),
        (hclk_vo0_s_root_autocs_en, 12, 1, []),
        (hclk_vo0_s_root_switch_en, 13, 1, []),
        (hclk_vo0_s_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO0_ROOT_CON0  0x0EC8
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO0_ROOT_CON0,
    u32,
    [
        (pclk_vo0_root_idle_th, 0, 16, []),
        (pclk_vo0_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO0_ROOT_CON1  0x0ECC
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO0_ROOT_CON1,
    u32,
    [
        (pclk_vo0_root_autocs_ctrl, 0, 12, []),
        (pclk_vo0_root_autocs_en, 12, 1, []),
        (pclk_vo0_root_switch_en, 13, 1, []),
        (pclk_vo0_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO0_S_ROOT_CON0  0x0ED0
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO0_S_ROOT_CON0,
    u32,
    [
        (pclk_vo0_s_root_idle_th, 0, 16, []),
        (pclk_vo0_s_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO0_S_ROOT_CON1  0x0ED4
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO0_S_ROOT_CON1,
    u32,
    [
        (pclk_vo0_s_root_autocs_ctrl, 0, 12, []),
        (pclk_vo0_s_root_autocs_en, 12, 1, []),
        (pclk_vo0_s_root_switch_en, 13, 1, []),
        (pclk_vo0_s_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1_ROOT_CON0  0x0ED8
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1_ROOT_CON0,
    u32,
    [
        (hclk_vo1_root_idle_th, 0, 16, []),
        (hclk_vo1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1_ROOT_CON1  0x0EDC
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1_ROOT_CON1,
    u32,
    [
        (hclk_vo1_root_autocs_ctrl, 0, 12, []),
        (hclk_vo1_root_autocs_en, 12, 1, []),
        (hclk_vo1_root_switch_en, 13, 1, []),
        (hclk_vo1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1_S_ROOT_CON0  0x0EE0
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1_S_ROOT_CON0,
    u32,
    [
        (hclk_vo1_s_root_idle_th, 0, 16, []),
        (hclk_vo1_s_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1_S_ROOT_CON1  0x0EE4
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1_S_ROOT_CON1,
    u32,
    [
        (hclk_vo1_s_root_autocs_ctrl, 0, 12, []),
        (hclk_vo1_s_root_autocs_en, 12, 1, []),
        (hclk_vo1_s_root_switch_en, 13, 1, []),
        (hclk_vo1_s_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO1_ROOT_CON0  0x0EE8
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO1_ROOT_CON0,
    u32,
    [
        (pclk_vo1_root_idle_th, 0, 16, []),
        (pclk_vo1_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO1_ROOT_CON1  0x0EEC
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO1_ROOT_CON1,
    u32,
    [
        (pclk_vo1_root_autocs_ctrl, 0, 12, []),
        (pclk_vo1_root_autocs_en, 12, 1, []),
        (pclk_vo1_root_switch_en, 13, 1, []),
        (pclk_vo1_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO1_S_ROOT_CON0  0x0EF0
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO1_S_ROOT_CON0,
    u32,
    [
        (pclk_vo1_s_root_idle_th, 0, 16, []),
        (pclk_vo1_s_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_VO1_S_ROOT_CON1  0x0EF4
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_VO1_S_ROOT_CON1,
    u32,
    [
        (pclk_vo1_s_root_autocs_ctrl, 0, 12, []),
        (pclk_vo1_s_root_autocs_en, 12, 1, []),
        (pclk_vo1_s_root_switch_en, 13, 1, []),
        (pclk_vo1_s_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_GPU_ROOT_CON0  0x0EF8
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_GPU_ROOT_CON0,
    u32,
    [
        (pclk_gpu_root_idle_th, 0, 16, []),
        (pclk_gpu_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_GPU_ROOT_CON1  0x0EFC
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_GPU_ROOT_CON1,
    u32,
    [
        (pclk_gpu_root_autocs_ctrl, 0, 12, []),
        (pclk_gpu_root_autocs_en, 12, 1, []),
        (pclk_gpu_root_switch_en, 13, 1, []),
        (pclk_gpu_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_CENTER_ROOT_CON0  0x0F00
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_CENTER_ROOT_CON0,
    u32,
    [
        (hclk_center_root_idle_th, 0, 16, []),
        (hclk_center_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_CENTER_ROOT_CON1  0x0F04
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_CENTER_ROOT_CON1,
    u32,
    [
        (hclk_center_root_autocs_ctrl, 0, 12, []),
        (hclk_center_root_autocs_en, 12, 1, []),
        (hclk_center_root_switch_en, 13, 1, []),
        (hclk_center_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_CENTER_ROOT_CON0  0x0F08
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_CENTER_ROOT_CON0,
    u32,
    [
        (pclk_center_root_idle_th, 0, 16, []),
        (pclk_center_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_CENTER_ROOT_CON1  0x0F0C
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_CENTER_ROOT_CON1,
    u32,
    [
        (pclk_center_root_autocs_ctrl, 0, 12, []),
        (pclk_center_root_autocs_en, 12, 1, []),
        (pclk_center_root_switch_en, 13, 1, []),
        (pclk_center_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_S200_ROOT_CON0  0x0F10
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_S200_ROOT_CON0,
    u32,
    [
        (aclk_center_s200_root_idle_th, 0, 16, []),
        (aclk_center_s200_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_CENTER_S200_ROOT_CON1  0x0F14
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_CENTER_S200_ROOT_CON1,
    u32,
    [
        (aclk_center_s200_root_autocs_ctrl, 0, 12, []),
        (aclk_center_s200_root_autocs_en, 12, 1, []),
        (aclk_center_s200_root_switch_en, 13, 1, []),
        (aclk_center_s200_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_SDIO_ROOT_CON0  0x0F18
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_SDIO_ROOT_CON0,
    u32,
    [
        (hclk_sdio_root_idle_th, 0, 16, []),
        (hclk_sdio_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_SDIO_ROOT_CON1  0x0F1C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_SDIO_ROOT_CON1,
    u32,
    [
        (hclk_sdio_root_autocs_ctrl, 0, 12, []),
        (hclk_sdio_root_autocs_en, 12, 1, []),
        (hclk_sdio_root_switch_en, 13, 1, []),
        (hclk_sdio_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RGA3_ROOT_CON0  0x0F20
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RGA3_ROOT_CON0,
    u32,
    [
        (hclk_rga3_root_idle_th, 0, 16, []),
        (hclk_rga3_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_RGA3_ROOT_CON1  0x0F24
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_RGA3_ROOT_CON1,
    u32,
    [
        (hclk_rga3_root_autocs_ctrl, 0, 12, []),
        (hclk_rga3_root_autocs_en, 12, 1, []),
        (hclk_rga3_root_switch_en, 13, 1, []),
        (hclk_rga3_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1USB_TOP_ROOT_CON0  0x0F28
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1USB_TOP_ROOT_CON0,
    u32,
    [
        (hclk_vo1usb_top_root_idle_th, 0, 16, []),
        (hclk_vo1usb_top_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_VO1USB_TOP_ROOT_CON1  0x0F2C
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_VO1USB_TOP_ROOT_CON1,
    u32,
    [
        (hclk_vo1usb_top_root_autocs_ctrl, 0, 12, []),
        (hclk_vo1usb_top_root_autocs_en, 12, 1, []),
        (hclk_vo1usb_top_root_switch_en, 13, 1, []),
        (hclk_vo1usb_top_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M300_ROOT_CON0  0x0F30
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M300_ROOT_CON0,
    u32,
    [
        (aclk_top_m300_root_idle_th, 0, 16, []),
        (aclk_top_m300_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_ACLK_TOP_M300_ROOT_CON1  0x0F34
generate_register_bitfields!(
    CRU_AUTOCS_ACLK_TOP_M300_ROOT_CON1,
    u32,
    [
        (aclk_top_m300_root_autocs_ctrl, 0, 12, []),
        (aclk_top_m300_root_autocs_en, 12, 1, []),
        (aclk_top_m300_root_switch_en, 13, 1, []),
        (aclk_top_m300_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_RKNN_DSU0_SRC_T_CON0  0x0F38
generate_register_bitfields!(
    CRU_AUTOCS_CLK_RKNN_DSU0_SRC_T_CON0,
    u32,
    [
        (clk_rknn_dsu0_src_t_idle_th, 0, 16, []),
        (clk_rknn_dsu0_src_t_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_CLK_RKNN_DSU0_SRC_T_CON1  0x0F3C
generate_register_bitfields!(
    CRU_AUTOCS_CLK_RKNN_DSU0_SRC_T_CON1,
    u32,
    [
        (clk_rknn_dsu0_src_t_autocs_ctrl, 0, 12, []),
        (clk_rknn_dsu0_src_t_autocs_en, 12, 1, []),
        (clk_rknn_dsu0_src_t_switch_en, 13, 1, []),
        (clk_rknn_dsu0_src_t_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_AUDIO_ROOT_CON0  0x0F40
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_AUDIO_ROOT_CON0,
    u32,
    [
        (hclk_audio_root_idle_th, 0, 16, []),
        (hclk_audio_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_HCLK_AUDIO_ROOT_CON1  0x0F44
generate_register_bitfields!(
    CRU_AUTOCS_HCLK_AUDIO_ROOT_CON1,
    u32,
    [
        (hclk_audio_root_autocs_ctrl, 0, 12, []),
        (hclk_audio_root_autocs_en, 12, 1, []),
        (hclk_audio_root_switch_en, 13, 1, []),
        (hclk_audio_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_AUDIO_ROOT_CON0  0x0F48
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_AUDIO_ROOT_CON0,
    u32,
    [
        (pclk_audio_root_idle_th, 0, 16, []),
        (pclk_audio_root_wait_th, 16, 16, []),
    ]
);

// CRU_AUTOCS_PCLK_AUDIO_ROOT_CON1  0x0F4C
generate_register_bitfields!(
    CRU_AUTOCS_PCLK_AUDIO_ROOT_CON1,
    u32,
    [
        (pclk_audio_root_autocs_ctrl, 0, 12, []),
        (pclk_audio_root_autocs_en, 12, 1, []),
        (pclk_audio_root_switch_en, 13, 1, []),
        (pclk_audio_root_clksel_cfg, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);
