use crate::{DatabaseWrapper, DbDriver, RtcError, RtcResult, RunReturn, Status, TodoItem};
use chrono::{DateTime, Utc};
use colored::*;

pub struct CrudHandler<T: DbDriver> {
    db_wrapper: DatabaseWrapper<T>,
}

impl<T: DbDriver> CrudHandler<T> {
    pub fn new<U: DbDriver>(db_wrapper: DatabaseWrapper<U>) -> CrudHandler<U> {
        CrudHandler { db_wrapper }
    }

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

    pub fn filter(
        &self,
        name_substr: Option<&str>,
        status: Option<&str>,
        before_date: Option<&str>,
        after_date: Option<&str>,
    ) -> RtcResult<RunReturn> {
        if name_substr.is_none()
            && status.is_none()
            && before_date.is_none()
            && after_date.is_none()
        {
            println!("Running getall");
        } else {
            print!("Running filter with: ");
            println!(
                "name: '{}', status: '{}', before_date: '{}', after_date: '{}'",
                name_substr.unwrap_or_default().green(),
                status.unwrap_or_default().green(),
                before_date.unwrap_or_default().green(),
                after_date.unwrap_or_default().green(),
            );
        }

        self.db_wrapper
            .driver
            .error_if_db_empty("Filter process stopping")?;

        let mut results: Vec<TodoItem> = self.db_wrapper.driver.get_all::<TodoItem>();

        if let Some(n) = name_substr {
            results = CrudHandler::<T>::filter_items(results, |item| item.name().contains(n))
        };

        if let Some(s) = status {
            results = CrudHandler::<T>::filter_items(results, |item| {
                item.status.to_string().to_lowercase() == s.to_lowercase()
            })
        };

        if let Some(d) = before_date {
            let before_date = CrudHandler::<T>::get_date_from_str(d)?;
            results =
                CrudHandler::<T>::filter_items(results, |item| item.creation_date() < &before_date)
        };

        if let Some(d) = after_date {
            let after_date = CrudHandler::<T>::get_date_from_str(d)?;
            results =
                CrudHandler::<T>::filter_items(results, |item| item.creation_date() > &after_date)
        };

        if results.is_empty() {
            println!("Found no items");
        } else {
            println!("Found {} items:", results.len());
        }

        results.sort_by(|a, b| a.id().cmp(&b.id()));
        TodoItem::print_multiple(&results);
        Ok(RunReturn::Filter(results))
    }

    fn get_date_from_str(date_str: &str) -> RtcResult<DateTime<Utc>> {
        let iso_date_str = String::from(date_str) + " 00:00:00.000 +0000";
        match iso_date_str.parse::<DateTime<Utc>>() {
            Ok(d) => Ok(d),
            Err(e) => Err(RtcError::Parse(format!("'{}', {}", date_str, e))),
        }
    }

    fn filter_items<F>(items: Vec<TodoItem>, predicate: F) -> Vec<TodoItem>
    where
        F: Fn(&TodoItem) -> bool,
    {
        items
            .into_iter()
            .filter(predicate)
            .collect::<Vec<TodoItem>>()
    }
}
