/// 生成 register_bitfields! 宏调用的宏
///
/// # 参数
/// - reg_name: 寄存器名称
/// - reg_type: 寄存器类型 (u8, u16, u32, u64)
/// - fields: 字段定义数组，每个元素为 (field_name, offset, numbits)
///
/// # 示例
/// ```rust
/// generate_register_bitfields!(
///     GPIO_CTRL,
///     u32,
///     [
///         (mode, 0, 2),
///         (pull, 2, 2, [None = 0, Up = 1, Down = 2]),
///         (speed, 4, 2),
///         (enable, 6, 1, [Disabled = 0, Enabled = 1])
///     ]
/// );
///
/// generate_register_bitfields!(
///     SPI_CTRL,
///     u32,
///     [
///         (mode, 0, 2, [Mode0 = 0, Mode1 = 1, Mode2 = 2, Mode3 = 3]),
///         (bits, 2, 4),
///         (enable, 8, 1, [Disabled = 0, Enabled = 1])
///     ]
/// );
/// ```

#[macro_export]
macro_rules! generate_register_bitfields {
    ($reg_name:ident, $reg_type:tt, [$(($field_name:ident, $offset:literal, $numbits:literal, [$($enum_name:ident = $enum_val:expr),* $(,)?])),+ $(,)?]) => {
        register_bitfields![$reg_type,
            $reg_name [
                $(
                    $field_name OFFSET($offset) NUMBITS($numbits) [
                        $($enum_name = $enum_val),*
                    ],
                )+
            ]
        ];
    };
}
