use crate::generate_register_bitfields;
use tock_registers::{register_bitfields, registers::ReadWrite};

#[repr(C)]
pub struct V0pllRegisters {
    pub v0pll_con0: ReadWrite<u32, CRU_V0PLL_CON0::Register>,
    pub v0pll_con1: ReadWrite<u32, CRU_V0PLL_CON1::Register>,
    pub v0pll_con2: ReadWrite<u32, CRU_V0PLL_CON2::Register>,
    pub v0pll_con3: ReadWrite<u32, CRU_V0PLL_CON3::Register>,
    pub v0pll_con4: ReadWrite<u32, CRU_V0PLL_CON4::Register>,
    pub v0pll_con5: ReadWrite<u32, CRU_V0PLL_CON5::Register>,
    pub v0pll_con6: ReadWrite<u32, CRU_V0PLL_CON6::Register>,
    _reserved: u32,
}

// CRU_V0PLL_CON0  0x0160;
generate_register_bitfields!(
    CRU_V0PLL_CON0,
    u32,
    [
        (v0pll_m, 0, 10, []),
        (reserved, 10, 5, []),
        (v0pll_bp, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_V0PLL_CON1  0x0164;
generate_register_bitfields!(
    CRU_V0PLL_CON1,
    u32,
    [
        (v0pll_p, 0, 6, []),
        (v0pll_s, 6, 3, []),
        (reserved0, 9, 4, []),
        (v0pll_resetb, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_V0PLL_CON2  0x0168;
generate_register_bitfields!(
    CRU_V0PLL_CON2,
    u32,
    [(v0pll_k, 0, 16, []), (reserved, 16, 16, [])]
);

// CRU_V0PLL_CON3  0x016C;
generate_register_bitfields!(
    CRU_V0PLL_CON3,
    u32,
    [
        (v0pll_mfr, 0, 8, []),
        (v0pll_mrr, 8, 6, []),
        (v0pll_sel_pf, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_V0PLL_CON4  0x0170;
generate_register_bitfields!(
    CRU_V0PLL_CON4,
    u32,
    [
        (v0pll_sscg_en, 0, 1, []),
        (reserved0, 1, 2, []),
        (v0pll_afc_enb, 3, 1, []),
        (v0pll_extafc, 4, 5, []),
        (reserved1, 9, 5, []),
        (v0pll_feed_en, 14, 1, []),
        (v0pll_fsel, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_V0PLL_CON5  0x0174;
generate_register_bitfields!(
    CRU_V0PLL_CON5,
    u32,
    [
        (v0pll_fout_mask, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_V0PLL_CON6  0x0178;
generate_register_bitfields!(
    CRU_V0PLL_CON6,
    u32,
    [
        (reserved, 0, 10, []),
        (v0pll_afc_code, 10, 4, []),
        (v0pll_lock, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

#[repr(C)]
pub struct AupllRegisters {
    pub aupll_con0: ReadWrite<u32, CRU_AUPLL_CON0::Register>,
    pub aupll_con1: ReadWrite<u32, CRU_AUPLL_CON1::Register>,
    pub aupll_con2: ReadWrite<u32, CRU_AUPLL_CON2::Register>,
    pub aupll_con3: ReadWrite<u32, CRU_AUPLL_CON3::Register>,
    pub aupll_con4: ReadWrite<u32, CRU_AUPLL_CON4::Register>,
    pub aupll_con5: ReadWrite<u32, CRU_AUPLL_CON5::Register>,
    pub aupll_con6: ReadWrite<u32, CRU_AUPLL_CON6::Register>,
    _reserved: u32,
}

// CRU_AUPLL_CON0  0x0180;
generate_register_bitfields!(
    CRU_AUPLL_CON0,
    u32,
    [
        (aupll_m, 0, 10, []),
        (reserved, 10, 5, []),
        (aupll_bp, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_AUPLL_CON1  0x0184;
generate_register_bitfields!(
    CRU_AUPLL_CON1,
    u32,
    [
        (aupll_p, 0, 6, []),
        (aupll_s, 6, 3, []),
        (reserved0, 9, 4, []),
        (aupll_resetb, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_AUPLL_CON2  0x0188;
generate_register_bitfields!(
    CRU_AUPLL_CON2,
    u32,
    [(aupll_k, 0, 16, []), (reserved, 16, 16, [])]
);

// CRU_AUPLL_CON3  0x018C;
generate_register_bitfields!(
    CRU_AUPLL_CON3,
    u32,
    [
        (aupll_mfr, 0, 8, []),
        (aupll_mrr, 8, 6, []),
        (aupll_sel_pf, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_AUPLL_CON4  0x0190;
generate_register_bitfields!(
    CRU_AUPLL_CON4,
    u32,
    [
        (aupll_sscg_en, 0, 1, []),
        (reserved0, 1, 2, []),
        (aupll_afc_enb, 3, 1, []),
        (aupll_extafc, 4, 5, []),
        (reserved1, 9, 5, []),
        (aupll_feed_en, 14, 1, []),
        (aupll_fsel, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_AUPLL_CON5  0x0194;
generate_register_bitfields!(
    CRU_AUPLL_CON5,
    u32,
    [
        (aupll_fout_mask, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_AUPLL_CON6  0x0198;
generate_register_bitfields!(
    CRU_AUPLL_CON6,
    u32,
    [
        (reserved, 0, 10, []),
        (aupll_afc_code, 10, 4, []),
        (aupll_lock, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

#[repr(C)]
pub struct CpllRegisters {
    pub cpll_con0: ReadWrite<u32, CRU_CPLL_CON0::Register>,
    pub cpll_con1: ReadWrite<u32, CRU_CPLL_CON1::Register>,
    pub cpll_con2: ReadWrite<u32, CRU_CPLL_CON2::Register>,
    pub cpll_con3: ReadWrite<u32, CRU_CPLL_CON3::Register>,
    pub cpll_con4: ReadWrite<u32, CRU_CPLL_CON4::Register>,
    pub cpll_con5: ReadWrite<u32, CRU_CPLL_CON5::Register>,
    pub cpll_con6: ReadWrite<u32, CRU_CPLL_CON6::Register>,
    _reserved: u32,
}

// CRU_CPLL_CON0  0x01A0;
generate_register_bitfields!(
    CRU_CPLL_CON0,
    u32,
    [
        (cpll_m, 0, 10, []),
        (reserved, 10, 5, []),
        (cpll_bp, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_CPLL_CON1  0x01A4;
generate_register_bitfields!(
    CRU_CPLL_CON1,
    u32,
    [
        (cpll_p, 0, 6, []),
        (cpll_s, 6, 3, []),
        (reserved0, 9, 4, []),
        (cpll_resetb, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_CPLL_CON2  0x01A8;
generate_register_bitfields!(
    CRU_CPLL_CON2,
    u32,
    [(cpll_k, 0, 16, []), (reserved, 16, 16, [])]
);

// CRU_CPLL_CON3  0x01AC;
generate_register_bitfields!(
    CRU_CPLL_CON3,
    u32,
    [
        (cpll_mfr, 0, 8, []),
        (cpll_mrr, 8, 6, []),
        (cpll_sel_pf, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_CPLL_CON4  0x01B0;
generate_register_bitfields!(
    CRU_CPLL_CON4,
    u32,
    [
        (cpll_sscg_en, 0, 1, []),
        (reserved0, 1, 2, []),
        (cpll_afc_enb, 3, 1, []),
        (cpll_extafc, 4, 5, []),
        (reserved1, 9, 5, []),
        (cpll_feed_en, 14, 1, []),
        (cpll_fsel, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_CPLL_CON5  0x01B4;
generate_register_bitfields!(
    CRU_CPLL_CON5,
    u32,
    [
        (cpll_fout_mask, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_CPLL_CON6  0x01B8;
generate_register_bitfields!(
    CRU_CPLL_CON6,
    u32,
    [
        (reserved, 0, 10, []),
        (cpll_afc_code, 10, 4, []),
        (cpll_lock, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

#[repr(C)]
pub struct GpllRegisters {
    pub gpll_con0: ReadWrite<u32, CRU_GPLL_CON0::Register>,
    pub gpll_con1: ReadWrite<u32, CRU_GPLL_CON1::Register>,
    pub gpll_con2: ReadWrite<u32, CRU_GPLL_CON2::Register>,
    pub gpll_con3: ReadWrite<u32, CRU_GPLL_CON3::Register>,
    pub gpll_con4: ReadWrite<u32, CRU_GPLL_CON4::Register>,
    pub gpll_con5: ReadWrite<u32, CRU_GPLL_CON5::Register>,
    pub gpll_con6: ReadWrite<u32, CRU_GPLL_CON6::Register>,
    _reserved: u32,
}

// CRU_GPLL_CON0  0x01C0;
generate_register_bitfields!(
    CRU_GPLL_CON0,
    u32,
    [
        (gpll_m, 0, 10, []),
        (reserved, 10, 5, []),
        (gpll_bp, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_GPLL_CON1  0x01C4;
generate_register_bitfields!(
    CRU_GPLL_CON1,
    u32,
    [
        (gpll_p, 0, 6, []),
        (gpll_s, 6, 3, []),
        (reserved0, 9, 4, []),
        (gpll_resetb, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_GPLL_CON2  0x01C8;
generate_register_bitfields!(
    CRU_GPLL_CON2,
    u32,
    [(gpll_k, 0, 16, []), (reserved, 16, 16, [])]
);

// CRU_GPLL_CON3  0x01CC;
generate_register_bitfields!(
    CRU_GPLL_CON3,
    u32,
    [
        (gpll_mfr, 0, 8, []),
        (gpll_mrr, 8, 6, []),
        (gpll_sel_pf, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_GPLL_CON4  0x01D0;
generate_register_bitfields!(
    CRU_GPLL_CON4,
    u32,
    [
        (gpll_sscg_en, 0, 1, []),
        (reserved0, 1, 2, []),
        (gpll_afc_enb, 3, 1, []),
        (gpll_extafc, 4, 5, []),
        (reserved1, 9, 5, []),
        (gpll_feed_en, 14, 1, []),
        (gpll_fsel, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_GPLL_CON5  0x01D4;
generate_register_bitfields!(
    CRU_GPLL_CON5,
    u32,
    [
        (gpll_fout_mask, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_GPLL_CON6  0x01D8;
generate_register_bitfields!(
    CRU_GPLL_CON6,
    u32,
    [
        (reserved, 0, 10, []),
        (gpll_afc_code, 10, 4, []),
        (gpll_lock, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

#[repr(C)]
pub struct NpllRegisters {
    pub npll_con0: ReadWrite<u32, CRU_NPLL_CON0::Register>,
    pub npll_con1: ReadWrite<u32, CRU_NPLL_CON1::Register>,
    pub npll_con2: ReadWrite<u32, CRU_NPLL_CON2::Register>,
    pub npll_con3: ReadWrite<u32, CRU_NPLL_CON3::Register>,
    pub npll_con4: ReadWrite<u32, CRU_NPLL_CON4::Register>,
    pub npll_con5: ReadWrite<u32, CRU_NPLL_CON5::Register>,
    pub npll_con6: ReadWrite<u32, CRU_NPLL_CON6::Register>,
    _reserved: u32,
}

// CRU_NPLL_CON0  0x01E0;
generate_register_bitfields!(
    CRU_NPLL_CON0,
    u32,
    [
        (npll_m, 0, 10, []),
        (reserved, 10, 5, []),
        (npll_bp, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_NPLL_CON1  0x01E4;
generate_register_bitfields!(
    CRU_NPLL_CON1,
    u32,
    [
        (npll_p, 0, 6, []),
        (npll_s, 6, 3, []),
        (reserved0, 9, 4, []),
        (npll_resetb, 13, 1, []),
        (reserved1, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_NPLL_CON2  0x01E8;
generate_register_bitfields!(
    CRU_NPLL_CON2,
    u32,
    [(npll_k, 0, 16, []), (reserved, 16, 16, [])]
);

// CRU_NPLL_CON3  0x01EC;
generate_register_bitfields!(
    CRU_NPLL_CON3,
    u32,
    [
        (npll_mfr, 0, 8, []),
        (npll_mrr, 8, 6, []),
        (npll_sel_pf, 14, 2, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_NPLL_CON4  0x01F0;
generate_register_bitfields!(
    CRU_NPLL_CON4,
    u32,
    [
        (npll_sscg_en, 0, 1, []),
        (reserved0, 1, 2, []),
        (npll_afc_enb, 3, 1, []),
        (npll_extafc, 4, 5, []),
        (reserved1, 9, 5, []),
        (npll_feed_en, 14, 1, []),
        (npll_fsel, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_NPLL_CON5  0x01F4;
generate_register_bitfields!(
    CRU_NPLL_CON5,
    u32,
    [
        (npll_fout_mask, 0, 1, []),
        (reserved, 1, 15, []),
        (write_enable, 16, 16, [])
    ]
);

// CRU_NPLL_CON6  0x01F8;
generate_register_bitfields!(
    CRU_NPLL_CON6,
    u32,
    [
        (reserved, 0, 10, []),
        (npll_afc_code, 10, 4, []),
        (npll_lock, 15, 1, []),
        (write_enable, 16, 16, [])
    ]
);
