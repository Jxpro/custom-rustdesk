use std::io::{self, Write};

/// 读取用户输入的一行文本
pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// 显示提示并读取用户输入
pub fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    read_line()
}

/// 验证输入是否为空
pub fn is_empty_input(input: &str) -> bool {
    input.trim().is_empty()
}
