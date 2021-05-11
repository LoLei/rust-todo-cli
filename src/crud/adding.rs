use crate::{CrudHandler, DbDriver, RtcError, RtcResult, RunReturn, TodoItem};
use colored::*;

impl<T: DbDriver> CrudHandler<T> {
    pub fn add(&mut self, name: &str) -> RtcResult<RunReturn> {
        println!("Running add for item with name: '{}'", name.green());

        let new_key = self.get_new_key();
        let todo_item = TodoItem::new(new_key, name);

        match self
            .db_wrapper
            .driver
            .set(&(new_key.to_string()), &todo_item)
        {
            Ok(_) => {
                println!("Created item with ID: '{}'", new_key.to_string().blue());
                Ok(RunReturn::Addition(new_key))
            }
            Err(e) => Err(RtcError::Creation(format!("Error: {}", e))),
        }
    }

    fn get_new_key(&mut self) -> usize {
        // Keep total creations as extra db entry instead of getting it via
        // .total_keys, so that deletions do no mess up new additions
        let ktc = &self.db_wrapper.key_total_creations().clone();

        let new_key = self
            .db_wrapper
            .driver
            .get::<usize>(ktc)
            .map_or_else(|| 1, |x| x + 1);
        self.db_wrapper.driver.set::<usize>(ktc, &new_key).unwrap();
        new_key
    }
}
