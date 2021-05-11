use rust_todo_cli::{Config, Operation};
use std::fs;

pub fn create_config(operation: Operation, args: Vec<String>) -> Config {
    Config::new_for_testing(operation, args, "testcase.json")
}

pub fn remove_db() -> std::io::Result<()> {
    fs::remove_file("testcase.json")
}
