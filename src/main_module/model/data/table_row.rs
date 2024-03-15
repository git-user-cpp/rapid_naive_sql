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

use crate::main_module::model::data::data_object::DataObject;

pub struct TableRow {
    incorrect_field_name: String,
    fields: Vec<DataObject>,
}

impl TableRow {
    pub fn new(fields: Vec<DataObject>) -> Self {
        TableRow {
            incorrect_field_name: String::from("[ERROR] [Incorrect field name]"),
            fields,
        }
    }

    pub fn add_field(data_object: DataObject) {
        todo!()
    }

    pub fn insert_field(data_object: DataObject) {
        todo!()
    }

    pub fn remove_field(field: String) {
        todo!()
    }

    pub fn remove_field_value(field: String) {
        todo!()
    }

    pub fn get_field_data_obj(field: String) {
        todo!()
    }

    pub fn replace_existing_obj_value(current: DataObject, new: DataObject) {
        todo!()
    }

    fn find_data_obj(field: String) {
        todo!()
    }
}
