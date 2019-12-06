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

trait Statistics {
    fn stats(&self);
}

impl Statistics for Releases {
    fn stats(&self) {
        let mut total_count = 0;
        for release in self {
            let count = release.download_count();
            total_count = total_count + count;
            println!("{} : {}", release.tag_name, yellow(count));
        }
        println!("Total : {}", red(total_count));
    }
}

fn red<T: std::fmt::Display>(t: T) -> String {
    format!("\x1b[31m{}\x1b[0m", t)
}

fn green<T: std::fmt::Display>(t: T) -> String {
    format!("\x1b[32m{}\x1b[0m", t)
}

fn yellow<T: std::fmt::Display>(t: T) -> String {
    format!("\x1b[33m{}\x1b[0m", t)
}

fn blue<T: std::fmt::Display>(t: T) -> String {
    format!("\x1b[34m{}\x1b[0m", t)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = format!("https://api.github.com/repos/{}/releases", args[1]);
    let mut res = reqwest::get(&url).unwrap();
    let releases: Releases = serde_json::from_str(&res.text().unwrap()).unwrap();
    releases.stats();
}
