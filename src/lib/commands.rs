use seahorse::command::Command;
use seahorse::color;

use crate::lib::github::api::Api;
use crate::lib::traits::Statistics;

pub fn release_command() -> Command {
    Command{name: "release".to_string(), action: release_action}
}

fn release_action(owner_repo: String) {
    let api = Api::new("".to_string());

    match api.release(&owner_repo) {
        Ok(releases) => releases.stats(),
        Err(_) => eprintln!("{}", color::red("Repository does not exist..."))
    }
}