use student_database::data::{Gender, GradeLevel, Student};

fn main() {
    println!("Hello, world!");
    let _student = Student {
        first_name: String::from("Iverson"),
        last_name: String::from("Briones"),
        year_of_birth: 13001,
        month_and_date_of_birth: 1982,
        gender: Gender::Male,
        lrn: 123456789012345,
        school_id: 12345678,
        grade_level: GradeLevel::Masteral,
        last_grade_average: 255,
    };
}