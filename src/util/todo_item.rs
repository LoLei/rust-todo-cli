use crate::RtcError;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Done,
    Open,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Could also implement From only
impl FromStr for Status {
    type Err = RtcError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "open" => Ok(Status::Open),
            "done" => Ok(Status::Done),
            _ => Err(RtcError::Parse(String::from(s))),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItem {
    id: usize,
    name: String,
    pub status: Status,
    #[serde(with = "ts_milliseconds")]
    creation_date: DateTime<Utc>,
}

impl TodoItem {
    pub(crate) fn new(id: usize, name: &str) -> TodoItem {
        TodoItem {
            id,
            name: String::from(name),
            status: Status::Open,
            creation_date: Utc::now(),
        }
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    pub(crate) fn print_multiple(items: &[TodoItem]) {
        for item in items {
            println!("{}", item);
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: '{}', name: '{}', status: '{}', creation_date: '{}'",
            self.id.to_string().blue(),
            self.name.blue(),
            match self.status {
                Status::Open => self.status.to_string().yellow(),
                Status::Done => self.status.to_string().green(),
            },
            self.creation_date.to_string().blue(),
        )
    }
}
