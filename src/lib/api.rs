use reqwest;
use serde_json;

use crate::lib::structs::release::Releases;

pub fn release(owner_repo: &str) -> Releases {
    let url = format!("https://api.github.com/repos/{}/releases", owner_repo);
    let mut res = reqwest::get(&url).unwrap();
    let releases: Releases = serde_json::from_str(&res.text().unwrap()).unwrap();
    releases
}