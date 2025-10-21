# RK3588 CRU 驱动库

## 项目简介

RK3588 CRU (Clock and Reset Unit) 驱动库是一个专为 RK3588 芯片设计的 Rust 时钟控制单元驱动库。该库提供了完整的时钟管理功能，包括 MMC 存储控制器时钟配置、NPU (神经网络处理单元) 时钟管理以及时钟门控等功能。

本项目采用 `no_std` 设计，完全适用于裸机和嵌入式环境，特别针对 U-Boot 引导加载程序环境进行了优化。通过基于 `tock-registers` 的类型安全寄存器访问，确保了硬件操作的可靠性和安全性。

## 功能特性

- **完整的 MMC 时钟支持**: 支持 EMMC、SDIO、SFC 等存储控制器的时钟配置和频率管理
- **NPU 时钟管理**: 提供全面的 NPU 时钟控制，包括频率设置、门控使能和状态监控
- **时钟门控功能**: 精确控制各个模块的时钟使能/禁用状态，优化功耗管理
- **类型安全寄存器访问**: 基于 `tock-registers` 提供类型安全的硬件寄存器操作
- **no_std 兼容**: 完全不依赖标准库，适用于裸机和嵌入式环境
- **ARM64 架构优化**: 专门针对 RK3588 ARM64 平台进行优化
- **U-Boot 环境支持**: 在 U-Boot 引导环境下提供稳定可靠的时钟管理功能
- **丰富的时钟源支持**: 支持 PLL、OSC 等多种时钟源和灵活的分频配置

## 快速开始

### 环境要求

- Rust 2024 Edition
- ARM64 开发环境
- 支持 U-Boot 的 RK3588 硬件平台
- ostool 工具 (用于测试)

### 安装步骤

1. 安装 `ostool` 依赖工具：

```bash
cargo install ostool
```

2. 将项目添加到 `Cargo.toml`：

```toml
[dependencies]
rk3588-clk = "0.1.0"
```

### 基本使用

```rust
use rk3588_clk::{Rk3588Cru, constant::*};
use core::ptr::NonNull;

// 创建 CRU 实例
let cru_addr = 0xfd7c0000; // RK3588 CRU 基地址
let cru = Rk3588Cru::new(NonNull::new(cru_addr as *mut u8).unwrap());

// 初始化 CRU
cru.init();

// 配置 EMMC 时钟频率
let emmc_rate = 200_000_000; // 200MHz
match cru.mmc_set_clk(CCLK_EMMC, emmc_rate) {
    Ok(actual_rate) => println!("EMMC 时钟设置为: {} Hz", actual_rate),
    Err(_) => println!("EMMC 时钟设置失败"),
}

// 使能 NPU 时钟门控
match cru.npu_gate_enable(ACLK_NPU0) {
    Ok(enabled) => println!("NPU ACLK0 门控状态: {}", enabled),
    Err(e) => println!("NPU 门控使能失败: {}", e),
}
```

## 项目结构

```
src/
├── lib.rs              # 主入口和 Rk3588Cru 结构体定义
├── autocs.rs           # 自动时钟选择功能
├── clksel.rs           # 时钟选择寄存器定义
├── constant.rs         # 硬件常量和时钟 ID 定义
├── gate.rs             # 时钟门控制寄存器
├── pll.rs              # PLL 锁相环控制寄存器
├── softrst.rs          # 软件复位寄存器
└── tools.rs            # 工具函数 (分频计算等)

tests/
└── test.rs             # 集成测试，包含 MMC 和 NPU 功能测试
```

## API 文档

### 核心结构体

- **`Rk3588Cru`**: 主要的 CRU 接口结构体，提供所有时钟控制功能
- **`Rk3588CruRegisters`**: CRU 寄存器映射结构体，包含所有硬件寄存器定义

### 主要接口

#### MMC 时钟控制

- `Rk3588Cru::new(addr)`: 创建新的 CRU 实例
- `Rk3588Cru::init()`: 初始化 CRU
- `Rk3588Cru::mmc_get_clk(clk_id)`: 获取指定 MMC 时钟的当前频率
- `Rk3588Cru::mmc_set_clk(clk_id, rate)`: 设置指定 MMC 时钟的频率

**支持的 MMC 时钟 ID:**
- `CCLK_EMMC`: EMMC 控制器时钟
- `CCLK_SRC_SDIO`: SDIO 控制器时钟
- `SCLK_SFC`: SFC (SPI Flash Controller) 时钟
- `BCLK_EMMC`: EMMC 总线时钟

#### NPU 时钟管理

