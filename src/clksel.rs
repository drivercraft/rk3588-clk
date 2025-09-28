use crate::generate_register_bitfields;
use tock_registers::{register_bitfields, registers::ReadWrite};

#[repr(C)]
pub struct ClkSelRegisters {
    pub cru_clksel_con00: ReadWrite<u32, CRU_CLKSEL_CON00::Register>,
    pub cru_clksel_con01: ReadWrite<u32, CRU_CLKSEL_CON01::Register>,
    pub cru_clksel_con02: ReadWrite<u32, CRU_CLKSEL_CON02::Register>,
    pub cru_clksel_con03: ReadWrite<u32, CRU_CLKSEL_CON03::Register>,
    pub cru_clksel_con04: ReadWrite<u32, CRU_CLKSEL_CON04::Register>,
    pub cru_clksel_con05: ReadWrite<u32, CRU_CLKSEL_CON05::Register>,
    pub cru_clksel_con06: ReadWrite<u32, CRU_CLKSEL_CON06::Register>,
    pub cru_clksel_con07: ReadWrite<u32, CRU_CLKSEL_CON07::Register>,
    pub cru_clksel_con08: ReadWrite<u32, CRU_CLKSEL_CON08::Register>,
    pub cru_clksel_con09: ReadWrite<u32, CRU_CLKSEL_CON09::Register>,
    pub cru_clksel_con10: ReadWrite<u32, CRU_CLKSEL_CON10::Register>,
    _reserved0: [u32; 4],
    pub cru_clksel_con15: ReadWrite<u32, CRU_CLKSEL_CON15::Register>,
    pub cru_clksel_con16: ReadWrite<u32, CRU_CLKSEL_CON16::Register>,
    pub cru_clksel_con17: ReadWrite<u32, CRU_CLKSEL_CON17::Register>,
    pub cru_clksel_con18: ReadWrite<u32, CRU_CLKSEL_CON18::Register>,
    pub cru_clksel_con19: ReadWrite<u32, CRU_CLKSEL_CON19::Register>,
    pub cru_clksel_con20: ReadWrite<u32, CRU_CLKSEL_CON20::Register>,
    pub cru_clksel_con21: ReadWrite<u32, CRU_CLKSEL_CON21::Register>,
    pub cru_clksel_con22: ReadWrite<u32, CRU_CLKSEL_CON22::Register>,
    _reserved1: u32,
    pub cru_clksel_con24: ReadWrite<u32, CRU_CLKSEL_CON24::Register>,
    pub cru_clksel_con25: ReadWrite<u32, CRU_CLKSEL_CON25::Register>,
    pub cru_clksel_con26: ReadWrite<u32, CRU_CLKSEL_CON26::Register>,
    pub cru_clksel_con27: ReadWrite<u32, CRU_CLKSEL_CON27::Register>,
    pub cru_clksel_con28: ReadWrite<u32, CRU_CLKSEL_CON28::Register>,
    pub cru_clksel_con29: ReadWrite<u32, CRU_CLKSEL_CON29::Register>,
    pub cru_clksel_con30: ReadWrite<u32, CRU_CLKSEL_CON30::Register>,
    pub cru_clksel_con31: ReadWrite<u32, CRU_CLKSEL_CON31::Register>,
    pub cru_clksel_con32: ReadWrite<u32, CRU_CLKSEL_CON32::Register>,
    pub cru_clksel_con33: ReadWrite<u32, CRU_CLKSEL_CON33::Register>,
    pub cru_clksel_con34: ReadWrite<u32, CRU_CLKSEL_CON34::Register>,
    pub cru_clksel_con35: ReadWrite<u32, CRU_CLKSEL_CON35::Register>,
    pub cru_clksel_con36: ReadWrite<u32, CRU_CLKSEL_CON36::Register>,
    _reserved2: u32,
    pub cru_clksel_con38: ReadWrite<u32, CRU_CLKSEL_CON38::Register>,
    pub cru_clksel_con39: ReadWrite<u32, CRU_CLKSEL_CON39::Register>,
    pub cru_clksel_con40: ReadWrite<u32, CRU_CLKSEL_CON40::Register>,
    pub cru_clksel_con41: ReadWrite<u32, CRU_CLKSEL_CON41::Register>,
    pub cru_clksel_con42: ReadWrite<u32, CRU_CLKSEL_CON42::Register>,
    pub cru_clksel_con43: ReadWrite<u32, CRU_CLKSEL_CON43::Register>,
    pub cru_clksel_con44: ReadWrite<u32, CRU_CLKSEL_CON44::Register>,
    pub cru_clksel_con45: ReadWrite<u32, CRU_CLKSEL_CON45::Register>,
    pub cru_clksel_con46: ReadWrite<u32, CRU_CLKSEL_CON46::Register>,
    pub cru_clksel_con47: ReadWrite<u32, CRU_CLKSEL_CON47::Register>,
    pub cru_clksel_con48: ReadWrite<u32, CRU_CLKSEL_CON48::Register>,
    pub cru_clksel_con49: ReadWrite<u32, CRU_CLKSEL_CON49::Register>,
    pub cru_clksel_con50: ReadWrite<u32, CRU_CLKSEL_CON50::Register>,
    pub cru_clksel_con51: ReadWrite<u32, CRU_CLKSEL_CON51::Register>,
    pub cru_clksel_con52: ReadWrite<u32, CRU_CLKSEL_CON52::Register>,
    pub cru_clksel_con53: ReadWrite<u32, CRU_CLKSEL_CON53::Register>,
    pub cru_clksel_con54: ReadWrite<u32, CRU_CLKSEL_CON54::Register>,
    pub cru_clksel_con55: ReadWrite<u32, CRU_CLKSEL_CON55::Register>,
    pub cru_clksel_con56: ReadWrite<u32, CRU_CLKSEL_CON56::Register>,
    pub cru_clksel_con57: ReadWrite<u32, CRU_CLKSEL_CON57::Register>,
    pub cru_clksel_con58: ReadWrite<u32, CRU_CLKSEL_CON58::Register>,
    pub cru_clksel_con59: ReadWrite<u32, CRU_CLKSEL_CON59::Register>,
    pub cru_clksel_con60: ReadWrite<u32, CRU_CLKSEL_CON60::Register>,
    pub cru_clksel_con61: ReadWrite<u32, CRU_CLKSEL_CON61::Register>,
    pub cru_clksel_con62: ReadWrite<u32, CRU_CLKSEL_CON62::Register>,
    pub cru_clksel_con63: ReadWrite<u32, CRU_CLKSEL_CON63::Register>,
    // cru_clksel_con64: ReadWrite<u32, CRU_CLKSEL_CON64::Register>,
    _reserved3: u32,
    pub cru_clksel_con65: ReadWrite<u32, CRU_CLKSEL_CON65::Register>,
    // cru_clksel_con66: ReadWrite<u32, CRU_CLKSEL_CON66::Register>,
    _reserved4: u32,
    pub cru_clksel_con67: ReadWrite<u32, CRU_CLKSEL_CON67::Register>,
    _reserved5: [u32; 5],
    // cru_clksel_con68: ReadWrite<u32, CRU_CLKSEL_CON68::Register>,
    // cru_clksel_con69: ReadWrite<u32, CRU_CLKSEL_CON69::Register>,
    // cru_clksel_con70: ReadWrite<u32, CRU_CLKSEL_CON70::Register>,
    // cru_clksel_con71: ReadWrite<u32, CRU_CLKSEL_CON71::Register>,
    // cru_clksel_con72: ReadWrite<u32, CRU_CLKSEL_CON72::Register>,
    pub cru_clksel_con73: ReadWrite<u32, CRU_CLKSEL_CON73::Register>,
    pub cru_clksel_con74: ReadWrite<u32, CRU_CLKSEL_CON74::Register>,
    // cru_clksel_con75: ReadWrite<u32, CRU_CLKSEL_CON75::Register>,
    // cru_clksel_con76: ReadWrite<u32, CRU_CLKSEL_CON76::Register>,
    _reserved6: [u32; 2],
    pub cru_clksel_con77: ReadWrite<u32, CRU_CLKSEL_CON77::Register>,
    pub cru_clksel_con78: ReadWrite<u32, CRU_CLKSEL_CON78::Register>,
    _reserved7: u32,
    // cru_clksel_con79: ReadWrite<u32, CRU_CLKSEL_CON79::Register>,
    pub cru_clksel_con80: ReadWrite<u32, CRU_CLKSEL_CON80::Register>,
    pub cru_clksel_con81: ReadWrite<u32, CRU_CLKSEL_CON81::Register>,
    pub cru_clksel_con82: ReadWrite<u32, CRU_CLKSEL_CON82::Register>,
    pub cru_clksel_con83: ReadWrite<u32, CRU_CLKSEL_CON83::Register>,
    pub cru_clksel_con84: ReadWrite<u32, CRU_CLKSEL_CON84::Register>,
    pub cru_clksel_con85: ReadWrite<u32, CRU_CLKSEL_CON85::Register>,
    _reserved8: [u32; 3],
    // cru_clksel_con86: ReadWrite<u32, CRU_CLKSEL_CON86::Register>,
    // cru_clksel_con87: ReadWrite<u32, CRU_CLKSEL_CON87::Register>,
    // cru_clksel_con88: ReadWrite<u32, CRU_CLKSEL_CON88::Register>,
    pub cru_clksel_con89: ReadWrite<u32, CRU_CLKSEL_CON89::Register>,
    pub cru_clksel_con90: ReadWrite<u32, CRU_CLKSEL_CON90::Register>,
    pub cru_clksel_con91: ReadWrite<u32, CRU_CLKSEL_CON91::Register>,
    _reserved9: u32,
    // cru_clksel_con92: ReadWrite<u32, CRU_CLKSEL_CON92::Register>,
    pub cru_clksel_con93: ReadWrite<u32, CRU_CLKSEL_CON93::Register>,
    pub cru_clksel_con94: ReadWrite<u32, CRU_CLKSEL_CON94::Register>,
    _reserved10: u32,
    // cru_clksel_con95: ReadWrite<u32, CRU_CLKSEL_CON95::Register>,
    pub cru_clksel_con96: ReadWrite<u32, CRU_CLKSEL_CON96::Register>,
    _reserved11: u32,
    // cru_clksel_con97: ReadWrite<u32, CRU_CLKSEL_CON97::Register>,
    pub cru_clksel_con98: ReadWrite<u32, CRU_CLKSEL_CON98::Register>,
    pub cru_clksel_con99: ReadWrite<u32, CRU_CLKSEL_CON99::Register>,
    pub cru_clksel_con100: ReadWrite<u32, CRU_CLKSEL_CON100::Register>,
    _reserved12: u32,
    // cru_clksel_con101: ReadWrite<u32, CRU_CLKSEL_CON101::Register>,
    pub cru_clksel_con102: ReadWrite<u32, CRU_CLKSEL_CON102::Register>,
    _reserved13: u32,
    // cru_clksel_con103: ReadWrite<u32, CRU_CLKSEL_CON103::Register>,
    cru_clksel_con104: ReadWrite<u32, CRU_CLKSEL_CON104::Register>,
    _reserved14: u32,
    // cru_clksel_con105: ReadWrite<u32, CRU_CLKSEL_CON105::Register>,
    pub cru_clksel_con106: ReadWrite<u32, CRU_CLKSEL_CON106::Register>,
    pub cru_clksel_con107: ReadWrite<u32, CRU_CLKSEL_CON107::Register>,
    pub cru_clksel_con108: ReadWrite<u32, CRU_CLKSEL_CON108::Register>,
    _reserved15: u32,
    // cru_clksel_con109: ReadWrite<u32, CRU_CLKSEL_CON109::Register>,
    pub cru_clksel_con110: ReadWrite<u32, CRU_CLKSEL_CON110::Register>,
    pub cru_clksel_con111: ReadWrite<u32, CRU_CLKSEL_CON111::Register>,
    pub cru_clksel_con112: ReadWrite<u32, CRU_CLKSEL_CON112::Register>,
    pub cru_clksel_con113: ReadWrite<u32, CRU_CLKSEL_CON113::Register>,
    pub cru_clksel_con114: ReadWrite<u32, CRU_CLKSEL_CON114::Register>,
    pub cru_clksel_con115: ReadWrite<u32, CRU_CLKSEL_CON115::Register>,
    pub cru_clksel_con116: ReadWrite<u32, CRU_CLKSEL_CON116::Register>,
    pub cru_clksel_con117: ReadWrite<u32, CRU_CLKSEL_CON117::Register>,
    pub cru_clksel_con118: ReadWrite<u32, CRU_CLKSEL_CON118::Register>,
    pub cru_clksel_con119: ReadWrite<u32, CRU_CLKSEL_CON119::Register>,
    pub cru_clksel_con120: ReadWrite<u32, CRU_CLKSEL_CON120::Register>,
    pub cru_clksel_con121: ReadWrite<u32, CRU_CLKSEL_CON121::Register>,
    pub cru_clksel_con122: ReadWrite<u32, CRU_CLKSEL_CON122::Register>,
    pub cru_clksel_con123: ReadWrite<u32, CRU_CLKSEL_CON123::Register>,
    pub cru_clksel_con124: ReadWrite<u32, CRU_CLKSEL_CON124::Register>,
    pub cru_clksel_con125: ReadWrite<u32, CRU_CLKSEL_CON125::Register>,
    pub cru_clksel_con126: ReadWrite<u32, CRU_CLKSEL_CON126::Register>,
    _reserved16: u32,
    // cru_clksel_con127: ReadWrite<u32, CRU_CLKSEL_CON127::Register>,
    pub cru_clksel_con128: ReadWrite<u32, CRU_CLKSEL_CON128::Register>,
    pub cru_clksel_con129: ReadWrite<u32, CRU_CLKSEL_CON129::Register>,
    pub cru_clksel_con130: ReadWrite<u32, CRU_CLKSEL_CON130::Register>,
    pub cru_clksel_con131: ReadWrite<u32, CRU_CLKSEL_CON131::Register>,
    _reserved17: u32,
    // cru_clksel_con132: ReadWrite<u32, CRU_CLKSEL_CON132::Register>,
    pub cru_clksel_con133: ReadWrite<u32, CRU_CLKSEL_CON133::Register>,
    _reserved18: [u32; 2],
    // cru_clksel_con134: ReadWrite<u32, CRU_CLKSEL_CON134::Register>,
    // cru_clksel_con135: ReadWrite<u32, CRU_CLKSEL_CON135::Register>,
    pub cru_clksel_con136: ReadWrite<u32, CRU_CLKSEL_CON136::Register>,
    _reserved19: u32,
    // cru_clksel_con137: ReadWrite<u32, CRU_CLKSEL_CON137::Register>,
    pub cru_clksel_con138: ReadWrite<u32, CRU_CLKSEL_CON138::Register>,
    pub cru_clksel_con139: ReadWrite<u32, CRU_CLKSEL_CON139::Register>,
    pub cru_clksel_con140: ReadWrite<u32, CRU_CLKSEL_CON140::Register>,
    pub cru_clksel_con141: ReadWrite<u32, CRU_CLKSEL_CON141::Register>,
    pub cru_clksel_con142: ReadWrite<u32, CRU_CLKSEL_CON142::Register>,
    _reserved20: u32,
    // cru_clksel_con143: ReadWrite<u32, CRU_CLKSEL_CON143::Register>,
    pub cru_clksel_con144: ReadWrite<u32, CRU_CLKSEL_CON144::Register>,
    pub cru_clksel_con145: ReadWrite<u32, CRU_CLKSEL_CON145::Register>,
    pub cru_clksel_con146: ReadWrite<u32, CRU_CLKSEL_CON146::Register>,
    pub cru_clksel_con147: ReadWrite<u32, CRU_CLKSEL_CON147::Register>,
    pub cru_clksel_con148: ReadWrite<u32, CRU_CLKSEL_CON148::Register>,
    pub cru_clksel_con149: ReadWrite<u32, CRU_CLKSEL_CON149::Register>,
    pub cru_clksel_con150: ReadWrite<u32, CRU_CLKSEL_CON150::Register>,
    pub cru_clksel_con151: ReadWrite<u32, CRU_CLKSEL_CON151::Register>,
    pub cru_clksel_con152: ReadWrite<u32, CRU_CLKSEL_CON152::Register>,
    pub cru_clksel_con153: ReadWrite<u32, CRU_CLKSEL_CON153::Register>,
    pub cru_clksel_con154: ReadWrite<u32, CRU_CLKSEL_CON154::Register>,
    pub cru_clksel_con155: ReadWrite<u32, CRU_CLKSEL_CON155::Register>,
    pub cru_clksel_con156: ReadWrite<u32, CRU_CLKSEL_CON156::Register>,
    pub cru_clksel_con157: ReadWrite<u32, CRU_CLKSEL_CON157::Register>,
    pub cru_clksel_con158: ReadWrite<u32, CRU_CLKSEL_CON158::Register>,
    pub cru_clksel_con159: ReadWrite<u32, CRU_CLKSEL_CON159::Register>,
    pub cru_clksel_con160: ReadWrite<u32, CRU_CLKSEL_CON160::Register>,
    pub cru_clksel_con161: ReadWrite<u32, CRU_CLKSEL_CON161::Register>,
    _reserved21: u32,
    // cru_clksel_con162: ReadWrite<u32, CRU_CLKSEL_CON162::Register>,
    pub cru_clksel_con163: ReadWrite<u32, CRU_CLKSEL_CON163::Register>,
    _reserved22: u32,
    // cru_clksel_con164: ReadWrite<u32, CRU_CLKSEL_CON164::Register>,
    pub cru_clksel_con165: ReadWrite<u32, CRU_CLKSEL_CON165::Register>,
    pub cru_clksel_con166: ReadWrite<u32, CRU_CLKSEL_CON166::Register>,
    _reserved23: [u32; 3],
    // cru_clksel_con167: ReadWrite<u32, CRU_CLKSEL_CON167::Register>,
    // cru_clksel_con168: ReadWrite<u32, CRU_CLKSEL_CON168::Register>,
    // cru_clksel_con169: ReadWrite<u32, CRU_CLKSEL_CON169::Register>,
    pub cru_clksel_con170: ReadWrite<u32, CRU_CLKSEL_CON170::Register>,
    _reserved24: u32,
    // cru_clksel_con171: ReadWrite<u32, CRU_CLKSEL_CON171::Register>,
    pub cru_clksel_con172: ReadWrite<u32, CRU_CLKSEL_CON172::Register>,
    _reserved25: u32,
    // cru_clksel_con173: ReadWrite<u32, CRU_CLKSEL_CON173::Register>,
    pub cru_clksel_con174: ReadWrite<u32, CRU_CLKSEL_CON174::Register>,
    _reserved26: u32,
    // cru_clksel_con175: ReadWrite<u32, CRU_CLKSEL_CON175::Register>,
    pub cru_clksel_con176: ReadWrite<u32, CRU_CLKSEL_CON176::Register>,
    pub cru_clksel_con177: ReadWrite<u32, CRU_CLKSEL_CON177::Register>,
}

