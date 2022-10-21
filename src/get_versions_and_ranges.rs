use crate::common::{Dependency, PackageNames};
use crate::util::get_client;
use eyre::Result;
use indicatif::ProgressBar;
use owo_colors::OwoColorize;
use tokio::fs::{read_to_string, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::task;

pub async fn invoke() -> Result<()> {
    let package_names: PackageNames =
        serde_json::from_str(&read_to_string("out/package_names.json").await?)?;

    let pb = ProgressBar::new(package_names.count);

    let mut all_versions = Vec::new();
    let mut all_ranges: Vec<String> = Vec::new();

    for name in package_names.names {
        let dependency = task::spawn(async move {
            let client = get_client();
            let dependency: Dependency = client
                .get(format!("https://skimdb.npmjs.com/registry/{name}"))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            dependency
        })
        .await?;

        if let Some(versions) = dependency.versions {
            for (version, _) in versions {
                all_versions.push(version);
            }
        }

        if let Some(dependencies) = dependency.dependencies {
            for (_, range) in dependencies {
                all_ranges.push(range);
            }
        }

        if let Some(dev_dependencies) = dependency.dev_dependencies {
            for (_, range) in dev_dependencies {
                all_ranges.push(range);
            }
        }

        pb.inc(1);
    }
    pb.finish_with_message("found all versions + ranges");

    all_versions.dedup();
    all_ranges.dedup();

    write_results(all_versions, all_ranges).await?;
    println!(
        "{} all versions and ranges.",
        "Successfully wrote".green().bold()
    );

    Ok(())
}

pub async fn write_results(versions: Vec<String>, ranges: Vec<String>) -> Result<()> {
    let mut version_out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("out/versions.json")
        .await?;
    let mut ranges_out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("out/ranges.json")
        .await?;

    let versions = serde_json::to_string(&versions)?;
    let ranges = serde_json::to_string(&ranges)?;

    version_out_file.write_all(versions.as_bytes()).await?;
    ranges_out_file.write_all(ranges.as_bytes()).await?;

    Ok(())
}
