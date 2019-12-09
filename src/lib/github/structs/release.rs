use serde::{Deserialize, Serialize};

use crate::lib::app::color;
use crate::lib::traits::{Statistics};

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: u32,
    pub node_id: String,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: Author,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub download_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: u32,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub u_type: String,
    pub site_admin: bool,
}

impl Release {
    pub fn download_count(&self) -> u32 {
        (&self.assets).into_iter().fold(0, |s, a| s + a.download_count)
    }
}

pub type Releases = Vec<Release>;

impl Statistics for Releases {
    fn stats(&self) {
        let mut total_count = 0;
        for release in self {
            let count = release.download_count();
            let s = format!("{} | {}", release.tag_name, color::yellow(count));
            total_count = total_count + count;
            println!("{}", s);
            println!("{}", "-".to_string().repeat(s.len()));
        }
        println!("{}  | {}", color::green("Total"), color::red(total_count));
    }
}