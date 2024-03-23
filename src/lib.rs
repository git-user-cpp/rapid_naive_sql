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

pub(crate) mod databases;
use crate::databases::database::Database;

#[derive(Debug)]
pub struct RNSQL {
    pub databases: Vec<Database>,
}

impl RNSQL {
    pub fn new() -> Self {
        Self {
            databases: Vec::new(),
        }
    }

    pub fn add_database(&mut self, name: String) {
        self.databases.push(Database::create_database(name));
    }

    pub fn delete_database(&mut self, name: String) {
        let mut ind = 0;
        while ind != self.databases.len() {
            if self.databases[ind].name == name {
                self.databases.remove(ind);
            } else {
                ind += 1;
            }
        }
    }
}