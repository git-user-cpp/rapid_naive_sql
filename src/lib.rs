/*
 * NaiveSQL implemented in Rust.
 * Copyright (C) 2024  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! # Rapid Naive SQL
//!
//! `Rapid Naive SQL` is a database API designed for high-performance data management

use database::database::Database;
use std::collections::HashMap;
pub mod database;
pub mod functionality;
pub mod queries;

/// Creates environment for holding databases
///
/// # Examples
///
/// ```
/// use rapid_naive_sql::RNSQL;
/// use rapid_naive_sql::database::database::Database;
///
/// let mut project: RNSQL<String> = RNSQL::new();
/// let db1 = Database::create_database();
/// project.add_database("database name", db1);
/// ```
#[derive(Debug)]
pub struct RNSQL<Value> {
    pub databases: HashMap<String, Database<Value>>,
}

impl<Value> RNSQL<Value> {
    /// Creates an empty environment for holding databases
    pub fn new() -> Self {
        Self {
            databases: HashMap::new(),
        }
    }

    /// Adds database to the environment HashMap
    pub fn add_database(&mut self, name: &str, database: Database<Value>) {
        self.databases.insert(name.to_string(), database);
    }

    /// Removes database from the environment HashMap
    pub fn delete_database(&mut self, name: &str) {
        self.databases.remove(name);
    }
}
