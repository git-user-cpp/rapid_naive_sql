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

use std::collections::HashSet;

use crate::main_module::model::data::data_handler::DataHandler;
use crate::main_module::model::queries::commands::querry_command::QueryCommand;

pub struct CreateTableCommand {}

impl QueryCommand for CreateTableCommand {
    fn execute_command(command_line: String) {
        let binding = command_line.replace("\\);", "");
        let binding = binding.split(" \\(");
        let mut command_addr = binding;

        let table_name = command_addr.next().unwrap();
        let column_name = command_addr.next().unwrap();
        let mut column_names: HashSet<String> = HashSet::new();
        column_names.insert(column_name.to_string());
        let temp: HashSet<String> = HashSet::new();

        DataHandler::create_new_table(table_name, temp);
    }
}
