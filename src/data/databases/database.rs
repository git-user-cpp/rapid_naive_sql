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

use super::tables::table::Table;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn create_database(name: String) -> Self {
        Self {
            name,
            tables: Vec::new(),
        }
    }

    pub fn add_table(&mut self, name: String) {
        self.tables.push(Table::create_table(name));
    }

    pub fn delete_table(&mut self, name: String) {
        let mut ind = 0;
        while ind != self.tables.len() {
            if self.tables[ind].name == name {
                self.tables.remove(ind);
            } else {
                ind += 1;
            }
        }
    }
}