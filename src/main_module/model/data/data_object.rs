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

#[derive(Debug)]
pub struct DataObject {
    field_name: String,
    value: String,
}

impl DataObject {
    pub fn new(field_name: &str, field_value: &str) -> Self {
        DataObject {
            field_name: String::from(field_name),
            value: String::from(field_value),
        }
    }
}
