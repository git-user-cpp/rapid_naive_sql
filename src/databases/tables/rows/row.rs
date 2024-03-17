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

pub struct Row {
    primary_key: u32,
    columns: Vec<String>,
}

impl Row {
    pub fn create_row(primary_key: u32, columns: Vec<String>) -> Self {
        Self {
            primary_key,
            columns,
        }
    }
    
    pub fn create_row_without_columns(primary_key: u32) -> Self {
        Self {
            primary_key,
            columns: Vec::new(),
        }
    }

    pub fn create_column(&mut self, primary_key: u32, name: String) {
        todo!()
    }

    pub fn delete_column(&mut self, name: String) {
        todo!()
    }
}