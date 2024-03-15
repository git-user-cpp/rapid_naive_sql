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

use std::collections::{HashMap, HashSet};

use crate::main_module::model::data::table::Table;

pub struct DataHandler {
    tables: HashMap<String, Table>,
}

impl DataHandler {
    pub fn new() -> Self {
        DataHandler {
            tables: HashMap::new(),
        }
    }

    pub fn create_new_table(str: &str, set: HashSet<String>) {
        todo!()
    }

    pub fn insert_into_table() {
        todo!()
    }
}
