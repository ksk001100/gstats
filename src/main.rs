mod lib;

use std::env;

use lib::app::App;
use lib::commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app = App::new(
        "gstats".to_string(),
        "gstats [command] [Owner/Repo]".to_string(),
        "v0.0.1".to_string(),
        vec![
            commands::release_command()
        ],
    );

    app.run(args.clone());
}
