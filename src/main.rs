mod lib;

use std::env;

use seahorse::{App, color};

use lib::commands;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut app = App::new();

    app.name = "gstats".to_string();
    app.display_name = color::magenta("
     ██████╗ ███████╗████████╗ █████╗ ████████╗███████╗
    ██╔════╝ ██╔════╝╚══██╔══╝██╔══██╗╚══██╔══╝██╔════╝
    ██║  ███╗███████╗   ██║   ███████║   ██║   ███████╗
    ██║   ██║╚════██║   ██║   ██╔══██║   ██║   ╚════██║
    ╚██████╔╝███████║   ██║   ██║  ██║   ██║   ███████║
     ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚══════╝");
    app.usage = "gstats [command] [Owner/Repo]".to_string();
    app.version = env!("CARGO_PKG_VERSION").to_string();
    app.commands = vec![commands::release_command()];

    app.run(args.clone());
}
