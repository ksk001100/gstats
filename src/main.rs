mod lib;

use std::env;

use seahorse::{App, color};

use lib::commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let display_name = color::magenta("
     ██████╗ ███████╗████████╗ █████╗ ████████╗███████╗
    ██╔════╝ ██╔════╝╚══██╔══╝██╔══██╗╚══██╔══╝██╔════╝
    ██║  ███╗███████╗   ██║   ███████║   ██║   ███████╗
    ██║   ██║╚════██║   ██║   ██╔══██║   ██║   ╚════██║
    ╚██████╔╝███████║   ██║   ██║  ██║   ██║   ███████║
     ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝   ╚══════╝");

    let app = App::new()
        .name("gstats")
        .display_name(display_name)
        .usage("gstats [command] [Owner/Repo]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![
            commands::release_command(),
            commands::clone_command(),
            commands::view_command()
        ]);

    app.run(args);
}
