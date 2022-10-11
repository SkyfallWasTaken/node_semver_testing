use crate::util::get_client;
use eyre::Result;
use owo_colors::OwoColorize;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub async fn invoke() -> Result<()> {
    let client = get_client();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("out/dump.json")
        .await?;

    println!("{} to `dump.json`", "Writing dump".green().bold());
    let mut response = client
        .get("https://skimdb.npmjs.com/registry/_all_docs")
        .send()
        .await?;

    let mut buffer = Vec::new();
    let mut chunk_count = 0;
    while let Some(chunk) = response.chunk().await? {
        // New chunk!
        chunk_count += 1;
        if chunk_count % 50 == 0 {
            println!("Now on chunk {chunk_count} ({} bytes)", buffer.len());
        }
        buffer.write_all(&chunk).await?;
    }

    file.write_all(buffer.as_slice()).await?;

    Ok(())
}
