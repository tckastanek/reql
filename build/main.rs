extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;

mod config;
mod commands;

use commands::{Command, Commands};
use config::Config;

fn main() {
    let cfg = Config::new();
    let mut commands = Commands::new(&cfg.menu);

    for item in cfg.menu {
        let dir = format!("{}/{}", cfg.docs_dir.display(), item.section);
        let cmd = Command::new(&dir, item);
        commands.add_command(&cmd);
        println!("cargo:rerun-if-changed={}", cmd.src.display());
    }

    commands.generate(&cfg.cmds_src);
}
