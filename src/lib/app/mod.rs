pub mod command;
pub mod color;

use command::Command;

pub struct App {
    pub name: String,
    pub usage: String,
    pub version: String,
    pub commands: Vec<Command>,
}

impl App {
    pub fn new(name: String, usage: String, version: String, commands: Vec<Command>) -> Self {
        Self {name, usage, version, commands}
    }

    pub fn run(&self, args: Vec<String>) {
        let (cmd, owner_repo) = match args.len() {
            3 => ((&args[1]).clone(), (&args[2]).clone()),
            _ => (String::new(), String::new())
        };

        match (cmd.len(), owner_repo.len()) {
            (0, _) | (_, 0) => {
                self.help();
                std::process::exit(1);
            }
            _ => ()
        }

        match self.select_command(&cmd) {
            Some(command) => (command.action)(owner_repo),
            None => self.help()
        }
    }

    fn help(&self) {
        println!("NAME:\n   {}\n", self.name);
        println!("USAGE:\n   {}\n", self.usage);
        println!("VERSION:\n   {}\n", self.version);

        println!("COMMAND:");
        for c in &self.commands {
            println!("   {} {} {}", self.name, c.name, "[Owner/Repo]")
        }
    }

    fn select_command(&self, cmd: &String) -> Option<&Command> {
       (&self.commands).into_iter()
            .find(|command| &command.name == cmd)
    }
}