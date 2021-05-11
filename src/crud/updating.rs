use crate::{CrudHandler, DbDriver, RtcError, RtcResult, RunReturn, Status, TodoItem};
use colored::*;

impl<T: DbDriver> CrudHandler<T> {
    pub fn update(&mut self, id: &str, status: Status) -> RtcResult<RunReturn> {
        println!(
            "Running update with args: '{}', '{}'",
            id.green(),
            status.to_string().green()
        );

        self.db_wrapper
            .driver
            .error_if_db_empty("Update process stopping")?;

        let mut todo_item = match self.db_wrapper.driver.get::<TodoItem>(id) {
            Some(x) => x,
            None => {
                return Err(RtcError::ItemNotFound(format!(
                    "ID '{}' does not exist.",
                    id
                )))
            }
        };

        todo_item.status = status;

        match self.db_wrapper.driver.set(id, &todo_item) {
            Ok(_) => {
                println!("Found item with name: '{}'", todo_item.name().blue());
                println!(
                    "Changed status to: '{}'",
                    todo_item.status.to_string().blue()
                );
                Ok(RunReturn::Update(true))
            }
            Err(e) => Err(RtcError::Update(format!("Error: {}", e))),
        }
    }
}
