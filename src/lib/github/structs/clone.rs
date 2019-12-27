use serde::{Deserialize, Serialize};
use seahorse::color;

use crate::lib::traits::{Statistics};

#[derive(Serialize, Deserialize)]
pub struct Clone {
    pub count: u32,
    pub uniques: u32,
    pub clones: Vec<Clones>,
}

#[derive(Serialize, Deserialize)]
pub struct Clones {
    pub timestamp: String,
    pub count: u32,
    pub uniques: u32,
}

impl Statistics for Clone {
    fn stats(&self) {
        println!("Timestamp            | Count | Uniques");
        for c in &self.clones {
            let s = format!("{} | {}     | {}", c.timestamp, color::yellow(c.count), color::yellow(c.uniques));
            println!("{}", s);
            println!("{}", "-".to_string().repeat(s.len()));
        }
        println!("{}                | {}     | {}",
                 color::green("Total"),
                 color::red(self.count),
                 color::red(self.uniques)
        );
    }
}