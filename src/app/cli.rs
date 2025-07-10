use crate::app::help::show_help;
use crate::app::interactive::show_interactive_menu;
use crate::app::uuid::{get_uuid_with_confirmation, UuidResult};
use crate::core::handler::{
    display_decrypt_success, display_encrypt_success, display_error, perform_decrypt,
    perform_encrypt,
};
use clap::Parser;
use rust_i18n::t;

#[derive(Parser)]
#[clap(name = "RustDesk ID Tool")]
#[clap(about = "A tool for encrypting and decrypting RustDesk IDs", long_about = None)]
#[clap(disable_help_flag = true)]
pub struct Cli {
    /// Custom ID to encrypt
    #[clap(short, long)]
    id: Option<String>,

    /// Encrypted ID to decrypt
    #[clap(short, long)]
    eid: Option<String>,

    /// UUID for encryption/decryption
    #[clap(short, long)]
    uuid: Option<String>,

    /// Set the language
    #[clap(short, long, default_value = "en")]
    lang: String,

    /// Show detailed help information
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    help: bool,
}

pub fn run() {
    let cli = Cli::parse();
    rust_i18n::set_locale(&cli.lang);

    // Check if help flag is set
    if cli.help {
        show_help(&cli.lang);
        return;
    }

    let uuid_option = cli.uuid.as_deref();
    let has_id = cli.id.is_some();
    let has_eid = cli.eid.is_some();

    if !has_id && !has_eid && uuid_option.is_none() {
        show_interactive_menu(&cli.lang);
        return;
    }

    let uuid = match uuid_option {
        Some(u) => u.to_string(),
        None => match get_uuid_with_confirmation() {
            UuidResult::Success(uuid) => uuid,
            UuidResult::Cancelled => {
                println!("{}", t!("operation_cancelled"));
                return;
            }
            UuidResult::Error => {
                println!("{}", t!("error_uuid_required"));
                println!("{}", t!("help_prompt"));
                return;
            }
        },
    };

    run_with_uuid(&cli, &uuid);
}

fn run_with_uuid(cli: &Cli, uuid: &str) {
    if let Some(ref custom_id) = cli.id {
        let result = perform_encrypt(custom_id, uuid);
        match result {
            crate::core::handler::EncryptResult::Success { .. } => {
                display_encrypt_success(&result);
            }
            crate::core::handler::EncryptResult::Error(error_msg) => {
                display_error(&error_msg);
            }
        }
    } else if let Some(ref enc_id) = cli.eid {
        let result = perform_decrypt(enc_id, uuid);
        match result {
            crate::core::handler::DecryptResult::Success { .. } => {
                display_decrypt_success(&result);
            }
            crate::core::handler::DecryptResult::Error(error_msg) => {
                display_error(&error_msg);
            }
        }
    }
}
