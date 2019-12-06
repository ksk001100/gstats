use std::env;

use serde::{Deserialize, Serialize};
use serde_json::{self, Result};
use reqwest;

#[derive(Serialize, Deserialize)]
struct Release {
    url: String,
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
struct Asset {
    download_count: u32,
}

impl Release {
    fn download_count(&self) -> u32 {
        (&self.assets).into_iter().fold(0, |sum, asset| sum + asset.download_count)
    }
}

type Releases = Vec<Release>;

trait ReleasesMethods {
    fn stats(&self);
}

impl ReleasesMethods for Releases {
    fn stats(&self) {
        let mut total_count = 0;
        for release in self {
            let count = release.download_count();
            total_count = total_count + count;
            println!("{} : \x1b[33m{}\x1b[0m", release.tag_name, count);
        }
        println!("Total : \x1b[31m{}\x1b[0m", total_count);
    }
}

fn release_download_count(data: String) -> Result<()> {
    let releases: Releases = serde_json::from_str(&data).unwrap();
    releases.stats();

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = format!("https://api.github.com/repos/{}/releases", args[1]);
    let mut res = reqwest::get(&url).unwrap();
    release_download_count(res.text().unwrap()).unwrap();
}
