use eyre::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;
use tokio::{
    fs::{read_to_string, OpenOptions},
    io::AsyncWriteExt,
    time::Instant,
};

#[derive(Deserialize)]
pub struct Dump {
    total_rows: i32,
    rows: Vec<Dependency>,
}

#[derive(Deserialize)]
pub struct Dependency {
    id: String,
}

pub async fn invoke() -> Result<()> {
    println!("{} of raw dump.", "Starting parsing".green().bold());
    let start = Instant::now();
    let file = read_to_string("dump.json").await?;
    let dump: Dump = serde_json::from_str(&file)?;
    println!(
        "{} in {:?} seconds.",
        "Parsed dump".green().bold(),
        Instant::now() - start
    );

    println!(
        "{} with {} packages.",
        "Converting dump".green().bold(),
        dump.total_rows
    );

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("package_names.txt")
        .await?;

    let mut result = String::new();
    for dependency in dump.rows.iter() {
        result.push_str(&format!("{},", dependency.id))
    }

    out_file.write_all(result.as_bytes()).await?;

    println!(
        "{} in {:?} seconds.",
        "Finished".green().bold(),
        Instant::now() - start
    );

    Ok(())
}
