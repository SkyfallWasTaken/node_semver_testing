use eyre::Result;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{read_to_string, OpenOptions},
    io::AsyncWriteExt,
    time::Instant,
};

#[derive(Deserialize)]
struct Dump {
    total_rows: i32,
    rows: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
struct Dependency {
    id: String,
}

pub async fn invoke() -> Result<()> {
    println!("{} of raw dump.", "Starting parsing".green().bold());
    let start = Instant::now();
    let file = read_to_string("out/dump.json").await?;
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
        .append(true)
        .open("out/package_names.json")
        .await?;

    let mut result = String::new();
    for (index, dependency) in dump.rows.iter().enumerate() {
        result.push_str(&format!("  \"{}\"", dependency.id));
        if index != (dump.total_rows - 2) as usize {
            result.push(',');
            result.push('\n')
        }
    }

    out_file
        .write_all(format!("[\n{result}\n]").as_bytes())
        .await?;

    println!(
        "{} in {:?} seconds.",
        "Finished".green().bold(),
        Instant::now() - start
    );

    Ok(())
}