// CRU_CLKSEL_CON00  0x0300;
generate_register_bitfields!(
    CRU_CLKSEL_CON00,
    u32,
    [
        (clk_matrix_50m_src_div, 0, 5, []),
        (clk_matrix_50m_src_sel, 5, 1, []),
        (clk_matrix_100m_src_div, 6, 5, []),
        (clk_matrix_100m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON01  0x0304;
generate_register_bitfields!(
    CRU_CLKSEL_CON01,
    u32,
    [
        (clk_matrix_150m_src_div, 0, 5, []),
        (clk_matrix_150m_src_sel, 5, 1, []),
        (clk_matrix_200m_src_div, 6, 5, []),
        (clk_matrix_200m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON02  0x0308;
generate_register_bitfields!(
    CRU_CLKSEL_CON02,
    u32,
    [
        (clk_matrix_250m_src_div, 0, 5, []),
        (clk_matrix_250m_src_sel, 5, 1, []),
        (clk_matrix_300m_src_div, 6, 5, []),
        (clk_matrix_300m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON03  0x030C;
generate_register_bitfields!(
    CRU_CLKSEL_CON03,
    u32,
    [
        (clk_matrix_350m_src_div, 0, 5, []),
        (clk_matrix_350m_src_sel, 5, 1, []),
        (clk_matrix_400m_src_div, 6, 5, []),
        (clk_matrix_400m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON04  0x0310;
generate_register_bitfields!(
    CRU_CLKSEL_CON04,
    u32,
    [
        (clk_matrix_450m_src_div, 0, 5, []),
        (clk_matrix_450m_src_sel, 5, 1, []),
        (clk_matrix_500m_src_div, 6, 5, []),
        (clk_matrix_500m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON05  0x0314;
generate_register_bitfields!(
    CRU_CLKSEL_CON05,
    u32,
    [
        (clk_matrix_600m_src_div, 0, 5, []),
        (clk_matrix_600m_src_sel, 5, 1, []),
        (clk_matrix_650m_src_div, 6, 5, []),
        (clk_matrix_650m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON06  0x0318;
generate_register_bitfields!(
    CRU_CLKSEL_CON06,
    u32,
    [
        (clk_matrix_700m_src_div, 0, 5, []),
        (clk_matrix_700m_src_sel, 5, 1, []),
        (clk_matrix_800m_src_div, 6, 5, []),
        (clk_matrix_800m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON07  0x031C;
generate_register_bitfields!(
    CRU_CLKSEL_CON07,
    u32,
    [
        (clk_matrix_1000m_src_div, 0, 5, []),
        (clk_matrix_1000m_src_sel, 5, 1, []),
        (clk_matrix_1200m_src_div, 6, 5, []),
        (clk_matrix_1200m_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON08  0x0320;
generate_register_bitfields!(
    CRU_CLKSEL_CON08,
    u32,
    [
        (aclk_top_root_div, 0, 5, []),
        (aclk_top_root_sel, 5, 2, []),
        (pclk_top_root_sel, 7, 2, []),
        (aclk_low_top_root_div, 9, 5, []),
        (aclk_low_top_root_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON09  0x0324;
generate_register_bitfields!(
    CRU_CLKSEL_CON09,
    u32,
    [
        (aclk_top_m300_root_sel, 0, 2, []),
        (aclk_top_m500_root_sel, 2, 2, []),
        (aclk_top_m400_root_sel, 4, 2, []),
        (aclk_top_s200_root_sel, 6, 2, []),
        (aclk_top_s400_root_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON10  0x0328;
generate_register_bitfields!(
    CRU_CLKSEL_CON10,
    u32,
    [
        (clk_testout_top_div, 0, 6, []),
        (clk_testout_top_sel, 6, 3, []),
        (clk_testout_sel, 9, 3, []),
        (clk_testout_grp0_sel, 12, 3, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON15  0x033C;
generate_register_bitfields!(
    CRU_CLKSEL_CON15,
    u32,
    [
        (mclk_gmac0_out_div, 0, 7, []),
        (mclk_gmac0_out_sel, 7, 1, []),
        (refclko25m_eth0_out_div, 8, 7, []),
        (refclko25m_eth0_out_sel, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON16  0x0340;
generate_register_bitfields!(
    CRU_CLKSEL_CON16,
    u32,
    [
        (refclko25m_eth1_out_div, 0, 7, []),
        (refclko25m_eth1_out_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON17  0x0344;
generate_register_bitfields!(
    CRU_CLKSEL_CON17,
    u32,
    [
        (clk_cifout_out_div, 0, 8, []),
        (clk_cifout_out_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON18  0x0348;
generate_register_bitfields!(
    CRU_CLKSEL_CON18,
    u32,
    [
        (clk_mipi_camaraout_m0_div, 0, 8, []),
        (clk_mipi_camaraout_m0_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON19  0x034C;
generate_register_bitfields!(
    CRU_CLKSEL_CON19,
    u32,
    [
        (clk_mipi_camaraout_m1_div, 0, 8, []),
        (clk_mipi_camaraout_m1_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON20  0x0350;
generate_register_bitfields!(
    CRU_CLKSEL_CON20,
    u32,
    [
        (clk_mipi_camaraout_m2_div, 0, 8, []),
        (clk_mipi_camaraout_m2_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON21  0x0354;
generate_register_bitfields!(
    CRU_CLKSEL_CON21,
    u32,
    [
        (clk_mipi_camaraout_m3_div, 0, 8, []),
        (clk_mipi_camaraout_m3_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON22  0x0358;
generate_register_bitfields!(
    CRU_CLKSEL_CON22,
    u32,
    [
        (clk_mipi_camaraout_m4_div, 0, 8, []),
        (clk_mipi_camaraout_m4_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON24  0x0360;
generate_register_bitfields!(
    CRU_CLKSEL_CON24,
    u32,
    [
        (hclk_audio_root_sel, 0, 2, []),
        (pclk_audio_root_sel, 2, 2, []),
        (clk_i2s0_8ch_tx_src_div, 4, 5, []),
        (clk_i2s0_8ch_tx_src_sel, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON25  0x0364;
generate_register_bitfields!(
    CRU_CLKSEL_CON25,
    u32,
    [(clk_i2s0_8ch_tx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON26  0x0368;
generate_register_bitfields!(
    CRU_CLKSEL_CON26,
    u32,
    [
        (mclk_i2s0_8ch_tx_sel, 0, 2, []),
        (clk_i2s0_8ch_rx_src_div, 2, 5, []),
        (clk_i2s0_8ch_rx_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON27  0x036C;
generate_register_bitfields!(
    CRU_CLKSEL_CON27,
    u32,
    [(clk_i2s0_8ch_rx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON28  0x0370;
generate_register_bitfields!(
    CRU_CLKSEL_CON28,
    u32,
    [
        (mclk_i2s0_8ch_rx_sel, 0, 2, []),
        (i2s0_8ch_mclkout_sel, 2, 2, []),
        (clk_i2s2_2ch_src_div, 4, 5, []),
        (clk_i2s2_2ch_src_sel, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON29  0x0374;
generate_register_bitfields!(CRU_CLKSEL_CON29, u32, [(clk_i2s2_2ch_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON30  0x0378;
generate_register_bitfields!(
    CRU_CLKSEL_CON30,
    u32,
    [
        (mclk_i2s2_2ch_sel, 0, 2, []),
        (i2s2_2ch_mclkout_sel, 2, 1, []),
        (clk_i2s3_2ch_src_div, 3, 5, []),
        (clk_i2s3_2ch_src_sel, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON31  0x037C;
generate_register_bitfields!(CRU_CLKSEL_CON31, u32, [(clk_i2s3_2ch_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON32  0x0380;
generate_register_bitfields!(
    CRU_CLKSEL_CON32,
    u32,
    [
        (mclk_i2s3_2ch_sel, 0, 2, []),
        (i2s3_2ch_mclkout_sel, 2, 1, []),
        (clk_spdif0_src_div, 3, 5, []),
        (clk_spdif0_src_sel, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON33  0x0384;
generate_register_bitfields!(CRU_CLKSEL_CON33, u32, [(clk_spdif0_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON34  0x0388;
generate_register_bitfields!(
    CRU_CLKSEL_CON34,
    u32,
    [
        (mclk_spdif0_sel, 0, 2, []),
        (clk_spdif1_src_div, 2, 5, []),
        (clk_spdif1_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON35  0x038C;
generate_register_bitfields!(CRU_CLKSEL_CON35, u32, [(clk_spdif1_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON36  0x0390;
generate_register_bitfields!(
    CRU_CLKSEL_CON36,
    u32,
    [
        (mclk_spdif1_sel, 0, 2, []),
        (mclk_pdm1_div, 2, 5, []),
        (mclk_pdm1_sel, 7, 2, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON38  0x0398;
generate_register_bitfields!(
    CRU_CLKSEL_CON38,
    u32,
    [
        (aclk_bus_root_div, 0, 5, []),
        (aclk_bus_root_sel, 5, 1, []),
        (clk_i2c1_sel, 6, 1, []),
        (clk_i2c2_sel, 7, 1, []),
        (clk_i2c3_sel, 8, 1, []),
        (clk_i2c4_sel, 9, 1, []),
        (clk_i2c5_sel, 10, 1, []),
        (clk_i2c6_sel, 11, 1, []),
        (clk_i2c7_sel, 12, 1, []),
        (clk_i2c8_sel, 13, 1, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON39  0x039C;
generate_register_bitfields!(
    CRU_CLKSEL_CON39,
    u32,
    [
        (clk_can0_div, 0, 5, []),
        (clk_can0_sel, 5, 1, []),
        (clk_can1_div, 6, 5, []),
        (clk_can1_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON40  0x03A0;
generate_register_bitfields!(
    CRU_CLKSEL_CON40,
    u32,
    [
        (clk_can2_div, 0, 5, []),
        (clk_can2_sel, 5, 1, []),
        (clk_saradc_div, 6, 8, []),
        (clk_saradc_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON41  0x03A4;
generate_register_bitfields!(
    CRU_CLKSEL_CON41,
    u32,
    [
        (clk_tsadc_div, 0, 8, []),
        (clk_tsadc_sel, 8, 1, []),
        (clk_uart1_src_div, 9, 5, []),
        (clk_uart1_src_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON42  0x03A8;
generate_register_bitfields!(CRU_CLKSEL_CON42, u32, [(clk_uart1_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON43  0x03AC;
generate_register_bitfields!(
    CRU_CLKSEL_CON43,
    u32,
    [
        (sclk_uart1_sel, 0, 2, []),
        (clk_uart2_src_div, 2, 5, []),
        (clk_uart2_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON44  0x03B0;
generate_register_bitfields!(CRU_CLKSEL_CON44, u32, [(clk_uart2_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON45  0x03B4;
generate_register_bitfields!(
    CRU_CLKSEL_CON45,
    u32,
    [
        (sclk_uart2_sel, 0, 2, []),
        (clk_uart3_src_div, 2, 5, []),
        (clk_uart3_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON46  0x03B8;
generate_register_bitfields!(CRU_CLKSEL_CON46, u32, [(clk_uart3_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON47  0x03BC;
generate_register_bitfields!(
    CRU_CLKSEL_CON47,
    u32,
    [
        (sclk_uart3_sel, 0, 2, []),
        (clk_uart4_src_div, 2, 5, []),
        (clk_uart4_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON48  0x03C0;
generate_register_bitfields!(CRU_CLKSEL_CON48, u32, [(clk_uart4_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON49  0x03C4;
generate_register_bitfields!(
    CRU_CLKSEL_CON49,
    u32,
    [
        (sclk_uart4_sel, 0, 2, []),
        (clk_uart5_src_div, 2, 5, []),
        (clk_uart5_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON50  0x03C8;
generate_register_bitfields!(CRU_CLKSEL_CON50, u32, [(clk_uart5_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON51  0x03CC;
generate_register_bitfields!(
    CRU_CLKSEL_CON51,
    u32,
    [
        (sclk_uart5_sel, 0, 2, []),
        (clk_uart6_src_div, 2, 5, []),
        (clk_uart6_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON52  0x03D0;
generate_register_bitfields!(CRU_CLKSEL_CON52, u32, [(clk_uart6_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON53  0x03D4;
generate_register_bitfields!(
    CRU_CLKSEL_CON53,
    u32,
    [
        (sclk_uart6_sel, 0, 2, []),
        (clk_uart7_src_div, 2, 5, []),
        (clk_uart7_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON54  0x03D8;
generate_register_bitfields!(CRU_CLKSEL_CON54, u32, [(clk_uart7_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON55  0x03DC;
generate_register_bitfields!(
    CRU_CLKSEL_CON55,
    u32,
    [
        (sclk_uart7_sel, 0, 2, []),
        (clk_uart8_src_div, 2, 5, []),
        (clk_uart8_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON56  0x03E0;
generate_register_bitfields!(CRU_CLKSEL_CON56, u32, [(clk_uart8_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON57  0x03E4;
generate_register_bitfields!(
    CRU_CLKSEL_CON57,
    u32,
    [
        (sclk_uart8_sel, 0, 2, []),
        (clk_uart9_src_div, 2, 5, []),
        (clk_uart9_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON58  0x03E8;
generate_register_bitfields!(CRU_CLKSEL_CON58, u32, [(clk_uart9_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON59  0x03EC;
generate_register_bitfields!(
    CRU_CLKSEL_CON59,
    u32,
    [
        (sclk_uart9_sel, 0, 2, []),
        (clk_spi0_sel, 2, 2, []),
        (clk_spi1_sel, 4, 2, []),
        (clk_spi2_sel, 6, 2, []),
        (clk_spi3_sel, 8, 2, []),
        (clk_spi4_sel, 10, 2, []),
        (clk_pwm1_sel, 12, 2, []),
        (clk_pwm2_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON60  0x03F0;
generate_register_bitfields!(
    CRU_CLKSEL_CON60,
    u32,
    [
        (clk_pwm3_sel, 0, 2, []),
        (clk_bus_timer_root_sel, 2, 1, []),
        (dbclk_gpio1_div, 3, 5, []),
        (dbclk_gpio1_sel, 8, 1, []),
        (dbclk_gpio2_div, 9, 5, []),
        (dbclk_gpio2_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON61  0x03F4;
generate_register_bitfields!(
    CRU_CLKSEL_CON61,
    u32,
    [
        (dbclk_gpio3_div, 0, 5, []),
        (dbclk_gpio3_sel, 5, 1, []),
        (dbclk_gpio4_div, 6, 5, []),
        (dbclk_gpio4_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON62  0x03F8;
generate_register_bitfields!(
    CRU_CLKSEL_CON62,
    u32,
    [
        (dclk_decom_div, 0, 5, []),
        (dclk_decom_sel, 5, 1, []),
        (clk_bisrintf_pllsrc_div, 6, 5, []),
        (reserved, 11, 5, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON63  0x03FC;
generate_register_bitfields!(
    CRU_CLKSEL_CON63,
    u32,
    [
        (clk_testout_ddr01_div, 0, 6, []),
        (clk_testout_ddr01_sel, 6, 1, []),
        (reserved, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON65  0x0404;
generate_register_bitfields!(
    CRU_CLKSEL_CON65,
    u32,
    [
        (clk_testout_ddr23_div, 0, 6, []),
        (clk_testout_ddr23_sel, 6, 1, []),
        (reserved, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON67  0x040C;
generate_register_bitfields!(
    CRU_CLKSEL_CON67,
    u32,
    [
        (aclk_isp1_root_div, 0, 5, []),
        (aclk_isp1_root_sel, 5, 2, []),
        (hclk_isp1_root_sel, 7, 2, []),
        (clk_isp1_core_div, 9, 5, []),
        (clk_isp1_core_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON73  0x0424;
generate_register_bitfields!(
    CRU_CLKSEL_CON73,
    u32,
    [
        (hclk_rknn_root_sel, 0, 2, []),
        (clk_rknn_dsu0_src_t_div, 2, 5, []),
        (clk_rknn_dsu0_src_t_sel, 7, 3, []),
        (clk_testout_npu_div, 10, 5, []),
        (clk_testout_npu_sel, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON74  0x0428;
generate_register_bitfields!(
    CRU_CLKSEL_CON74,
    u32,
    [
        (clk_rknn_dsu0_sel, 0, 1, []),
        (pclk_nputop_root_sel, 1, 2, []),
        (clk_nputimer_root_sel, 3, 1, []),
        (clk_npu_pvtpll_sel, 4, 1, []),
        (hclk_npu_cm0_root_sel, 5, 2, []),
        (clk_npu_cm0_rtc_div, 7, 5, []),
        (clk_npu_cm0_rtc_sel, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON77  0x0434;
generate_register_bitfields!(
    CRU_CLKSEL_CON77,
    u32,
    [
        (hclk_nvm_root_sel, 0, 2, []),
        (aclk_nvm_root_div, 2, 5, []),
        (aclk_nvm_root_sel, 7, 1, []),
        (cclk_emmc_div, 8, 6, []),
        (cclk_emmc_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON78  0x0438;
generate_register_bitfields!(
    CRU_CLKSEL_CON78,
    u32,
    [
        (bclk_emmc_div, 0, 5, []),
        (bclk_emmc_sel, 5, 1, []),
        (sclk_sfc_div, 6, 6, []),
        (sclk_sfc_sel, 12, 2, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON80  0x0440;
generate_register_bitfields!(
    CRU_CLKSEL_CON80,
    u32,
    [
        (pclk_php_root_sel, 0, 2, []),
        (aclk_pcie_root_div, 2, 5, []),
        (aclk_pcie_root_sel, 7, 1, []),
        (aclk_php_root_div, 8, 5, []),
        (aclk_php_root_sel, 13, 1, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON81  0x0444;
generate_register_bitfields!(
    CRU_CLKSEL_CON81,
    u32,
    [
        (clk_gmac0_ptp_ref_div, 0, 6, []),
        (clk_gmac0_ptp_ref_sel, 6, 1, []),
        (clk_gmac1_ptp_ref_div, 7, 6, []),
        (clk_gmac1_ptp_ref_sel, 13, 1, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON82  0x0448;
generate_register_bitfields!(
    CRU_CLKSEL_CON82,
    u32,
    [
        (clk_rxoob0_div, 0, 7, []),
        (clk_rxoob0_sel, 7, 1, []),
        (clk_rxoob1_div, 8, 7, []),
        (clk_rxoob1_sel, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON83  0x044C;
generate_register_bitfields!(
    CRU_CLKSEL_CON83,
    u32,
    [
        (clk_rxoob2_div, 0, 7, []),
        (clk_rxoob2_sel, 7, 1, []),
        (clk_gmac_125m_cru_i_div, 8, 7, []),
        (clk_gmac_125m_cru_i_sel, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON84  0x0450;
generate_register_bitfields!(
    CRU_CLKSEL_CON84,
    u32,
    [
        (clk_gmac_50m_cru_i_div, 0, 7, []),
        (clk_gmac_50m_cru_i_sel, 7, 1, []),
        (clk_utmi_otg2_div, 8, 4, []),
        (clk_utmi_otg2_sel, 12, 2, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON85  0x0454;
generate_register_bitfields!(
    CRU_CLKSEL_CON85,
    u32,
    [
        (clk_gmac0_tx_125m_o_div, 0, 6, []),
        (clk_gmac1_tx_125m_o_div, 6, 6, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON89  0x0464;
generate_register_bitfields!(
    CRU_CLKSEL_CON89,
    u32,
    [
        (hclk_rkvdec0_root_sel, 0, 2, []),
        (aclk_rkvdec0_root_div, 2, 5, []),
        (aclk_rkvdec0_root_sel, 7, 2, []),
        (aclk_rkvdec_ccu_div, 9, 5, []),
        (aclk_rkvdec_ccu_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON90  0x0468;
generate_register_bitfields!(
    CRU_CLKSEL_CON90,
    u32,
    [
        (clk_rkvdec0_ca_div, 0, 5, []),
        (clk_rkvdec0_ca_sel, 5, 1, []),
        (clk_rkvdec0_hevc_ca_div, 6, 5, []),
        (clk_rkvdec0_hevc_ca_sel, 11, 2, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON91  0x046C;
generate_register_bitfields!(
    CRU_CLKSEL_CON91,
    u32,
    [
        (clk_rkvdec0_core_div, 0, 5, []),
        (clk_rkvdec0_core_sel, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON93  0x0474;
generate_register_bitfields!(
    CRU_CLKSEL_CON93,
    u32,
    [
        (hclk_rkvdec1_root_sel, 0, 2, []),
        (aclk_rkvdec1_root_div, 2, 5, []),
        (aclk_rkvdec1_root_sel, 7, 2, []),
        (clk_rkvdec1_ca_div, 9, 5, []),
        (clk_rkvdec1_ca_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON94  0x0478;
generate_register_bitfields!(
    CRU_CLKSEL_CON94,
    u32,
    [
        (clk_rkvdec1_hevc_ca_div, 0, 5, []),
        (clk_rkvdec1_hevc_ca_sel, 5, 2, []),
        (clk_rkvdec1_core_div, 7, 5, []),
        (clk_rkvdec1_core_sel, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON96  0x0480;
generate_register_bitfields!(
    CRU_CLKSEL_CON96,
    u32,
    [
        (aclk_usb_root_div, 0, 5, []),
        (aclk_usb_root_sel, 5, 1, []),
        (hclk_usb_root_sel, 6, 2, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON98  0x0488;
generate_register_bitfields!(
    CRU_CLKSEL_CON98,
    u32,
    [
        (aclk_vdpu_root_div, 0, 5, []),
        (aclk_vdpu_root_sel, 5, 2, []),
        (aclk_vdpu_low_root_sel, 7, 2, []),
        (hclk_vdpu_root_sel, 9, 2, []),
        (reserved, 11, 5, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON99  0x048C;
generate_register_bitfields!(
    CRU_CLKSEL_CON99,
    u32,
    [
        (aclk_jpeg_decoder_root_div, 0, 5, []),
        (aclk_jpeg_decoder_root_sel, 5, 2, []),
        (resclk_iep2p0_core_diverved, 7, 5, []),
        (clk_iep2p0_core_sel, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON100  0x0490;
generate_register_bitfields!(
    CRU_CLKSEL_CON100,
    u32,
    [
        (clk_rga2_core_div, 0, 5, []),
        (clk_rga2_core_sel, 5, 3, []),
        (clk_rga3_0_core_div, 8, 5, []),
        (clk_rga3_0_core_sel, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON102  0x0498;
generate_register_bitfields!(
    CRU_CLKSEL_CON102,
    u32,
    [
        (hclk_rkvenc0_root_sel, 0, 2, []),
        (aclk_rkvenc0_root_div, 2, 5, []),
        (aclk_rkvenc0_root_sel, 7, 2, []),
        (clk_rkvenc0_core_div, 9, 5, []),
        (clk_rkvenc0_core_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON104  0x04A0;
generate_register_bitfields!(
    CRU_CLKSEL_CON104,
    u32,
    [
        (hclk_rkvenc1_root_sel, 0, 2, []),
        (aclk_rkvenc1_root_div, 2, 5, []),
        (aclk_rkvenc1_root_sel, 7, 2, []),
        (clk_rkvenc1_core_div, 9, 5, []),
        (clk_rkvenc1_core_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON106  0x04A8;
generate_register_bitfields!(
    CRU_CLKSEL_CON106,
    u32,
    [
        (aclk_vi_root_div, 0, 5, []),
        (aclk_vi_root_sel, 5, 3, []),
        (hclk_vi_root_sel, 8, 2, []),
        (pclk_vi_root_sel, 10, 2, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON107  0x04AC;
generate_register_bitfields!(
    CRU_CLKSEL_CON107,
    u32,
    [
        (dclk_vicap_div, 0, 5, []),
        (dclk_vicap_sel, 5, 1, []),
        (clk_isp0_core_div, 6, 5, []),
        (clk_isp0_core_sel, 11, 2, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON108  0x04B0;
generate_register_bitfields!(
    CRU_CLKSEL_CON108,
    u32,
    [
        (clk_fisheye0_core_div, 0, 5, []),
        (clk_fisheye0_core_sel, 5, 2, []),
        (clk_fisheye1_core_div, 7, 5, []),
        (clk_fisheye1_core_sel, 12, 2, []),
        (iclk_csihost01_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON110  0x04B8;
generate_register_bitfields!(
    CRU_CLKSEL_CON110,
    u32,
    [
        (aclk_vop_root_div, 0, 5, []),
        (aclk_vop_root_sel, 5, 3, []),
        (aclk_vop_low_root_sel, 8, 2, []),
        (hclk_vop_root_sel, 10, 2, []),
        (pclk_vop_root_sel, 12, 2, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON111  0x04BC;
generate_register_bitfields!(
    CRU_CLKSEL_CON111,
    u32,
    [
        (dclk_vp0_src_div, 0, 7, []),
        (dclk_vp0_src_sel, 7, 2, []),
        (dclk_vp1_src_div, 9, 5, []),
        (dclk_vp1_src_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON112  0x04C0;
generate_register_bitfields!(
    CRU_CLKSEL_CON112,
    u32,
    [
        (dclk_vp2_src_div, 0, 5, []),
        (dclk_vp2_src_sel, 5, 2, []),
        (dclk_vp0_sel, 7, 2, []),
        (dclk_vp1_sel, 9, 2, []),
        (dclk_vp2_sel, 11, 2, []),
        (reserved, 13, 13, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON113  0x04C4;
generate_register_bitfields!(
    CRU_CLKSEL_CON113,
    u32,
    [
        (dclk_vp3_div, 0, 7, []),
        (dclk_vp3_sel, 7, 2, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON114  0x04C8;
generate_register_bitfields!(
    CRU_CLKSEL_CON114,
    u32,
    [
        (clk_dsihost0_div, 0, 7, []),
        (clk_dsihost0_sel, 7, 2, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON115  0x04CC;
generate_register_bitfields!(
    CRU_CLKSEL_CON115,
    u32,
    [
        (clk_dsihost1_div, 0, 7, []),
        (clk_dsihost1_sel, 7, 2, []),
        (aclk_vop_sub_src_sel, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON116  0x04D0;
generate_register_bitfields!(
    CRU_CLKSEL_CON116,
    u32,
    [
        (aclk_vo0_root_div, 0, 5, []),
        (aclk_vo0_root_sel, 5, 1, []),
        (hclk_vo0_root_sel, 6, 2, []),
        (hclk_vo0_s_root_sel, 8, 2, []),
        (pclk_vo0_root_sel, 10, 2, []),
        (pclk_vo0_s_root_sel, 12, 2, []),
        (reserved, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON117  0x04D4;
generate_register_bitfields!(
    CRU_CLKSEL_CON117,
    u32,
    [
        (clk_aux16mhz_0_div, 0, 8, []),
        (clk_aux16mhz_1_div, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON118  0x04D8;
generate_register_bitfields!(
    CRU_CLKSEL_CON118,
    u32,
    [
        (clk_i2s4_8ch_tx_src_div, 0, 5, []),
        (clk_i2s4_8ch_tx_src_sel, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON119  0x04DC;
generate_register_bitfields!(
    CRU_CLKSEL_CON119,
    u32,
    [(clk_i2s4_8ch_tx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON120  0x04E0;
generate_register_bitfields!(
    CRU_CLKSEL_CON120,
    u32,
    [
        (mclk_i2s4_8ch_tx_sel, 0, 2, []),
        (reserved0, 2, 1, []),
        (clk_i2s8_8ch_tx_src_div, 3, 5, []),
        (clk_i2s8_8ch_tx_src_sel, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON121  0x04E4;
generate_register_bitfields!(
    CRU_CLKSEL_CON121,
    u32,
    [(clk_i2s8_8ch_tx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON122  0x04E8;
generate_register_bitfields!(
    CRU_CLKSEL_CON122,
    u32,
    [
        (mclk_i2s8_8ch_tx_sel, 0, 2, []),
        (reserved0, 2, 1, []),
        (clk_spdif2_dp0_src_div, 3, 5, []),
        (clk_spdif2_dp0_src_sel, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON123  0x04EC;
generate_register_bitfields!(
    CRU_CLKSEL_CON123,
    u32,
    [(clk_spdif2_dp0_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON124  0x04F0;
generate_register_bitfields!(
    CRU_CLKSEL_CON124,
    u32,
    [
        (resmclk_4x_spdif2_dp0_selerved, 0, 2, []),
        (clk_spdif5_dp1_src_div, 2, 5, []),
        (clk_spdif5_dp1_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON125  0x04F4;
generate_register_bitfields!(
    CRU_CLKSEL_CON125,
    u32,
    [(clk_spdif5_dp1_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON126  0x04F8;
generate_register_bitfields!(
    CRU_CLKSEL_CON126,
    u32,
    [
        (mclk_4x_spdif5_dp1_sel, 0, 2, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON128  0x0500;
generate_register_bitfields!(
    CRU_CLKSEL_CON128,
    u32,
    [
        (aclk_hdcp1_root_div, 0, 5, []),
        (aclk_hdcp1_root_sel, 5, 2, []),
        (aclk_hdmirx_root_div, 7, 5, []),
        (aclk_hdmirx_root_sel, 12, 1, []),
        (hclk_vo1_root_sel, 13, 2, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON129  0x0504;
generate_register_bitfields!(
    CRU_CLKSEL_CON129,
    u32,
    [
        (hclk_vo1_s_root_sel, 0, 2, []),
        (pclk_vo1_root_sel, 2, 2, []),
        (pclk_vo1_s_root_sel, 4, 2, []),
        (clk_i2s7_8ch_rx_src_div, 6, 5, []),
        (clk_i2s7_8ch_rx_src_sel, 11, 1, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON130  0x0508;
generate_register_bitfields!(
    CRU_CLKSEL_CON130,
    u32,
    [(clk_i2s7_8ch_rx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON131  0x050C;
generate_register_bitfields!(
    CRU_CLKSEL_CON131,
    u32,
    [
        (mclk_i2s7_8ch_rx_sel, 0, 2, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON133  0x0514;
generate_register_bitfields!(
    CRU_CLKSEL_CON133,
    u32,
    [
        (reserved0, 0, 1, []),
        (clk_hdmitx0_earc_div, 1, 5, []),
        (clk_hdmitx0_earc_sel, 6, 1, []),
        (reserved1, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON136  0x0520;
generate_register_bitfields!(
    CRU_CLKSEL_CON136,
    u32,
    [
        (reserved0, 0, 1, []),
        (clk_hdmitx1_earc_div, 1, 5, []),
        (clk_hdmitx1_earc_sel, 6, 1, []),
        (reserved1, 7, 9, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON138  0x0528;
generate_register_bitfields!(
    CRU_CLKSEL_CON138,
    u32,
    [
        (clk_hdmirx_aud_src_div, 0, 8, []),
        (clk_hdmirx_aud_src_sel, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON139  0x052C;
generate_register_bitfields!(
    CRU_CLKSEL_CON139,
    u32,
    [(clk_hdmirx_aud_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON140  0x0530;
generate_register_bitfields!(
    CRU_CLKSEL_CON140,
    u32,
    [
        (clk_hdmirx_aud_sel, 0, 1, []),
        (clk_edp0_200m_sel, 1, 2, []),
        (clk_edp1_200m_sel, 3, 2, []),
        (clk_i2s5_8ch_tx_src_div, 5, 5, []),
        (clk_i2s5_8ch_tx_src_sel, 10, 1, []),
        (reserved, 11, 5, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON141  0x0534;
generate_register_bitfields!(
    CRU_CLKSEL_CON141,
    u32,
    [(clk_i2s5_8ch_tx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON142  0x0538;
generate_register_bitfields!(
    CRU_CLKSEL_CON142,
    u32,
    [
        (mclk_i2s5_8ch_tx_sel, 0, 2, []),
        (reserved, 2, 14, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON144  0x0540;
generate_register_bitfields!(
    CRU_CLKSEL_CON144,
    u32,
    [
        (reserved0, 0, 3, []),
        (clk_i2s6_8ch_tx_src_div, 3, 5, []),
        (clk_i2s6_8ch_tx_src_sel, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON145  0x0544;
generate_register_bitfields!(
    CRU_CLKSEL_CON145,
    u32,
    [(clk_i2s6_8ch_tx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON146  0x0548;
generate_register_bitfields!(
    CRU_CLKSEL_CON146,
    u32,
    [
        (mclk_i2s6_8ch_tx_sel, 0, 2, []),
        (clk_i2s6_8ch_rx_src_div, 2, 5, []),
        (clk_i2s6_8ch_rx_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON147  0x054C;
generate_register_bitfields!(
    CRU_CLKSEL_CON147,
    u32,
    [(clk_i2s6_8ch_rx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON148  0x0550;
generate_register_bitfields!(
    CRU_CLKSEL_CON148,
    u32,
    [
        (mclk_i2s6_8ch_rx_sel, 0, 2, []),
        (i2s6_8ch_mclkout_sel, 2, 2, []),
        (clk_spdif3_src_div, 4, 5, []),
        (clk_spdif3_src_sel, 9, 1, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON149  0x0554;
generate_register_bitfields!(CRU_CLKSEL_CON149, u32, [(clk_spdif3_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON150  0x0558;
generate_register_bitfields!(
    CRU_CLKSEL_CON150,
    u32,
    [
        (mclk_spdif3_sel, 0, 2, []),
        (clk_spdif4_src_div, 2, 5, []),
        (clk_spdif4_src_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON151  0x055C;
generate_register_bitfields!(CRU_CLKSEL_CON151, u32, [(clk_spdif4_frac_div, 0, 32, []),]);

// CRU_CLKSEL_CON152  0x0560;
generate_register_bitfields!(
    CRU_CLKSEL_CON152,
    u32,
    [
        (mclk_spdif4_sel, 0, 2, []),
        (mclk_spdifrx0_div, 2, 5, []),
        (mclk_spdifrx0_sel, 7, 2, []),
        (mclk_spdifrx1_div, 9, 5, []),
        (mclk_spdifrx1_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON153  0x0564;
generate_register_bitfields!(
    CRU_CLKSEL_CON153,
    u32,
    [
        (mclk_spdifrx2_div, 0, 5, []),
        (mclk_spdifrx2_sel, 5, 2, []),
        (clk_i2s9_8ch_rx_src_div, 7, 5, []),
        (clk_i2s9_8ch_rx_src_sel, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON154  0x0568;
generate_register_bitfields!(
    CRU_CLKSEL_CON154,
    u32,
    [(clk_i2s9_8ch_rx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON155  0x056C;
generate_register_bitfields!(
    CRU_CLKSEL_CON155,
    u32,
    [
        (mclk_i2s9_8ch_rx_sel, 0, 2, []),
        (reserved0, 2, 1, []),
        (clk_i2s10_8ch_rx_src_div, 3, 5, []),
        (clk_i2s10_8ch_rx_src_sel, 8, 1, []),
        (reserved1, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON156  0x0570;
generate_register_bitfields!(
    CRU_CLKSEL_CON156,
    u32,
    [(clk_i2s10_8ch_rx_frac_div, 0, 32, []),]
);

// CRU_CLKSEL_CON157  0x0574;
generate_register_bitfields!(
    CRU_CLKSEL_CON157,
    u32,
    [
        (mclk_i2s10_8ch_rx_sel, 0, 2, []),
        (clk_hdmitrx_refsrc_div, 2, 5, []),
        (clk_hdmitrx_refsrc_sel, 7, 1, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON158  0x0578;
generate_register_bitfields!(
    CRU_CLKSEL_CON158,
    u32,
    [
        (clk_gpu_src_t_div, 0, 5, []),
        (clk_gpu_src_t_sel, 5, 3, []),
        (clk_testout_gpu_div, 8, 5, []),
        (clk_testout_gpu_sel, 13, 1, []),
        (clk_gpu_src_sel, 14, 1, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON159  0x057C;
generate_register_bitfields!(
    CRU_CLKSEL_CON159,
    u32,
    [
        (clk_gpu_stacks_div, 0, 5, []),
        (aclk_s_gpu_biu_div, 5, 5, []),
        (aclk_m0_gpu_biu_div, 10, 5, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON160  0x0580;
generate_register_bitfields!(
    CRU_CLKSEL_CON160,
    u32,
    [
        (aclk_m1_gpu_biu_div, 0, 5, []),
        (aclk_m2_gpu_biu_div, 5, 5, []),
        (aclk_m3_gpu_biu_div, 10, 5, []),
        (reserved, 15, 1, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON161  0x0584;
generate_register_bitfields!(
    CRU_CLKSEL_CON161,
    u32,
    [
        (pclk_gpu_root_sel, 0, 2, []),
        (clk_gpu_pvtpll_sel, 2, 1, []),
        (reserved, 3, 13, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON163  0x058C;
generate_register_bitfields!(
    CRU_CLKSEL_CON163,
    u32,
    [
        (aclk_av1_root_div, 0, 7, []),
        (aclk_av1_root_sel, 5, 2, []),
        (pclk_av1_root_sel, 7, 2, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON165  0x0594;
generate_register_bitfields!(
    CRU_CLKSEL_CON165,
    u32,
    [
        (aclk_center_root_sel, 0, 2, []),
        (aclk_center_low_root_sel, 2, 2, []),
        (hclk_center_root_sel, 4, 2, []),
        (pclk_center_root_sel, 6, 2, []),
        (aclk_center_s200_root_sel, 8, 2, []),
        (aclk_center_s400_root_sel, 10, 2, []),
        (clk_ddr_timer_root_sel, 12, 1, []),
        (reserved, 13, 3, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON166  0x0598;
generate_register_bitfields!(
    CRU_CLKSEL_CON166,
    u32,
    [
        (clk_ddr_cm0_rtc_div, 0, 5, []),
        (clk_ddr_cm0_rtc_sel, 5, 1, []),
        (reserved, 6, 10, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON170  0x05A8;
generate_register_bitfields!(
    CRU_CLKSEL_CON170,
    u32,
    [
        (aclk_vo1usb_top_root_div, 0, 5, []),
        (aclk_vo1usb_top_root_sel, 5, 1, []),
        (hclk_vo1usb_top_root_sel, 6, 2, []),
        (reserved, 8, 8, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON172  0x05B0;
generate_register_bitfields!(
    CRU_CLKSEL_CON172,
    u32,
    [
        (hclk_sdio_root_sel, 0, 2, []),
        (cclk_src_sdio_div, 2, 6, []),
        (cclk_src_sdio_sel, 8, 2, []),
        (reserved, 10, 6, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON174  0x05B8;
generate_register_bitfields!(
    CRU_CLKSEL_CON174,
    u32,
    [
        (aclk_rga3_root_div, 0, 5, []),
        (aclk_rga3_root_sel, 5, 2, []),
        (hclk_rga3_root_sel, 7, 2, []),
        (clk_rga3_1_core_div, 9, 5, []),
        (clk_rga3_1_core_sel, 14, 2, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON176  0x05C0;
generate_register_bitfields!(
    CRU_CLKSEL_CON176,
    u32,
    [
        (clk_ref_pipe_phy0_pll_src_div, 0, 6, []),
        (clk_ref_pipe_phy1_pll_src_div, 6, 6, []),
        (reserved, 12, 4, []),
        (write_enable, 16, 16, []),
    ]
);

// CRU_CLKSEL_CON177  0x05C4;
generate_register_bitfields!(
    CRU_CLKSEL_CON177,
    u32,
    [
        (clk_ref_pipe_phy2_pll_src_div, 0, 6, []),
        (clk_ref_pipe_phy0_sel, 6, 1, []),
        (clk_ref_pipe_phy1_sel, 7, 1, []),
        (clk_ref_pipe_phy2_sel, 8, 1, []),
        (reserved, 9, 7, []),
        (write_enable, 16, 16, []),
    ]
);
