use core::fmt;
use std::io::{self};

// #[derive(Debug,Default)]
// enum Departments {
//     Engineering,
//     Business,
//     ComputerScience,
//     Archtitecture,
// }
#[derive(Debug)]
struct Person {
    name: String,
    id: u32,
    courses: Vec<String>,
    // department: Departments,
    department: String,
    role: Role,
}

#[allow(dead_code)]
enum AccessLevel {
    StudentOnly,
    StaffOnly,
    AdminOnly,
    Public,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Role {
    Student,
    Teacher,
    Staff,
    Principal,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Student {
    person: Person,
    grade: Option<u8>,
    attendance_percentage: f64,
    area: Vec<String>,
}

#[allow(dead_code)]
struct Teacher {
    person: Person,
    years_experience: u8,
    salary: f64,
}

#[allow(dead_code)]
struct Staff {
    person: Person,
    department: String,
    salary: f64,
    area: Vec<String>,
}

fn main() {
    // let person_1 = Person::new_person("benjamin".to_owned(), 1);
    let mut _student_areas = vec![
        String::from("Mess hall"),
        String::from("Bathroom"),
        String::from("Toilet"),
        String::from("Principle office"),
        String::from("Club hall"),
        String::from("Doctors office"),
    ];
    _student_areas.push("Club House".to_owned());
    _student_areas.push("Principal office".to_owned());

    let mut _staffs_areas = vec![
        String::from("Mess hall"),
        String::from("Bathroom"),
        String::from("Toilet"),
        String::from("Principle office"),
        String::from("Club hall"),
        String::from("Doctors office"),
    ];
    _staffs_areas.push("Garage".to_owned());
    _staffs_areas.push("Principals office".to_owned());

    let mut _teacher_areas = vec![
        String::from("Mess hall"),
        String::from("Bathroom"),
        String::from("Toilet"),
        String::from("Principle office"),
        String::from("Club hall"),
        String::from("Doctors office"),
    ];
    _teacher_areas.push("Laboratory".to_string());

    let person_1 = Person::default();
    println!("{:?}", person_1);

    println!("Hello, world!");

    // let student_areas=Vec::new();
    // student_areas.push("")
    let student_1 = Student::student_new(person_1, Some(33), 89.0, _student_areas);
    println!("student 1:{}", student_1);
}

#[allow(dead_code)]
impl Person {
    fn new_person(
        name: String,
        id: u32,
        courses: Vec<String>,
        department: String,
        role: Role,
    ) -> Self {
        Self {
            name,
            id,
            courses,
            department,
            role,
        }
    }
}

#[allow(dead_code)]
impl Teacher {
    fn new(person: Person, years_experience: u8, salary: f64) -> Self {
        Self {
            person,
            years_experience,
            salary,
        }
    }

    fn teacher_name(&'a self) -> &'a str {
        &self.person.name
    }
    // fn teacher_id(&self) -> u32 {
    //     self.person.id
    // }
    // fn teacher_role(&self) -> Role {
    //     self.person.role.clone()
    // }
    fn teacher_introduce(&self) {
        println!("{}", self)
    }
}

#[allow(dead_code)]
impl Student {
    fn student_new(
        person: Person,
        grade: Option<u8>,
        attendance_percentage: f64,
        area: Vec<String>,
    ) -> Self {
        Self {
            person,
            grade,
            attendance_percentage,
            area,
        }
    }
    fn areas_to_access(&self) -> Vec<String> {
        self.area.clone()
    }
    fn display_areas(&self) {
        println!("areas to explore:{:#?}", self.area)
    }

    fn can_access(&self, area_name: String) -> Result<(), io::Error> {
        if area_name.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot be empty",
            ));
        }
        // let result=areas.iter().find(|&search| *search ==area_name);
        let result = self.area.iter().find(|&search| *search == area_name);
        match result {
            Some(allowed) => {
                println!("allowed\n{}", allowed);
                Ok(())
                // print!("allowed\n,{}",allowd);
                // std::io::stdout().flush()?;
            }
            None => {
                println!("not found");

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{} searched:,area not found", area_name),
                ))
            }
        }
    }
}

#[allow(dead_code)]
impl Staff {
    fn new(person: Person, department: String, salary: f64, area: Vec<String>) -> Self {
        Self {
            person,
            department,
            salary,
            area,
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        // fn default(&mut self) -> Self {
        Person {
            name: "john doe".to_owned(),
            id: 0,
            courses: vec!["Mathematics".to_owned(), "English".to_owned()],
            department: String::from("Engineering"),
            role: Role::Student,
            // department: Departments::Business,
        }
    }
}

// impl Default for Departments{
//     fn default() -> Self {
//         Departments::ComputerScience
//     }
// }

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Student info:\nstudent:{}\ngrade:{}\nattendance:{}\n",
            self.person,
            self.grade.unwrap_or_default(),
            // match self.grade{
            // Some(person_grade)=>println!("grade:{}",person_grade),

