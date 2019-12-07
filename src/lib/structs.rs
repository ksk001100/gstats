use serde::{Deserialize, Serialize};
use serde_json::{self, Result};

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