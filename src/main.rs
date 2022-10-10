use owo_colors::OwoColorize;
use std::{env, process};

mod extract_package_names;
mod test_dump;
mod write_dump;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();

    // We don't need a full argument parser, so this will do
    let args: Vec<String> = env::args().collect();
    let command_string = args.get(1).unwrap_or(&String::new()).to_lowercase();
    let command = command_string.as_str();

    match command {
        "write-dump" => write_dump::invoke().await.unwrap(),
        "extract-package-names" => extract_package_names::invoke().await.unwrap(),
        "test-dump" => test_dump::invoke().await,
        _ => {
            eprintln!("{}", "command not found!".red().bold());
            eprintln!("commands: write-dump, test-dump");
            process::exit(exitcode::USAGE)
        }
    }
}
