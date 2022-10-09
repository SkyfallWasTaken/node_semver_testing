use eyre::Result;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use owo_colors::OwoColorize;

pub async fn invoke() -> Result<()> {
    let client = reqwest::ClientBuilder::new().gzip(true).build()?;

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("dump.json")
        .await?;

    println!("{} to `dump.json`", "Writing dump".green().bold());
    let mut response = client
        .get("https://skimdb.npmjs.com/registry/_all_docs")
        .send()
        .await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?
    }

    Ok(())
}
