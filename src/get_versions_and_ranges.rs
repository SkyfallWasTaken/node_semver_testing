use crate::util::get_client;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs::read_to_string;
use tokio::task;

#[derive(Deserialize, Debug)]
struct Empty {}

#[derive(Deserialize, Debug)]
struct Dependency {
    versions: HashMap<String, Empty>,
}

pub async fn invoke() -> Result<()> {
    let package_names: Vec<String> =
        serde_json::from_str(&read_to_string("out/package_names.json").await?)?;

    let mut data = Vec::new();

    for name in package_names {
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
            data.push(version)
        }
    }

    dbg!(data.dedup());

    Ok(())
}
