use crate::util::get_client;
use eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs::read_to_string;
use tokio::task;

#[derive(Deserialize, Debug)]
struct Empty;

#[derive(Deserialize, Debug)]
struct Dependency {
    versions: HashMap<String, Empty>,
}

pub async fn invoke() -> Result<()> {
    let package_names: Vec<String> =
        serde_json::from_str(&read_to_string("out/package_names.json").await?)?;

    for name in package_names {
        task::spawn(async move {
            let client = get_client();
            let dependency: Dependency = client
                .get(format!("https://skimdb.npmjs.com/registry/{name}"))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            dbg!(dependency);
        })
        .await?;
    }

    Ok(())
}
