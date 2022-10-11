use eyre::Result;
use owo_colors::OwoColorize;
use std::{env, process};

mod common;
mod extract_package_names;
mod get_versions_and_ranges;
mod test_versions_and_ranges;
mod util;
mod write_dump;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // We don't need a full argument parser, so this will do
    let args: Vec<String> = env::args().collect();
    let command_string = args.get(1).unwrap_or(&String::new()).to_lowercase();
    let command = command_string.as_str();

    match command {
        "write-dump" => write_dump::invoke().await.unwrap(),
        "extract-package-names" => extract_package_names::invoke().await?,
        "get-versions-and-ranges" => get_versions_and_ranges::invoke().await?,
        _ => {
            eprintln!("{}", "command not found!".red().bold());
            eprintln!("commands: write-dump, extract-package-names, get-versions-and-ranges");
            process::exit(exitcode::USAGE)
        }
    };

    Ok(())
}
