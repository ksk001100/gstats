use std::env;

use serde::{Deserialize, Serialize};
use serde_json::{self, Result};
use reqwest;

#[derive(Serialize, Deserialize)]
struct Release {
    url: String,
    tag_name: String,
    assets: Vec<ReleaseAssets>,
}

#[derive(Serialize, Deserialize)]
struct ReleaseAssets {
    download_count: u32,
}

fn release_download_count(data: String) -> Result<()> {
    let repos: Vec<Release> = serde_json::from_str(&data).unwrap();
    let mut total_count = 0;
    for repo in repos {
        let count = repo.assets.into_iter().fold(0, |sum, asset| sum + asset.download_count);
        total_count = total_count + count;
        println!("{} : {}", repo.tag_name, count);
    }
    println!("Total : {}", total_count);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = format!("https://api.github.com/repos/{}/releases", args[1]);
    let mut res = reqwest::get(&url).unwrap();
    release_download_count(res.text().unwrap()).unwrap();
}
