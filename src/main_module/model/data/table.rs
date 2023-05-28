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
