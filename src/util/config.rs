use clap::ArgMatches;
use colored::*;
use std::env;
use std::fs;

pub enum Operation {
    Add,
    Delete,
    Update,
    Filter,
    GetAll,
}

pub struct Config {
    operation: Operation,
    args: Vec<String>,
    database_file: String,
}

impl Config {
    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn database_file(&self) -> &String {
        &self.database_file
    }

    /// Creates Config from clap CLI arguments
    pub fn new(matches: &ArgMatches) -> Result<Config, std::io::Error> {
        // Is there a better way to do this?
        // I'd like to match on matches or the arg group, i.e. the reverse of this
        let operation;
        let args: Vec<String>;

        if matches.is_present("add") {
            operation = Operation::Add;
            args = Config::get_args_vector("add", matches);
        } else if matches.is_present("delete") {
            operation = Operation::Delete;
            args = vec![matches.value_of("delete").unwrap().to_string()];
        } else if matches.is_present("update") {
            operation = Operation::Update;
            args = Config::get_args_vector("update", matches);
        } else if matches.is_present("filter") {
            operation = Operation::Filter;
            args = Config::get_args_vector("filter", matches);
        } else if matches.is_present("getall") {
            operation = Operation::GetAll;
            args = vec![];
        } else {
            // Else can never occur due to ArgGroup restriction in main.rs
            panic!("This code can never be reached.");
            // >inb4 this code will be reached
        }

        let cache_dir = if let Some(p) = dirs::cache_dir() {
            p
        } else {
            println!("{}", "Warning: Using tmp dir".yellow());
            env::temp_dir()
        };

        fs::create_dir_all(cache_dir.join("rust_todo_cli"))?;

        Ok(Config {
            operation,
            args,
            database_file: String::from(
                cache_dir
                    .join("rust_todo_cli")
                    .join("rtc.json")
                    .to_str()
                    .unwrap(),
            ),
        })
    }

    fn get_args_vector(name: &str, matches: &ArgMatches) -> Vec<String> {
        matches
            .values_of(name)
            .unwrap()
            .map(std::string::ToString::to_string)
            .collect()
    }

    /// Used in test cases
    pub fn new_for_testing(operation: Operation, args: Vec<String>, db_name: &str) -> Config {
        Config {
            operation,
            args,
            database_file: String::from(db_name),
        }
    }
}