- `Rk3588Cru::npu_get_clk(clk_id)`: 获取 NPU 时钟频率
- `Rk3588Cru::npu_set_clk(clk_id, rate)`: 设置 NPU 时钟频率
- `Rk3588Cru::npu_gate_enable(gate_id)`: 使能 NPU 时钟门控
- `Rk3588Cru::npu_gate_disable(gate_id)`: 禁用 NPU 时钟门控
- `Rk3588Cru::npu_gate_status(gate_id)`: 查询 NPU 时钟门控状态

**支持的 NPU 时钟 ID:**
- `HCLK_NPU_ROOT`: NPU 根时钟
- `CLK_NPU_DSU0`: NPU DSU0 时钟
- `PCLK_NPU_ROOT`: NPU 外设时钟
- `CLK_NPUTIMER_ROOT`: NPU 定时器时钟

**支持的 NPU 门控 ID:**
- `ACLK_NPU0/1/2`: NPU 各模块 ACLK
- `HCLK_NPU0/1/2`: NPU 各模块 HCLK
- `PCLK_NPU_*`: NPU 外设时钟
- `CLK_NPUTIMER*`: NPU 定时器时钟

## 使用示例

### MMC 时钟控制示例

```rust
use rk3588_clk::{Rk3588Cru, constant::*};
use core::ptr::NonNull;

fn configure_emmc_clock(cru: &Rk3588Cru) -> Result<(), &'static str> {

    // 设置 EMMC 时钟为 200MHz
    let target_rate = 200_000_000;
    match cru.mmc_set_clk(CCLK_EMMC, target_rate) {
        Ok(actual_rate) => {
            println!("EMMC 时钟设置成功: {} Hz", actual_rate);

            // 验证时钟设置
            match cru.mmc_get_clk(CCLK_EMMC) {
                Ok(read_rate) => {
                    println!("EMMC 时钟读取: {} Hz", read_rate);
                    if read_rate == actual_rate {
                        println!("时钟设置验证成功");
                    }
                }
                Err(e) => return Err("时钟读取失败"),
            }
        }
        Err(e) => return Err("时钟设置失败"),
    }

    Ok(())
}
```

### NPU 时钟管理示例

```rust
use rk3588_clk::{Rk3588Cru, constant::*};
use core::ptr::NonNull;

fn configure_npu_clocks(cru: &Rk3588Cru) -> Result<(), &'static str> {

    // 使能 NPU 相关的时钟门控
    let npu_gates = [
        ACLK_NPU0, HCLK_NPU0,
        ACLK_NPU1, HCLK_NPU1,
        ACLK_NPU2, HCLK_NPU2,
        PCLK_NPU_GRF, PCLK_NPU_TIMER,
    ];

    for &gate_id in &npu_gates {
        match cru.npu_gate_enable(gate_id) {
            Ok(enabled) => {
                println!("门控 {} 使能状态: {}", gate_id, enabled);
                if !enabled {
                    return Err("门控使能失败");
                }
            }
            Err(e) => return Err("门控操作失败"),
        }
    }

    // 设置 NPU 根时钟为 200MHz
    match cru.npu_set_clk(HCLK_NPU_ROOT, 200_000_000) {
        Ok(actual_rate) => {
            println!("NPU 根时钟设置: {} Hz", actual_rate);
        }
        Err(e) => return Err("NPU 时钟设置失败"),
    }

    // 设置 NPU DSU0 时钟为 500MHz
    match cru.npu_set_clk(CLK_NPU_DSU0, 500_000_000) {
        Ok(actual_rate) => {
            println!("NPU DSU0 时钟设置: {} Hz", actual_rate);
        }
        Err(e) => return Err("NPU DSU0 时钟设置失败"),
    }

    println!("NPU 时钟配置完成");
    Ok(())
}
```

### 完整使用示例

```rust
use rk3588_clk::{Rk3588Cru, constant::*};
use core::ptr::NonNull;

fn main() -> Result<(), &'static str> {
    // 初始化 CRU
    let cru_addr = 0xfd7c0000;
    let cru = Rk3588Cru::new(NonNull::new(cru_addr as *mut u8).unwrap());
    cru.init();

    // 配置存储时钟
    println!("配置存储时钟...");
    if let Err(e) = configure_emmc_clock(&cru) {
        println!("存储时钟配置失败: {}", e);
        return Err(e);
    }

    // 配置 NPU 时钟
    println!("配置 NPU 时钟...");
    if let Err(e) = configure_npu_clocks(&cru) {
        println!("NPU 时钟配置失败: {}", e);
        return Err(e);
    }

    // 运行系统时钟诊断
    println!("系统时钟诊断:");
    if let Err(e) = clock_diagnostics(&cru) {
        println!("时钟诊断失败: {}", e);
        return Err(e);
    }

    Ok(())
}

fn clock_diagnostics(cru: &Rk3588Cru) -> Result<(), &'static str> {
    // 检查关键时钟状态
    let critical_clocks = [
        (CCLK_EMMC, "EMMC"),
        (HCLK_NPU_ROOT, "NPU_ROOT"),
        (CLK_NPU_DSU0, "NPU_DSU0"),
    ];

    for &(clk_id, name) in &critical_clocks {
        match cru.npu_get_clk(clk_id) {
            Ok(rate) => println!("{} 时钟: {} Hz", name, rate),
            Err(_) => println!("{} 时钟读取失败", name),
        }
    }

    Ok(())
}
```

