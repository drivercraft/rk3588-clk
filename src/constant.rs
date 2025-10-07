pub const CCLK_EMMC_DIV_SHIFT: u32 = 8;
pub const CCLK_EMMC_DIV_MASK: u32 = 0x3f << CCLK_EMMC_DIV_SHIFT;
pub const CCLK_EMMC_SEL_SHIFT: u32 = 14;
pub const CCLK_EMMC_SEL_MASK: u32 = 3 << CCLK_EMMC_SEL_SHIFT;

pub const CCLK_EMMC_SEL_GPLL: u32 = 0;
pub const CCLK_EMMC_SEL_CPLL: u32 = 1;

pub const SCLK_SFC_SEL_CPLL: u32 = 0;
pub const SCLK_SFC_SEL_GPLL: u32 = 1;
pub const SCLK_SFC_SEL_24M: u32 = 2;

// sd/mmmc
pub const CCLK_SRC_SDIO: u32 = 395;
pub const CCLK_EMMC: u32 = 300;
pub const BCLK_EMMC: u32 = 301;
pub const SCLK_SFC: u32 = 303;
pub const DCLK_DECOM: u32 = 150;

// npu
pub const ACLK_NPU1: u32 = 276;
pub const HCLK_NPU1: u32 = 277;
pub const ACLK_NPU2: u32 = 278;
pub const HCLK_NPU2: u32 = 279;
pub const HCLK_NPU_CM0_ROOT: u32 = 280;
pub const FCLK_NPU_CM0_CORE: u32 = 281;
pub const CLK_NPU_CM0_RTC: u32 = 282;
pub const PCLK_NPU_PVTM: u32 = 283;
pub const PCLK_NPU_GRF: u32 = 284;
pub const CLK_NPU_PVTM: u32 = 285;
pub const CLK_CORE_NPU_PVTM: u32 = 286;
pub const ACLK_NPU0: u32 = 287;
pub const HCLK_NPU0: u32 = 288;
pub const HCLK_NPU_ROOT: u32 = 289;
pub const CLK_NPU_DSU0: u32 = 290;
pub const PCLK_NPU_ROOT: u32 = 291;
pub const PCLK_NPU_TIMER: u32 = 292;
pub const CLK_NPUTIMER_ROOT: u32 = 293;
pub const CLK_NPUTIMER0: u32 = 294;
pub const CLK_NPUTIMER1: u32 = 295;
pub const PCLK_NPU_WDT: u32 = 296;
pub const TCLK_NPU_WDT: u32 = 297;
