use crate::error::{Result, SQLRiteError};
use crate::sql::db::table::Table;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The database is represented by this structure.assert_eq!
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Database {
    /// Name of this database. (schema name, not filename)
    pub db_name: String,
    /// HashMap of tables in this database
    pub tables: HashMap<String, Table>,
}

impl Database {
    /// Creates an empty `Database`
    ///
    /// # Examples
    ///
    /// ```
    /// let mut db = sql::db::database::Database::new("my_db".to_string());
    /// ```
    pub fn new(db_name: String) -> Self {
        Database {
            db_name,
            tables: HashMap::new(),
        }
    }

    /// Returns true if the database contains a table with the specified key as a table name.
    ///
    pub fn contains_table(&self, table_name: String) -> bool {
        self.tables.contains_key(&table_name)
    }

    /// Returns an immutable reference of `sql::db::table::Table` if the database contains a
    /// table with the specified key as a table name.
    ///
    pub fn get_table(&self, table_name: String) -> Result<&Table> {
        if let Some(table) = self.tables.get(&table_name) {
            Ok(table)
        } else {
            Err(SQLRiteError::General(String::from("Table not found.")))
        }
    }

    /// Returns an mutable reference of `sql::db::table::Table` if the database contains a
    /// table with the specified key as a table name.
    ///
    pub fn get_table_mut(&mut self, table_name: String) -> Result<&mut Table> {
        if let Some(table) = self.tables.get_mut(&table_name) {
            Ok(table)
        } else {
            Err(SQLRiteError::General(String::from("Table not found.")))
        }
    }
}
