mod lib;

use std::env;

use lib::github::api::Api;
use lib::color;
use lib::traits::Statistics;

fn main() {
    let args: Vec<String> = env::args().collect();
    let api = Api::new("".to_string());

    match api.release(&args[1]) {
        Ok(releases) => releases.stats(),
        Err(_) => println!("{}", color::red("Repository does not exist..."))
    }
}
