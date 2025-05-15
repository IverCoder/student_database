/*
 * SPDX-FileCopyrightText: © 2025 Iverson Briones <ivercoder@proton.me>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 * 
 * Copyright © 2025 Iverson Briones <ivercoder@proton.me>
 * 
 * This file is part of `student_database`.
 * 
 * `student_database` is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * `student_database` is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
 * details.
 * 
 * You should have received a copy of the GNU General Public License along with
 * `student_database`. If not, see <https://www.gnu.org/licenses/>.
 */

use student_database::data::{Gender, GradeLevel, Student};

fn main() {
    println!("Hello, world!");
    let _student = Student {
        first_name: String::from("Firstname"),
        last_name: String::from("Lastname"),
        year_of_birth: 13001,
        month_and_date_of_birth: 1982,
        gender: Gender::Male,
        lrn: 123456789012345,
        school_id: 12345678,
        grade_level: GradeLevel::Masteral,
        last_grade_average: 255,
    };
}