extern crate clap;

use clap::{App, Arg, ArgGroup};
use colored::*;

use rust_todo_cli::{run, Config, RtcResult};

fn main() -> RtcResult<()> {
    let matches = App::new("Rust TODO CLI")
        .version("0.1")
        .author("Lorenz Leitner")
        .about("CLI for managing TODOs")
        .arg(
            Arg::with_name("add")
                .help("Add an item, return ID")
                .short("a")
                .long("add")
                .takes_value(true)
                .multiple(true)
                .value_name("NAME"),
        )
        .arg(
            Arg::with_name("delete")
                .help("Delete an item")
                .short("d")
                .long("delete")
                .takes_value(true)
                .value_name("ID"),
        )
        .arg(
            Arg::with_name("update")
                .help("Update an item")
                .short("u")
                .long("update")
                .takes_value(true)
                .number_of_values(2)
                .value_name("ID,STATUS"),
        )
        .arg(
            Arg::with_name("filter")
                .help("Filter items by name, status and/or date")
                .short("f")
                .long("filter")
                .takes_value(true)
                .multiple(true)
                .value_name("FILTERS"),
        )
        .arg(
            Arg::with_name("getall")
                .help("Get all items in the database")
                .short("g")
                .long("getall"),
        )
        .group(
            ArgGroup::with_name("req_cmds")
                .args(&["add", "delete", "update", "filter", "getall"])
                .required(true),
        )
        .get_matches();

    let config = match Config::new(&matches) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    match run(&config) {
        Ok(_) => Ok(()),
        Err(x) => {
            eprintln!("{}", format!("{}", x).bold().red());
            Err(x)
        }
    }
}