## 测试结果

### 运行测试

#### 带U-Boot环境的硬件测试

```bash
# 带uboot的开发板测试
cargo test --test test -- tests --show-output --uboot
```

#### 本地模拟测试

```bash
# 本地测试（不依赖 U-Boot）
cargo test --test test -- tests --show-output
```

### 测试输出示例

```
     _____                                         __
    / ___/ ____   ____ _ _____ _____ ___   ____ _ / /
    \__ \ / __ \ / __ `// ___// ___// _ \ / __ `// / 
   ___/ // /_/ // /_/ // /   / /   /  __// /_/ // /  
  /____// .___/ \__,_//_/   /_/    \___/ \__,_//_/   
       /_/                                           

Version                       : 0.12.2
Platfrom                      : RK3588 OPi 5 Plus
Start CPU                     : 0x0
FDT                           : 0xffff900000f37000
🐛 0.000ns    [sparreal_kernel::driver:16] add registers
🐛 0.000ns    [rdrive::probe::fdt:168] Probe [interrupt-controller@fe600000]->[GICv3]
 0.000ns    [somehal::arch::mu:181] Map `iomap       `: RW- ffff9000fe600000, 0xffff9000fe61 [0xfe600000, 0xfe610000)
🐛 0.000ns    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fe680000, 0xffff9000fe780000) -> [0xfe680000, 0xfe780000)
🐛 0.000ns    [rdrive::probe::fdt:168] Probe [timer]->[ARMv8 Timer]
🐛 0.000ns    [sparreal_rt::arch::timer:78] ARMv8 Timer IRQ: IrqConfig { irq: 0x1e, trigger: LevelHigh, is_p: true }
🐛 0.000ns    [rdrive::probe::fdt:168] Probe [psci]->[ARM PSCI]
🐛 0.000ns    [sparreal_rt::arch::power:76] PCSI [Smc]
🐛 0.000ns    [sparreal_kernel::irq:39] [GICv3](405) open
🔍 0.000ns    [arm_gic_driver::version::v3:342] Initializing GICv3 Distributor@0xffff9000fe600000, security state: NonSecure...
🔍 0.000ns    [arm_gic_driver::version::v3:356] GICv3 Distributor disabled
🔍 0.000ns    [arm_gic_driver::version::v3:865] CPU interface initialization for CPU: 0x0
🔍 0.000ns    [arm_gic_driver::version::v3:921] CPU interface initialized successfully
🐛 0.000ns    [sparreal_kernel::irq:64] [GICv3](405) init cpu: CPUHardId(0)
🐛 0.000ns    [sparreal_rt::arch::timer:30] ARMv8 Timer: Enabled
🐛 18.117s    [sparreal_kernel::irq:136] Enable irq 0x1e on chip 405
🐛 18.118s    [sparreal_kernel::hal_al::run:33] Driver initialized
🐛 18.740s    [rdrive:132] probe pci devices
begin test
Run test: test_platform
💡 18.794s    [test::tests:338] Found node: mmc@fe2e0000
💡 18.795s    [test::tests:343] Syscon address range: 0xfe2e0000 - 0xfe2f0000
💡 18.795s    [test::tests:346] Aligned Syscon address range: 0xfe2e0000 - 0xfe2f0000
🐛 18.796s    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fe2e0000, 0xffff9000fe2f0000) -> [0xfe2e0000, 0xfe2f0000)
💡 18.826s    [test::tests:338] Found node: clock-controller@fd7c0000
💡 18.827s    [test::tests:343] Syscon address range: 0xfd7c0000 - 0xfd81c000
💡 18.828s    [test::tests:346] Aligned Syscon address range: 0xfd7c0000 - 0xfd81c000
🐛 18.828s    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fd7c0000, 0xffff9000fd81c000) -> [0xfd7c0000, 0xfd81c000)
💡 18.857s    [test::tests:338] Found node: syscon@fd5a2000
💡 18.857s    [test::tests:343] Syscon address range: 0xfd5a2000 - 0xfd5a2100
💡 18.858s    [test::tests:346] Aligned Syscon address range: 0xfd5a2000 - 0xfd5a3000
🐛 18.859s    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fd5a2000, 0xffff9000fd5a3000) -> [0xfd5a2000, 0xfd5a3000)
💡 18.892s    [test::tests:338] Found node: npu@fdab0000
💡 18.893s    [test::tests:343] Syscon address range: 0xfdab0000 - 0xfdac0000
💡 18.894s    [test::tests:346] Aligned Syscon address range: 0xfdab0000 - 0xfdac0000
🐛 18.895s    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fdab0000, 0xffff9000fdac0000) -> [0xfdab0000, 0xfdac0000)
💡 18.925s    [test::tests:338] Found node: power-management@fd8d8000
💡 18.926s    [ted8d9000) -> [0xfd8d8000, 0xfd8d9000)
💡 18.929s    [test::tests:61] emmc ptr: 0xffff9000fe2e0000

