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
use crate::data::databases::tables::rows::row::Row;

#[derive(Debug)]
pub struct Table<Value> {
    pub rows: HashMap<u32, Row<Value>>,
}

impl<Value> Table<Value> {
    pub fn create_table() -> Self {
        Self {
            rows: HashMap::new(),
        }
    }

    pub fn add_row(&mut self, primary_key: u32, row: Row<Value>) {
        self.rows.insert(primary_key, row);
    }

    pub fn delete_row(&mut self, primary_key: u32) {
        self.rows.remove(&primary_key);
    }
}