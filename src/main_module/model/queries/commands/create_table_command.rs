/*
 * MIT License
 *
 * Copyright (c) 2023 Andrew Kushyk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
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
