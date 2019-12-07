use reqwest;
use serde_json;

use crate::lib::github::structs::release::Releases;

pub struct Api {
    token: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn release(&self, owner_repo: &str) -> Result<Releases, Box<std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/releases", owner_repo);
        let mut res = reqwest::get(&url).unwrap();
        let releases = serde_json::from_str(&res.text().unwrap())?;

        Ok(releases)
    }
}