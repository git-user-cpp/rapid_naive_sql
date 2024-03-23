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
use rapid_naive_sql::RNSQL;
 
fn main() {
   let mut project = RNSQL::new();

   println!("{:#?}\n\n", project);

   project.add_database(String::from("db1"));

   println!("{:#?}\n\n", project);

   project.databases[0].add_table(String::from("tb1"));

   println!("{:#?}\n\n", project);

   project.databases[0].tables[0].add_row(1);

   println!("{:#?}\n\n", project);

   project.databases[0].tables[0].rows[0].create_column(String::from("name"));

   println!("{:#?}\n\n", project);

   project.databases[0].tables[0].rows[0].delete_column(String::from("name"));

   println!("{:#?}\n\n", project);

   project.databases[0].tables[0].delete_row(1);

   println!("{:#?}\n\n", project);

   project.databases[0].delete_table(String::from("tb1"));

   println!("{:#?}\n\n", project);

   project.delete_database(String::from("db1"));

   println!("{:#?}", project);
}