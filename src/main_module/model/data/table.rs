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

use crate::main_module::model::data::data_object::DataObject;

pub struct Table {
    error_id: String,
    error_arg: String,
    error_rws: String,
    column_names: HashSet<String>,
    rows: HashMap<String, Vec<DataObject>>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            error_id: String::from("[ERROR] [Incorrect ID]"),
            error_arg: String::from("[ERROR] [Incorrect rows in insert data package"),
            error_rws: String::from("[ERROR] [Trying to reach an empty row]"),
            column_names: HashSet::new(),
            rows: HashMap::new(),
        }
    }

    fn generate_unique_id() {
        todo!()
    }

    fn verify_id() {
        todo!()
    }

    fn verify_field() {
        todo!()
    }

    fn verify_rows() {
        todo!()
    }

    fn verify_column_names_correct() {
        todo!()
    }

    fn verify_id_and_field() {
        todo!()
    }

    fn verify_id_and_columns() {
        todo!()
    }

    pub fn row_is_empty() {
        todo!()
    }

    pub fn get_field_value() {
        todo!()
    }

    pub fn add_row() {
        todo!()
    }

    pub fn return_row() {
        todo!()
    }

    pub fn remove_row() {
        todo!()
    }

    pub fn update_row_field_values() {
        todo!()
    }
}