💡 18.931s    [test::tests:63] npu grf ptr: 0xffff9000fd5a2000
💡 18.931s    [test::tests:64] npu ptr: 0xffff9000fdab0000
💡 18.932s    [test::tests:65] pmu ptr: 0xffff9000fd8d8000
💡 18.933s    [test::tests:73] emmc addr: 0xffff9000fe2e0000
💡 18.933s    [test::tests:74] clk addr: 0xffff9000fd7c0000
💡 18.934s    [test::tests:75] npu grf addr: 0xffff9000fd5a2000
💡 18.935s    [test::tests:76] npu addr: 0xffff9000fdab0000
💡 18.935s    [test::tests:77] pmu addr: 0xffff9000fd8d8000
💡 18.968s    [test::tests:296] Found node: npu@fdab0000
💡 18.969s    [test::tests:301] NPU0 address range: 0xfdab0000 - 0xfdac0000
💡 18.969s    [test::tests:304] Aligned NPU0 address range: 0xfdab0000 - 0xfdac0000
🐛 18.970s    [somehal::arch::mem::mmu:181] Map `iomap       `: RW- | [0xffff9000fdab0000, 0xffff9000fdac0000) -> [0xfdab0000, 0xfdac0000)
💡 19.004s    [test::tests:320] Found power domain node: power-controller
💡 19.038s    [test::tests:320] Found power domain node: power-controller
💡 19.071s    [test::tests:320] Found power domain node: power-controller
💡 19.072s    [test::tests:278] NPU Version: 0x46495245
💡 19.072s    [sdmmc::emmc:74] EMMC Controller created: EMMC Controller { base_addr: 0xffff9000fe2e0000, card: None, caps: 0x226dc881, clock_base: 200000000 }
💡 19.074s    [sdmmc::emmc:91] Init EMMC Controller
🐛 19.075s    [sdmmc::emmc:100] Card inserted: true
💡 19.075s    [sdmmc::emmc:105] EMMC Version: 0x5
💡 19.076s    [sdmmc::emmc:108] EMMC Capabilities 1: 0b100010011011011100100010000001
💡 19.077s    [sdmmc::emmc:114] EMMC Capabilities 2: 0b1000000000000000000000000111
💡 19.078s    [sdmmc::emmc:162] voltage range: 0x60000, 0x12
💡 19.078s    [sdmmc::emmc::rockchip:145] EMMC Power Control: 0xd
🐛 19.089s    [sdmmc::emmc:974] Bus width set to 1
🐛 19.090s    [sdmmc::emmc::rockchip:318] card_clock: 0, bus_width: 1, timing: 0
💡 19.090s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x0
🐛 19.091s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.092s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.093s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x0
🐛 19.093s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x0
🐛 19.094s    [sdmmc::emmc::rockchip:318] card_clock: 400000, bus_width: 1, timing: 0
🐛 19.095s    [rk3588_clk:111] Setting clk_id 314 to rate 400000
🐛 19.096s    [rk3588_clk:152] CCLK_EMMC: src_clk 2, div 60, new_value 0xbb00, final_value 0xff00bb00
🐛 19.097s    [rk3588_clk:73] Getting clk_id 314
💡 19.097s    [sdmmc::emmc::rockchip:32] input_clk: 400000
💡 19.098s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.099s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x0
🐛 19.099s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.100s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.101s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.102s    [sdmmc::emmc::rockchip 0x7
🐛 19.102s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x0
🐛 19.103s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x0
🐛 19.104s    [sdmmc::emmc::rockchip:318] card_clock: 400000, bus_width: 1, timing: 0
🐛 19.105s    [rk3588_clk:111] Setting clk_id 314 to rate 400000
🐛 19.105s    [rk3588_clk:152] CCLK_EMMC: src_clk 2, div 60, new_value 0xbb00, final_value 0xff00bb00
🐛 19.106s    [rk3588_clk:73] Getting clk_id 314
💡 19.107s    [sdmmc::emmc::rockchip:32] input_clk: 400000
💡 19.108s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.108s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x0
🐛 19.109s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.110s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.111s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.111s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.112s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x0
🐛 19.113s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x0
💡 19.113s    [sdmmc::emmc:226] eMMC initialization started
🔍 19.114s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x0, arg=0x0, resp_type=0x0, command=0x0
🔍 19.115s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.116s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.117s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.129s    [sdmmc::emmc::cmd:416] eMMC reset complete
🔍 19.130s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x0, resp_type=0x1, command=0x102
🔍 19.131s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.132s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.133s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.145s    [sdmmc::emmc::cmd:431] eMMC first CMD1 response (no args): 0xff8080
🔍 19.146s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.147s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.148s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.149s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.152s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0xff8080
💡 19.152s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0xff8080
🔍 19.154s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.155s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.156s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.157s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.159s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0xff8080
💡 19.160s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0xff8080
🔍 19.162s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.163s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.164s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.164s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.167s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0xff8080
💡 19.168s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0xff8080
🔍 19.170s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.171s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.172s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.172s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.175s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0xff8080
💡 19.176s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0xff8080
🔍 19.177s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.179s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.179s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.180s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.183s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0x40ff8080
💡 19.184s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0x40ff8080
🔍 19.185s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x1, arg=0x40060000, resp_type=0x1, command=0x102
🔍 19.186s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.187s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.188s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.191s    [sdmmc::emmc::cmd:453] CMD1 response raw: 0xc0ff8080
💡 19.191s    [sdmmc::emmc::cmd:454] eMMC CMD1 response: 0xc0ff8080
💡 19.192s    [sdmmc::emmc::cmd:478] eMMC initialization status: true
🐛 19.194s    [sdmmc::emmc::cmd:486] Clock control before CMD2: 0x7, stable: true
🔍 19.195s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x2, arg=0x0, resp_type=0x7, command=0x209
🔍 19.196s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.197s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.197s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.200s    [sdmmc::emmc::cmd:69] eMMC response: 0x45010044 0x56343033 0x3201bb29 0x7a017c00
🔍 19.201s    [sdmmc:sp_type=0x15, command=0x31a
🔍 19.202s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.203s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.204s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🔍 19.206s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x9, arg=0x10000, resp_type=0x7, command=0x909
🔍 19.208s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.208s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.209s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
💡 19.212s    [sdmmc::emmc::cmd:69] eMMC response: 0xd00f0032 0x8f5903ff 0xffffffef 0x8a404000
🐛 19.213s    [sdmmc::emmc:256] eMMC CSD version: 4
🔍 19.213s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x7, arg=0x10000, resp_type=0x15, command=0x71a
🔍 19.214s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.215s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.216s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🐛 19.219s    [sdmmc::emmc:327] cmd7: 0x700
🔍 19.219s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x6, arg=0x3b90100, resp_type=0x1d, command=0x61b
🔍 19.220s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.221s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.222s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🐛 19.225s    [sdmmc::emmc:1012] cmd6 0x800
🔍 19.225s    [sdmmc::emmc::cmd:244] Sending command: opcode=0xd, arg=0x10000, resp_type=0x15, command=0xd1a
🔍 19.226s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.227s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.228s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🔍 19.231s    [sdmmc::emmc::cmd:583] cmd_d 0x900
🐛 19.231s    [sdmmc::emmc::rockchip:318] card_clock: 400000, bus_width: 1, timing: 1
🐛 19.232s    [rk3588_clk:111] Setting clk_id 314 to rate 400000
🐛 19.233s    [rk3588_clk:152] CCLK_EMMC: src_clk 2, div 60, new_value 0xbb00, final_value 0xff00bb00
🐛 19.234s    [rk3588_clk:73] Getting clk_id 314
💡 19.234s    [sdmmc::emmc::rock2;0;188;18m💡 19.235s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.236s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x0
🐛 19.237s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.237s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.238s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.239s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.239s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x4
🐛 19.240s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x2
🐛 19.241s    [sdmmc::emmc::rockchip:318] card_clock: 52000000, bus_width: 1, timing: 1
🐛 19.242s    [rk3588_clk:111] Setting clk_id 314 to rate 52000000
🐛 19.243s    [rk3588_clk:152] CCLK_EMMC: src_clk 1, div 23, new_value 0x5600, final_value 0xff005600
🐛 19.244s    [rk3588_clk:73] Getting clk_id 314
💡 19.244s    [sdmmc::emmc::rockchip:32] input_clk: 65217391
💡 19.245s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.246s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x1
🐛 19.246s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x107
💡 19.247s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.248s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.248s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.249s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x4
🐛 19.250s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x2
🔍 19.251s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x8, arg=0x0, resp_type=0x15, command=0x83a
🔍 19.252s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.252s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.253s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🔍 19.254s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
🔍 19.255s    [sdmmc::emmc:354] EXT_CSD: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 3, 0, 144, 23, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 146, 4, 0, 7, 0, 0, 2, 0, 0, 21, 31, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 13, 0, 0, 0, 0, 8, 0, 2, 0, 87, 31, 10, 3, 221, 221, 0, 0, 0, 10, 10, 10, 10, 10, 10, 1, 0, 224, 163, 3, 23, 19, 23, 7, 7, 16, 1, 3, 1, 8, 32, 0, 7, 166, 166, 85, 3, 0, 0, 0, 0, 221, 221, 0, 1, 255, 0, 0, 0, 0, 1, 25, 25, 0, 16, 0, 0, 221, 82, 67, 51, 48, 66, 48, 48, 55, 81, 80, 8, 8, 8, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 16, 0, 3, 3, 0, 5, 3, 3, 1, 63, 63, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]
🐛 19.268s    [sdmmc::emmc:412] Boot partition size: 0x400000
🐛 19.269s    [sdmmc::emmc:413] RPMB partition size: 0x1000000
🐛 19.270s    [sdmmc::emmc:434] GP partition sizes: [0, 0, 0, 0]
🔍 19.271s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x8, arg=0x0, resp_type=0x15, command=0x83a
 19.272s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.273s    [sdmmc::emmc::cmd:263] Response Status: 0b100001
🔍 19.273s    [sdmmc::emmc::cmd:288] Command completed: status=0b100001
🔍 19.274s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
🔍 19.275s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x8, arg=0x0, resp_type=0x15, command=0x83a
🔍 19.276s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.277s    [sdmmc::emmc::cmd:263] Response Status: 0b100001
🔍 19.277s    [sdmmc::emmc::cmd:288] Command completed: status=0b100001
🔍 19.278s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
🔍 19.279s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x6, arg=0x3b70200, resp_type=0x1d, command=0x61b
🔍 19.280s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.281s    [sdmmc::emmc::cmd:263] Response Status: 0b11
🔍 19.282s    [sdmmc::emmc::cmd:288] Command completed: status=0b11
🐛 19.283s    [sdmmc::emmc:1012] cmd6 0x800
🔍 19.283s    [sdmmc::emmc::cmd:244] Sending command: opcode=0xd, arg=0x10000, resp_type=0x15, command=0xd1a
🔍 19.284s    [sdmmc::emm0b1
🔍 19.286s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🔍 19.286s    [sdmmc::emmc::cmd:583] cmd_d 0x900
🐛 19.287s    [sdmmc::emmc:974] Bus width set to 8
🐛 19.288s    [sdmmc::emmc::rockchip:318] card_clock: 52000000, bus_width: 8, timing: 1
🐛 19.289s    [rk3588_clk:111] Setting clk_id 314 to rate 52000000
🐛 19.289s    [rk3588_clk:152] CCLK_EMMC: src_clk 1, div 23, new_value 0x5600, final_value 0xff005600
🐛 19.290s    [rk3588_clk:73] Getting clk_id 314
💡 19.291s    [sdmmc::emmc::rockchip:32] input_clk: 65217391
💡 19.292s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.292s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x1
🐛 19.293s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x107
💡 19.294s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.295s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.295s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.296s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x24
🐛 19.297s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x2
🔍 19.297s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x8, arg=0x0, resp_type=0x15, command=0x83a
🔍 19.299s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.299s    [sdmmc::emmc::cmd:263] Response Status: 0b1
🔍 19.300s    [sdmmc::emmc::cmd:288] Command completed: status=0b1
🔍 19.301s    [sdmmc::emmc::c    [sdmmc::emmc::cmd:244] Sending command: opcode=0x6, arg=0x3b90200, resp_type=0x1d, command=0x61b
🔍 19.303s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.304s    [sdmmc::emmc::cmd:263] Response Status: 0b11
🔍 19.304s    [sdmmc::emmc::cmd:288] Command completed: status=0b11
🐛 19.305s    [sdmmc::emmc:1012] cmd6 0x800
🐛 19.307s    [sdmmc::emmc::rockchip:318] card_clock: 52000000, bus_width: 8, timing: 9
🐛 19.308s    [rk3588_clk:111] Setting clk_id 314 to rate 52000000
🐛 19.308s    [rk3588_clk:152] CCLK_EMMC: src_clk 1, div 23, new_value 0x5600, final_value 0xff005600
🐛 19.309s    [rk3588_clk:73] Getting clk_id 314
💡 19.310s    [sdmmc::emmc::rockchip:32] input_clk: 65217391
💡 19.311s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.311s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x1
🐛 19.312s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x107
💡 19.313s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.313s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.314s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.315s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x24
💡 19.316s    [sdmmc::emmc::rockchip:145] EMMC Power Control: 0xb
🐛 19.326s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x1b
🐛 19.327s    [sdmmc::emmc::rockchip:318] card_clock: 200000000, bus_width: 8, timing: 9
🐛 19.328s    [rk3588_clk:111] Setting clk_id 314 to rate 200000000
🐛 19.329s    [rk3588_clk:152] CCLK_EMMC: src_clk 1, div 6, new_value 0x4500, final_value 0xff004500
🐛 19.330s    [rk3588_clk:73] Getting clk_id 314
💡 19.330s    [sdmmc::emmc::rockchip:32] input_clk: 250000000
💡 19.331s    [sdmmc::emmc::rockchip:42] EMMC Clock Mul: 0
💡 19.332s    [sdmmc::emmc::rockchip:78] EMMC Clock Divisor: 0x1
🐛 19.333s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x107
💡 19.333s    [sdmmc::emmc::rockchip:163] EMMC Clock Control: 0x2
🐛 19.336s    [sdmmc::emmc::rockchip:106] EMMC Clock Control: 0x7
💡 19.337s    [sdmmc::emmc::rockchip:275] Clock 0x7
🐛 19.337s    [sdmmc::emmc::rockchip:353] EMMC Host Control 1: 0x24
💡 19.338s    [sdmmc::emmc::rockchip:145] EMMC Power Control: 0xb
🐛 19.349s    [sdmmc::emmc::rockchip:307] EMMC Host Control 2: 0x1b
🔍 19.350s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.351s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.352s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.352s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.353s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.354s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.355s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.356s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.356s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.357s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.358s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.359s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.360s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.361s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.362s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.363s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.363s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.364s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.365s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.366s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.367s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.367s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.369s    [sdmmc::emmc::cmd:263] Response Sta0
🔍 19.369s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.370s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.371s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.372s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.373s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.373s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.374s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.375s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.376s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.377s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.378s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.379s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.380s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.380s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.381s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.382s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.383s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.384s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.384s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.386s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.386s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.387s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.388s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.389s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.390s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.390s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.391s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.392s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.393s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.394s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.395s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.396s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.397s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.397s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.398s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.399s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.400s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.401s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.402s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.403s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.403s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.404s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.405s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.406s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.407s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.408s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.408s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.409s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.410s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.411s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.412s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.413s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.414s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.414s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.415s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.416s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.417s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.418s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.419s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.420s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.420s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.421s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.422s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.423s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.424s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.425s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.425s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.426s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.427s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.428s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.429s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.430s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.431s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.431s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.432s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.433s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.434s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.435s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.436s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.437s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.437s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.438s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.439s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.440s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.441s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.442s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.442s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.444s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.444s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.445s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.446s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.447s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.448s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.448s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.449s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.450s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.451s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.452s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.453s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.454s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.455s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.455s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
🔍 19.456s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x15, arg=0x0, resp_type=0x15, command=0x153a
🔍 19.457s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.458s    [sdmmc::emmc::cmd:263] Response Status: 0b100000
🔍 19.459s    [sdmmc::emmc::cmd:288] Command completed: status=0b100000
💡 19.459s    [sdmmc::emmc:189] EMMC initialization completed successfully
SD card initialization successful!
Card type: MmcHc
Manufacturer ID: 0x45
Capacity: 0 MB
Block size: 512 bytes
Attempting to read first block...
🔍 19.461s    [sdmmc::emmc::block:365] pio read_blocks: block_id = 5034498, blocks = 1
🔍 19.462s    [sdmmc::emmc::block:383] Reading 1 blocks starting at address: 0x4cd202
🔍 19.463s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x11, arg=0x4cd202, resp_type=0x15, command=0x113a
🔍 19.464s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.465s    [sdmmc::emmc::cmd:263] Response Status: 0b100001
🔍 19.466s    [sdmmc::emmc::cmd:288] Command completed: status=0b100001
🔍 19.467s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
Successfully read first block!
First 16 bytes of first block: [40, E2, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 8F, D2, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, DB, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, E0, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, C0, EC, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, E9, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, EE, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, E4, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, C0, DE, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, F0, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, DD, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, E7, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, A9, D5, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, 5B, D7, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, 50, D6, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, 4E, D6, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 60, 4F, D6, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, CE, CD, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, 48, DF, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 8E, D2, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 60, D6, CD, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 90, D2, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, A0, 09, DD, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 80, B9, E1, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, EB, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 60, DD, E0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 20, D1, CD, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, E0, 7E, E2, 01, 00, 00, 00, 00, 00, 00, 00, 20, A8, D5, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 40, D7, CD, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 91, D2, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, C0, E5, D0, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
Testing write and read back...
🔍 19.485s    [sdmmc::emmc::block:417] pio write_blocks: block_id = 3, blocks = 1
🔍 19.486s    [sdmmc::emmc::block:439] Writing 1 blocks starting at address: 0x3
🔍 19.487s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x18, arg=0x3, resp_type=0x15, command=0x183a
🔍 19.488s    [sdmmc::emmc::cmd:263] Response Status: 0b10000
🔍 19.489s    [sdmmc::emmc::cmd:263] Response Status: 0b10001
🔍 19.490s    [sdmmc::emmc::cmd:288] Command completed: status=0b10001
🔍 19.491s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
Successfully wrote to block 3!
🔍 19.493s    [sdmmc::emmc::block:365] pio read_blocks: block_id = 3, blocks = 1
🔍 19.494s    [sdmmc::emmc::block:383] Reading 1 blocks starting at address: 0x3
🔍 19.495s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x11, arg=0x3, resp_type=0x15, command=0x113a
🔍 19.496s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.496s    [sdmmc::emmc::cmd:263] Response Status: 0b100001
🔍 19.497s    [sdmmc::emmc::cmd:288] Command completed: status=0b100001
🔍 19.498s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
Successfully read back block 3!
First 16 bytes of read block: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Data verification successful: written and read data match perfectly!
Testing multi-block read...
🔍 19.513s    [sdmmc::emmc::block:365] pio read_blocks: block_id = 200, blocks = 4
🔍 19.514s    [sdmmc::emmc::block:383] Reading 4 blocks starting at address: 0xc8
🔍 19.515s    [sdmmc::emmc::cmd:244] Sending command: opcode=0x12, arg=0xc8, resp_type=0x15, command=0x123a
🔍 19.516s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.516s    [sdmmc::emmc::cmd:263] Response Status: 0b100001
🔍 19.517s    [sdmmc::emmc::cmd:288] Command completed: status=0b100001
🔍 19.518s    [sdmmc::emmc::cmd:339] Data transfer: cmd.data_present=true
🔍 19.519s    [sdmmc::emmc::cmd:244] Sending command: opcode=0xc, arg=0x0, resp_type=0x1d, command=0xc1b
🔍 19.520s    [sdmmc::emmc::cmd:263] Response Status: 0b0
🔍 19.521s    [sdmmc::emmc::cmd:263] Response Status: 0b11
🔍 19.522s    [sdmmc::emmc::cmd:288] Command completed: status=0b11
Successfully read 4 blocks starting at block address 200!
First 16 bytes of first block: [A0, 2F, 00, B9, A1, 8B, 0D, A9, A0, 07, 42, A9, A0, 07, 04, A9]
First 16 bytes of last block: [B5, 01, BD, 01, C6, 01, CE, 01, D6, 01, DE, 01, E7, 01, EF, 01]
SD card test complete
npu version: 0x8010
💡 19.525s    [test::tests:351] test npu cru
🐛 19.525s    [rk3588_clk:439] Enabling gate_id 301
🐛 19.526s    [rk3588_clk:578] Getting status for gate_id 301
💡 19.527s    [rk3588_clk:631] gate_con30 value: 0x0
🐛 19.527s    [rk3588_clk:669] Gate 301 is enabled
npu gate enable: true
🐛 19.528s    [rk3588_clk:439] Enabling gate_id 302
🐛 19.529s    [rk3588_clk:578] Getting status for gate_id 302
💡 19.529s    [rk3588_clk:636] gate_con30 value: 0x0
🐛 19.530s    [rk3588_clk:669] Gate 302 is enabled
npu gate enable: true
🐛 19.531s    [rk3588_clk:439] Enabling gate_id 290
🐛 19.531s    [rk3588_clk:578] Getting status for gate_id 290
💡 19.532s    [rk3588_clk:586] gate_con27 value: 0xaa04
🐛 19.533s    [rk3588_clk:mtest test_platform passed
All tests passed
```

### 测试功能说明

测试程序会执行以下操作：

1. **设备树解析**: 从设备树中查找 CRU、NPU、EMMC 等硬件节点地址
2. **电源域管理**: 初始化 NPU 相关的电源域 (NPUTOP, NPU, NPU1, NPU2)
3. **EMMC 存储测试**:
   - 初始化 EMMC 控制器和时钟
   - 读取 SD 卡信息
   - 测试块读写操作
   - 验证数据一致性
4. **NPU 时钟门控测试**:
   - 使能 NPU 相关的时钟门控 (ACLK_NPU0, HCLK_NPU0, ACLK_NPU1, 等)
   - 验证门控状态
   - 读取 NPU 版本信息

**注意**: 完整测试需要支持 RK3588 的 ARM 硬件平台和 U-Boot 环境
