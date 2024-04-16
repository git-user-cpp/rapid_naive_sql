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

use std::collections::HashMap;

use super::tables::table::Table;

#[derive(Debug)]
pub struct Database<Value> {
    pub tables: HashMap<String, Table<Value>>,
}

impl<Value> Database<Value> {
    pub fn create_database() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, name: &str, table: Table<Value>) {
        self.tables.insert(name.to_string(), table);
    }

    pub fn delete_table(&mut self, name: &str) {
        self.tables.remove(name);
    }
}