use serde::{Deserialize, Serialize};
use seahorse::color;

use crate::lib::traits::Statistics;

#[derive(Serialize, Deserialize)]
pub struct View {
    pub count: u32,
    pub uniques: u32,
    pub views: Vec<Views>,
}

#[derive(Serialize, Deserialize)]
pub struct Views {
    pub timestamp: String,
    pub count: u32,
    pub uniques: u32,
}

impl Statistics for View {
    fn stats(&self) {
        println!("Timestamp            | Count | Uniques");
        for v in &self.views {
            let s = format!("{} | {}     | {}", v.timestamp, color::yellow(v.count), color::yellow(v.uniques));
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