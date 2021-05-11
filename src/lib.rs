use crate::util::db::pickle_db_impl;
use pickledb::PickleDb;

// Re-exports
pub use crate::crud::CrudHandler;
pub use crate::errors::rtc_error::{Result as RtcResult, RtcError};
pub use crate::util::config::{Config, Operation};
pub use crate::util::db::database_wrapper::{DatabaseWrapper, DbDriver};
pub use crate::util::todo_item::{Status, TodoItem};

use colored::*;
use std::collections::HashMap;
use std::str::FromStr;

mod crud;
mod errors;
mod util;

#[derive(Debug)]
pub enum RunReturn {
    Addition(usize),
    Deletion(bool),
    Update(bool),
    Filter(Vec<TodoItem>),
    GetAll(Vec<TodoItem>),
}

/// # Errors
/// Will return propagated errors of type [`RtcError`]
pub fn run(config: &Config) -> RtcResult<RunReturn> {
    let db = pickle_db_impl::init_pickle_db(&config.database_file());
    let db_wrapper = DatabaseWrapper::new(db);
    let mut crud_handler = CrudHandler::<PickleDb>::new(db_wrapper);

    match config.operation() {
        Operation::Add => run_add_test::<PickleDb>(&mut crud_handler, config.args()),
        Operation::Delete => run_delete_test::<PickleDb>(&mut crud_handler, config.args()),
        Operation::Update => run_update_test::<PickleDb>(&mut crud_handler, config.args()),
        Operation::Filter => run_filter_test::<PickleDb>(&crud_handler, config.args()),
        Operation::GetAll => run_getall_test::<PickleDb>(&crud_handler),
    }
}

fn run_add_test<T: DbDriver>(
    crud_handler: &mut CrudHandler<T>,
    args: &[String],
) -> RtcResult<RunReturn> {
    let name = args.iter().fold(String::from(""), |acc, it| acc + " " + it);
    crud_handler.add(&name.trim())
}

fn run_delete_test<T: DbDriver>(
    crud_handler: &mut CrudHandler<T>,
    args: &[String],
) -> RtcResult<RunReturn> {
    let id = &args[0];
    crud_handler.delete(id)
}

fn run_update_test<T: DbDriver>(
    crud_handler: &mut CrudHandler<T>,
    args: &[String],
) -> RtcResult<RunReturn> {
    let id = &args[0];
    let status = &args[1];
    crud_handler.update(id, Status::from_str(status)?)
}

fn run_filter_test<T: DbDriver>(
    crud_handler: &CrudHandler<T>,
    args: &[String],
) -> RtcResult<RunReturn> {
    let usage = format!(
        "{}\n\t{}\n\t{}\n",
        "USAGE for filter:".bold().yellow(),
        "any/all of name=substring status=(done|open) before=date(iso) after=date(iso)".yellow(),
        "e.g. -f name='my item' status=done before=2021-01-01 after=2020-01-01".yellow(),
    );

    if args.len() > 4 {
        print!("{}", usage);
        return Err(RtcError::Parse(String::from(
            "Length of arguments is greater than 4",
        )));
    }

    let mut arg_map = HashMap::new();

    for arg in args {
        let kv = arg.split('=').collect::<Vec<&str>>();
        if kv.len() != 2 {
            print!("{}", usage);
            return Err(RtcError::Parse(format!("{:?}", kv)));
        }
        arg_map.insert(kv[0], kv[1]);
    }

    crud_handler.filter(
        arg_map.get("name").cloned(),
        arg_map.get("status").cloned(),
        arg_map.get("before").cloned(),
        arg_map.get("after").cloned(),
    )
}

fn run_getall_test<T: DbDriver>(crud_handler: &CrudHandler<T>) -> RtcResult<RunReturn> {
    crud_handler.filter(None, None, None, None)
}
