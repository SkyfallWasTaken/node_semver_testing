use crate::common::PackageNames;
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
    total_rows: u64,
    rows: Vec<DumpDependency>,
}

#[derive(Serialize, Deserialize)]
struct DumpDependency {
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

    let mut names = Vec::new();
    for name in dump.rows {
        names.push(name.id);
    }

    let result = PackageNames {
        count: dump.total_rows,
        names,
    };

    out_file
        .write_all(serde_json::to_string(&result)?.as_bytes())
        .await?;

    println!(
        "{} in {:?} seconds.",
        "Finished".green().bold(),
        Instant::now() - start
    );

    Ok(())
}
