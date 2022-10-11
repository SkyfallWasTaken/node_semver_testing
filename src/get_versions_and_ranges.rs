use crate::common::{Dependency, PackageNames};
use crate::util::get_client;
use eyre::Result;
use indicatif::ProgressBar;
use tokio::fs::read_to_string;
use tokio::task;

pub async fn invoke() -> Result<()> {
    let package_names: PackageNames =
        serde_json::from_str(&read_to_string("out/package_names.json").await?)?;

    let pb = ProgressBar::new(package_names.count);

    let mut versions = Vec::new();
    let mut ranges: Vec<String> = Vec::new();

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

        for (version, _) in dependency.versions {
            versions.push(version)
        }

        if let Some(dependencies) = dependency.dependencies {
            for (_, version) in dependencies {
                ranges.push(version)
            }
        }

        if let Some(dev_dependencies) = dependency.dev_dependencies {
            for (_, version) in dev_dependencies {
                ranges.push(version)
            }
        }

        pb.inc(1);
    }
    pb.finish_with_message("found all versions + ranges");

    versions.dedup();
    ranges.dedup();

    Ok(())
}
