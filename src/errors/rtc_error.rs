use std::fmt;

pub type Result<T> = std::result::Result<T, RtcError>;

#[derive(Debug)]
pub enum RtcError {
    Creation(String),
    Deletion(String),
    Update(String),
    ItemNotFound(String),
    NoItems(String),
    Parse(String),
}

impl fmt::Display for RtcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RtcError::Creation(msg) => write!(f, "Could not create item: {}", msg),
            RtcError::Deletion(msg) => write!(f, "Could not delete item: {}", msg),
            RtcError::Update(msg) => write!(f, "Could not update item: {}", msg),
            RtcError::ItemNotFound(msg) => write!(f, "Item not found: {}", msg),
            RtcError::NoItems(msg) => write!(f, "No items in database: {}", msg),
            RtcError::Parse(msg) => write!(f, "Failed to parse: {}", msg),
        }
    }
}
