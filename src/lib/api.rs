use reqwest;

use crate::lib::color;
use crate::lib::structs::{Releases};
use crate::lib::traits::{Statistics};

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

pub fn release(owner_repo: &str) -> Releases {
    let url = format!("https://api.github.com/repos/{}/releases", owner_repo);
    let mut res = reqwest::get(&url).unwrap();
    let releases: Releases = serde_json::from_str(&res.text().unwrap()).unwrap();
    releases
}