#[macro_use]
extern crate rust_i18n;

i18n!("i18n");

mod app;
mod core;

fn main() {
    app::cli::run();
}
