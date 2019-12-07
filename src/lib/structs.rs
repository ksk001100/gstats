use serde::{Deserialize, Serialize};
use serde_json;

use crate::lib::color;
use crate::lib::traits::{Statistics};

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub download_count: u32,
}

impl Release {
    pub fn download_count(&self) -> u32 {
        (&self.assets).into_iter().fold(0, |sum, asset| sum + asset.download_count)
    }
}

pub type Releases = Vec<Release>;

impl Statistics for Releases {
    fn stats(&self) {
        let mut total_count = 0;
        for release in self {
            let count = release.download_count();
            total_count = total_count + count;
            let s = format!("{} | {}", release.tag_name, color::yellow(count));
            println!("{}", s);
            println!("{}", "-".to_string().repeat(s.len()));
        }
        println!("Total  | {}", color::red(total_count));
    }
}