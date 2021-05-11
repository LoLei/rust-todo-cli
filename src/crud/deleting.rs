use crate::{CrudHandler, DbDriver, RtcError, RtcResult, RunReturn, TodoItem};
use colored::*;

impl<T: DbDriver> CrudHandler<T> {
    pub fn delete(&mut self, id: &str) -> RtcResult<RunReturn> {
        println!("Running delete for id: '{}'", id.green());

        self.db_wrapper
            .driver
            .error_if_db_empty("Deletion process stopping")?;

        let item = self
            .db_wrapper
            .driver
            .get::<TodoItem>(id)
            .ok_or(RtcError::ItemNotFound(format!(
                "ID '{}' does not exist.",
                id
            )))?;

        match self.db_wrapper.driver.del(id) {
            Ok(true) => {
                println!(
                    "Deleted item with ID: '{}' ('{}')",
                    id.blue(),
                    item.name().green()
                );
                Ok(RunReturn::Deletion(true))
            }
            Ok(false) => Err(RtcError::ItemNotFound(format!(
                "ID '{}' does not exist.",
                id
            ))),
            Err(e) => Err(RtcError::Deletion(format!("Error: {}", e))),
        }
    }
}
