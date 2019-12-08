pub mod command;

use crate::lib::app::command::Command;

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

//        let cmd: String = match args.len() {
//            0 => {
//                self.help();
//                std::process::exit(1);
//                ""
//            },
//            2 => args[0].clone(),
//            _ => ""
//        };
        let cmd = (&args[0]).clone();

        let command = self.select_command(cmd);
        (command.action)(cmd.clone());
    }

    fn help(&self) {
        println!("NAME:\n   {}\n", self.name);
        println!("USAGE:\n   {}\n\n", self.usage);
        println!("VERSION:\n   {}\n\n", self.version);
    }

    fn select_command(&self, cmd: String) -> &Command {
        (&self.commands)
            .into_iter()
            .filter(|command| command.name == cmd)
            .collect::<Vec<&Command>>()
            .first()
            .unwrap()
    }
}