use reqwest;
use serde_json;
use reqwest::{
    Client,
    header::{
        AUTHORIZATION,
        HeaderMap,
        HeaderValue,
    }
};

use crate::lib::github::structs::{
    release::Releases,
    clone::Clone,
    view::View,
};

pub struct Api {
    client: Client,
    token: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        let client = Client::new();
        Self { client, token }
    }

    fn headers(&self) -> HeaderMap<HeaderValue> {
        let auth = format!("token {}", self.token);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
        headers
    }

    pub fn release(&self, owner_repo: &str) -> Result<Releases, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/releases", owner_repo);
        let mut res = self.client.get(&url).send()?;
        let releases = serde_json::from_str(&res.text().unwrap())?;

        Ok(releases)
    }

    pub fn clone(&self, owner_repo: &str) -> Result<Clone, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/traffic/clones", owner_repo);
        let mut res = self.client.get(&url)
            .headers(self.headers())
            .send()?;
        let clone = serde_json::from_str(&res.text().unwrap())?;

        Ok(clone)
    }

    pub fn view(&self, owner_repo: &str) -> Result<View, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{}/traffic/views", owner_repo);
        let mut res = self.client.get(&url)
            .headers(self.headers())
            .send()?;
        let view = serde_json::from_str(&res.text().unwrap())?;

        Ok(view)
    }
}