// if let 简洁控制流

fn main() {
    // 使用 match 必须列举所有可能
    let config_max = Some(3_u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 使用 if let 语法来简化
    let config_max = Some(3_u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