            // Some(person_grade)=>person_grade.to_string(),
            // None=>eprint,
            //
            // Some(person_grade)=>person_grade,
            // None=>eprint,
            // self.grade.unwrap_or_default(),
            self.attendance_percentage
        )
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person info:name:{}\nid:{}\ncourse:{}\ndepartment:{}\n",
            self.name,
            self.id,
            self.courses.join(","),
            self.department
        )
    }
}

impl fmt::Display for Teacher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Teacher information\n
person:{}\nyears of experience:{}\nsalary:{}",
            self.person, self.years_experience, self.salary
        )
    }
}

impl fmt::Display for Staff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Staffs information:\n{}\ndepartment:{}\n,salary:{}\n",
            self.person, self.department, self.salary
        )
    }
}

#[allow(dead_code)]
trait PersonInformation {
    fn print_id_card(&self);
    fn name(&self) -> &str;
    fn id(&self) -> u32;
    fn courses(&self) -> Vec<String>;
    fn department(&self) -> String;
    fn roles(&self) -> Role;
}

trait Areas {
    fn search_areas(&self, area_search: String) -> Result<(), io::Error>;
    fn any_search_areas(&self, area_search: String) -> Result<(), io::Error>;
}

impl Areas for Staff {
    fn search_areas(&self, area_search: String) -> Result<(), io::Error> {
        let area_searched = self
            .area
            .iter()
            .find(|&search_parameter| *search_parameter == area_search);
        match area_searched {
            Some(found) => {
                println!("found area:{}:user_input{}", found, area_search);
                Ok(())
            }
            None => {
                println!("not found:{}", area_search);

                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "input doesnt exist".to_string(),
                ))
            }
        }
    }

    fn any_search_areas(&self, area_search: String) -> Result<(), io::Error> {
        let area_search_result = self
            .area
            .iter()
            .any(|search_parameter| *search_parameter == area_search);
        if area_search_result {
            println!("found:{}", area_search);
            Ok(())
        } else {
            println!("not found:{}", area_search);
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cannot find search parameter".to_owned(),
            ))
        }
    }
}
impl Areas for Student {
    fn search_areas(&self, area_search: String) -> Result<(), io::Error> {
        let area_search = self
            .area
            .iter()
            .find(|&search_parameter| *search_parameter == area_search);
        match area_search {
            Some(found) => {
                println!("found area:{}", found);

                Ok(())
            }
            None => {
                println!("not found:{:?}", area_search);
                return Err(io::Error::new(io::ErrorKind::NotFound, "wrong input"));
            }
        }
    }
    fn any_search_areas(&self, area_search: String) -> Result<(), io::Error> {
        let area_search_result = self
            .area
            .iter()
            .any(|search_parameter| *search_parameter == area_search);
        if area_search_result {
            println!("found:{}", area_search);
            Ok(())
        } else {
            println!("not found");
            Err(io::Error::new(io::ErrorKind::NotFound, "nan".to_string()))
        }
    }
}

impl PersonInformation for Staff {
    fn print_id_card(&self) {
        println!("id information:{}", self)
    }
    fn name(&'a self) -> &'a str {
        &self.person.name
        // format!("Teacher name:{}",&self.person.name)
    }
    fn id(&self) -> u32 {
        self.person.id
    }
    fn courses(&self) -> Vec<String> {
        self.person.courses.clone()
    }
    fn department(&self) -> String {
        self.person.department.clone()
    }

    fn roles(&self) -> Role {
        self.person.role.clone()
    }
}

impl PersonInformation for Teacher {
    fn print_id_card(&self) {
        println!("id information:{}", self)
    }
    fn name(&'a self) -> &'a str {
        &self.person.name
        // format!("Teacher name:{}",&self.person.name)
    }
    fn id(&self) -> u32 {
        self.person.id
    }
    fn courses(&self) -> Vec<String> {
        self.person.courses.clone()
    }
    fn department(&self) -> String {
        self.person.department.clone()
    }

    fn roles(&self) -> Role {
        self.person.role.clone()
    }
}
// impl PersonInformation for  Student{

// }
// impl PersonInformation for Staff{

// }

// #[allow(dead_code)]
// fn print_information<T>(person_type: &T)
//     where
//     T:PersonInformation + Clone
// {

//     println!(
//         "person name:{}\nperson id:\nperson courses:{},department:{}roles:{}",
//         person_type.name,
//         person_type.id,
//         person_type.courses,
//         person_type.department,
//         person_type.roles
//     )
// }

// fn check_exsit<T:Area>(person_type: &T)->bool{
//     let result=
// }

// fn print_id_card(&self);
// fn name(&self) -> &str;
// fn id(&self) -> u32;
// fn courses(&self) -> Vec<String>;
// fn department(&self) -> String;
// fn roles(&self) -> Role;



    // fn search_areas(&self, area_search: String) -> Result<(), io::Error>;
    // fn any_search_areas(&self, area_search: String) -> Result<(), io::Error>;
