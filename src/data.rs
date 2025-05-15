//! Handles how every student detail is stored and encoded.
//! 
//! The [Gender] and [GradeLevel] types are declared here, as well as the
//! [Student] struct.

use std::fmt;

/// Types of genders available for a student.
/// 
/// Male, female, and non-binary options are offered. An option to disable the
/// non-binary selection is **not**, will **not** and will **never** be offered.
/// 
/// # Examples
/// ```rust
/// use student_database::data::Gender;
/// let student_gender = Gender::NonBinary;
/// println!("{}", student_gender);
/// ```
pub enum Gender {
    Male, Female, NonBinary,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::NonBinary => write!(f, "Non-binary"),
        }
    }
}

/// Grade levels available for a student.
/// 
/// A student can be as young as nursery to as advanced as doctorate. Please
/// refer to the Philippine curriculum on which this list is based on.
/// # Examples
/// ```rust
/// use student_database::data::GradeLevel;
/// let student_grade_level = GradeLevel::Grade11;
/// println!("{}", student_grade_level);
/// ```
pub enum GradeLevel {
    Nursery, Kindergarten, Preparatory,
    Grade1, Grade2, Grade3, Grade4, Grade5, Grade6,
    Grade7, Grade8, Grade9, Grade10, Grade11, Grade12,
    CollegeFirstYear, CollegeSecondYear, CollegeThirdYear, CollegeFourthYear,
    Masteral, Doctorate,
}

impl fmt::Display for GradeLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Nursery => write!(f, "Nursery"),
            Self::Kindergarten => write!(f, "Kindergarten"),
            Self::Preparatory => write!(f, "Preparatory"),
            Self::Grade1 => write!(f, "Grade 1"),
            Self::Grade2 => write!(f, "Grade 2"),
            Self::Grade3 => write!(f, "Grade 3"),
            Self::Grade4 => write!(f, "Grade 4"),
            Self::Grade5 => write!(f, "Grade 5"),
            Self::Grade6 => write!(f, "Grade 6"),
            Self::Grade7 => write!(f, "Grade 7"),
            Self::Grade8 => write!(f, "Grade 8"),
            Self::Grade9 => write!(f, "Grade 9"),
            Self::Grade10 => write!(f, "Grade 10"),
            Self::Grade11 => write!(f, "Grade 11"),
            Self::Grade12 => write!(f, "Grade 12"),
            Self::CollegeFirstYear => write!(f, "First Year College"),
            Self::CollegeSecondYear => write!(f, "Second Year College"),
            Self::CollegeThirdYear => write!(f, "Third Year College"),
            Self::CollegeFourthYear => write!(f, "Fourth Year College"),
            Self::Masteral => write!(f, "Masteral"),
            Self::Doctorate => write!(f, "Doctorate"),
        }
    }
}

/// The structure responsible for managing student data.
pub struct Student {
    /// A [`String`] containing the student's first name as indicated in their birth
    /// certificate.
    pub first_name: String,
    /// A [`String`] containing the student's last name as indicated in their birth
    /// certificate.
    pub last_name: String,
    /// The year the student is born, in YYYY format (e.g. 2009). Stored as [`u16`].
    /// **This is a temporary solution and will be replaced by a more robust way to
    /// store dates.**
    pub year_of_birth: u16, // YYYY format
    /// The month and day the student is born, in MMDD format
    /// (e.g. 0420 for April 20). Stored as [`u16`]. **This is a temporary solution
    /// and will be replaced by a more robust way to store dates.**
    pub month_and_date_of_birth: u16, // MMDD format
    /// The gender of the student. See [`Gender`] for more details.
    pub gender: Gender,
    /// The Learner's Reference Number (LRN) of the student. See DepEd rules for
    /// full details about an LRN. Usually a twelve-digit number, where the first
    /// six numbers is the school ID of the school that first registered the
    /// student into DepEd's registry (most likely the school the student
    /// went to for Nursery or Kindergarten). Once assigned by DepEd, this cannot
    /// be changed barring extraordinary circumstances.
    pub lrn: u64, // twelve-digit number
    /// The school ID of the student's current school. Usually a six-digit number.
    pub school_id: u32, // six-digit number
    /// The current grade level of the student. See [`GradeLevel`] for more details.
    pub grade_level: GradeLevel,
    /// The last grade average of a student. For example, if a student is currently
    /// in Grade 6, this will be their grade average from Grade 5. Set this to zero
    /// for students without a previous grade average
    /// (e.g. Nursery or Kindergarten students).
    pub last_grade_average: u8, // from 0 to 100, most likely to be in 70-97 range
}

impl Student {
    const INVALID_YEAR:&'static str = "Invalid year of birth.";
    const INVALID_MONTH_DATE: &'static str = "Invalid month and/or date of birth.";
    const INVALID_LRN: &'static str = "Invalid LRN.";
    const INVALID_SCHOOL_ID: &'static str = "Invalid LRN.";
    const INVALID_LAST_GRADE_AVERAGE: &'static str = "Invalid last grade average.";
    pub fn new(first_name: String, last_name: String, year_of_birth: u16,
        month_and_date_of_birth: u16, gender: Gender, lrn: u64, school_id: u32,
        grade_level: GradeLevel, last_grade_average: u8,
    ) -> Self {
        if year_of_birth > 9999 {
            panic!("{}", Self::INVALID_YEAR);
        } else if month_and_date_of_birth > 1231 {
            panic!("{} Date exceeds December 31 (`1231`).", Self::INVALID_MONTH_DATE);
        } else if month_and_date_of_birth < 0101 {
            panic!("{} Date is earlier than January 1 (`0101`).", Self::INVALID_MONTH_DATE);
        } else if lrn > 999999999999 {
            panic!("{} Exceeds 12 digits.", Self::INVALID_LRN);
        } else if school_id > 999999 {
            panic!("{} Exceeds 6 digits.", Self::INVALID_SCHOOL_ID);
        } else if last_grade_average > 100 {
            panic!("{} Exceeds 100.", Self::INVALID_LAST_GRADE_AVERAGE);
        }
        Self {
            first_name, last_name, year_of_birth, month_and_date_of_birth, gender, lrn,
            school_id, grade_level, last_grade_average,
        }
    }
}