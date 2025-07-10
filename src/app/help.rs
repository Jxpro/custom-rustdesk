use rust_i18n::t;

/// 显示详细的帮助信息
pub fn show_help(_lang: &str) {
    println!();
    println!("{}", t!("help_title"));
    println!();

    show_program_function();
    show_cli_usage();
    show_cli_parameters();
    show_cli_examples();
    show_config_file_location();
    show_notes();
    show_get_help();
}

/// 显示程序功能说明
fn show_program_function() {
    println!("{}", t!("program_function_title"));
    println!("{}", t!("program_function_desc1"));
    println!("{}", t!("program_function_desc2"));
    println!();
}

/// 显示命令行用法
fn show_cli_usage() {
    println!("{}", t!("cli_usage_title"));
    println!("{}", t!("cli_usage_encrypt"));
    println!("{}", t!("cli_usage_decrypt"));
    println!();
}

/// 显示命令行参数
fn show_cli_parameters() {
    println!("{}", t!("cli_params_title"));
    println!("{}", t!("cli_param_id"));
    println!("{}", t!("cli_param_eid"));
    println!("{}", t!("cli_param_uuid"));
    println!("{}", t!("cli_param_lang"));
    println!("{}", t!("cli_param_help"));
    println!();
}

/// 显示命令行示例
fn show_cli_examples() {
    println!("{}", t!("cli_examples_title"));
    println!("{}", t!("cli_example_encrypt"));
    println!("{}", t!("cli_example_decrypt"));
    println!("{}", t!("cli_example_help"));
    println!();
}

/// 显示配置文件位置
fn show_config_file_location() {
    println!("{}", t!("config_file_location_title"));
    println!("{}", t!("config_file_location_macos"));
    println!("{}", t!("config_file_location_windows"));
    println!();
}

/// 显示注意事项
fn show_notes() {
    println!("{}", t!("notes_title"));
    println!("{}", t!("note_1"));
    println!("{}", t!("note_2"));
    println!("{}", t!("note_3"));
    println!();
}

/// 显示获取帮助信息
fn show_get_help() {
    println!("{}", t!("get_help_title"));
    println!("{}", t!("get_help_github"));
    println!("{}", t!("get_help_email"));
    println!();
}
