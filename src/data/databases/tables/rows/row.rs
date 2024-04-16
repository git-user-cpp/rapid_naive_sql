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

#[derive(Debug)]
pub struct Row<Value> {
    pub columns: HashMap<String, Value>,
}

impl<Value> Row<Value> {
    pub fn create_row() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    pub fn create_column(&mut self, name: &str, value: Value) {
        self.columns.insert(name.to_string(), value);
    }

    pub fn delete_column(&mut self, name: &str) {
        self.columns.remove(name);
    }
}