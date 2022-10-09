use std::{env, process};
use owo_colors::OwoColorize;

mod write_dump;
mod test_dump;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();

    // We don't need a full argument parser, so this will do
    let args: Vec<String> = env::args().collect();
    let command_string = args.get(1).unwrap_or(&String::new()).to_lowercase();
    let command = command_string.as_str();

    match command {
        "write-dump" => write_dump::invoke().await.unwrap(),
        "test-dump" => test_dump::invoke().await,
        _ => {
            eprintln!("{}", "command not found!".red().bold());
            eprintln!("commands: write-dump, test-dump");
            process::exit(exitcode::USAGE)
        }
    }
}