mod adding;
mod deleting;
mod filter;
mod updating;

use crate::{DatabaseWrapper, DbDriver};

pub struct CrudHandler<T: DbDriver> {
    db_wrapper: DatabaseWrapper<T>,
}

impl<T: DbDriver> CrudHandler<T> {
    pub fn new<U: DbDriver>(db_wrapper: DatabaseWrapper<U>) -> CrudHandler<U> {
        CrudHandler { db_wrapper }
    }
}
