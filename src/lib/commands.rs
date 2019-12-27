use seahorse::Command;
use seahorse::color;

use crate::lib::github::api::Api;
use crate::lib::traits::Statistics;

pub fn release_command() -> Command {
    Command::new("release", "gstats release [Owner/Repo]", release_action)
}

fn release_action(args: Vec<String>) {
    let owner_repo = args[0].clone();
    let api = Api::new("".to_string());

    match api.release(&owner_repo) {
        Ok(releases) => releases.stats(),
        Err(_) => eprintln!("{}", color::red("Repository does not exist..."))
    }
}

pub fn clone_command() -> Command {
    Command::new("clone", "gstats clone [Owner/Repo]", clone_action)
}

fn clone_action(args: Vec<String>) {
    let owner_repo = args[0].clone();
    let token = env!("GITHUB_TOKEN");
    let api = Api::new(token.to_string());

    match api.clone(&owner_repo) {
        Ok(clone) => clone.stats(),
        Err(_) => eprintln!("{}", color::red("Repository does not exist..."))
    }
}