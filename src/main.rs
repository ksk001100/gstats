mod lib;

use std::env;

use lib::github::api;
use lib::traits::Statistics;

fn main() {
    let args: Vec<String> = env::args().collect();
    api::release(&args[1]).stats();
}
