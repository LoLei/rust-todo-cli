use crate::{CrudHandler, DbDriver, RtcError, RtcResult, RunReturn, TodoItem};
use chrono::{DateTime, Utc};
use colored::*;

impl<T: DbDriver> CrudHandler<T> {
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
