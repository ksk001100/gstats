mod lib;

use std::env;

use lib::github::api::Api;
use lib::color;
use lib::traits::Statistics;
use lib::app::{App, command::Command};
use crate::lib::app::command::Action;

fn main() {
    let args: Vec<String> = env::args().collect();

    let action: Action = |s: String| {
        println!("hello");
    };

    let app = App::new(
        "gstats".to_string(),
        "gstats [command] [Owner/Repo]".to_string(),
        "v0.0.1".to_string(),
        vec![
            Command{name: "release".to_string(), action }
        ],
    );

    app.run(args);

//
//    let api = Api::new("".to_string());
//
//    match api.release(&args[1]) {
//        Ok(releases) => releases.stats(),
//        Err(_) => eprintln!("{}", color::red("Repository does not exist..."))
//    }
}
