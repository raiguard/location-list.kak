#![allow(dead_code, unused)]

use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

mod types;
mod util;

use types::*;

#[derive(StructOpt)]
#[structopt(
    name = "kak-loli",
    about = "An implementation of location lists for kakoune."
)]
struct App {
    #[structopt(short, long)]
    client_name: Option<String>,

    #[structopt(short, long)]
    session_name: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Prints initialization kakscript
    Init,
    Grep {
        filename: PathBuf,
    },
    /// Creates a new location list
    New {
        list: String,
    },
    Next,
    Prev,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();

    let option_name = match app.client_name {
        Some(ref client_name) => client_name.clone(),
        None => "LOLIGLOBAL".to_string(),
    };

    match app.cmd {
        Command::Init => init(&app),
        Command::Grep { filename } => {
            let input = fs::read_to_string(filename)?;
            let list = LocationList::from_grep(option_name, input)?;
            util::kak_print(&format!("{:#?}", list));
        }
        Command::New { list: input } => {
            let list = LocationList::from_str_list(option_name, input)?;
        }
        _ => (),
    };

    Ok(())
}

fn init(app: &App) {
    // Inline the contents of the file at compile time
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let loli_cmd = format!(
        "set global loli_cmd '{} -s {}'",
        util::editor_escape(cmd),
        app.session_name
    );

    let script: &str = include_str!("../rc/loli.kak");
    let lgrep: &str = include_str!("../rc/lgrep.kak");

    println!("{}\n{}\n{}", script, lgrep, loli_cmd);
}
