use std::env::home_dir;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Cursor};
use std::path::Path;

use clap::{App, Arg};
use flate2::read::GzDecoder;
use serde::Deserialize;
use surf::http::headers::{ACCEPT, CONTENT_LENGTH};
use tar::Archive;

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("protonug")
        .version("1.0")
        .author("Sanchez <sanzcodinghelp@gmail.com>")
        .about("An installer/updater for the GE (GloriousEggroll) custom Steam Proton builds.")
        .subcommand(
            App::new("update")
                .about("Update the GE Proton build.")
                .arg(Arg::with_name("clean").short('c').long("clean").help("Remove the older existing GE proton builds.")),
        )
        .get_matches();

    let ge_proton_asset_pattern = regex::Regex::new(r"^GE-Proton(\d+-\d+)\.tar\.gz$").unwrap();
    let ge_proton_dir_pattern = regex::Regex::new(r"^GE-Proton(\d+-\d+)$").unwrap();

    println!("Getting the latest release of GE Proton...");

    let client = surf::Client::new();
    let mut response = client
        .get("https://api.github.com/repos/GloriousEggroll/proton-ge-custom/releases/latest")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await?;

    let latest_release: Release = response.body_json().await?;

    println!("Done");

    let ge_proton_asset = latest_release
        .assets
        .into_iter()
        .find(|asset| ge_proton_asset_pattern.is_match(&asset.name))
        .ok_or("No suitable GE-Proton asset found.")?;

    println!("Downloading the latest GE Proton build...");

    let mut response = client
        .get(&ge_proton_asset.browser_download_url)
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await?;

    let content_length: usize = response
        .header(CONTENT_LENGTH)
        .ok_or("No content length header")?
        .to_str()?
        .parse()?;

    let mut progress = pbr::ProgressBar::new(content_length as u64);
    progress.set_width(Some(80));

    let mut downloaded_data = Vec::new();
    while let Some(chunk) = response.next().await {
        let chunk = chunk?;
        progress.add(chunk.len() as u64);
        downloaded_data.extend(&chunk);
    }

    println!("Extracting GE Proton...");

    let cursor = Cursor::new(downloaded_data);
    let gz = GzDecoder::new(cursor);
    let mut archive = Archive::new(gz);

    let home = home_dir().ok_or("Failed to find home directory")?;
    let steam_proton_dir = home.join(".steam/root/compatibilitytools.d");
    create_dir_all(&steam_proton_dir)?;

    let clean = matches.subcommand_matches("update").map_or(false, |m| m.is_present("clean"));

    if clean {
        for entry in std::fs::read_dir(&steam_proton_dir)? {
            let entry = entry?;
            if let Some(ge_proton) = entry.path().file_name().and_then(|f| f.to_str()) {
                if ge_proton_dir_pattern.is_match(ge_proton) {
                    std::fs::remove_dir_all(entry.path())?;
                }
            }
        }
    }

    archive.unpack(&steam_proton_dir)?;

    println!("GE Proton successfully installed/updated!");

    Ok(())
}
