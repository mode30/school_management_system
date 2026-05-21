use core::fmt;
// use core::fmt;
use std::collections::HashMap;
#[allow(dead_code)]

struct Students {
    name:String,
    //string -> courses:options<u8>-> grades
    grades:HashMap<String,Option<u8>>
    // students: Vec<GradeBook>,
}

fn main() {
    let mut person_1 = Students::new("benjamin".to_owned());

    person_1.add_grade("benjamin".to_owned(), Some(9));
    person_1.add_grade("carson".to_owned(), Some(10));
    person_1.add_grade("kevin".to_owned(), Some(3));
    person_1.add_grade("jonathan".to_owned(), Some(11));
    person_1.add_grade("kanyinsola".to_string(), Some(88));

}

#[allow(dead_code)]
impl Students {
    fn new(name:String) -> Self {
        Students {
            name,
            grades: HashMap::new(),
        }
    }

    // #[allow(dead_code)]
    fn add_grade(&mut self, _course: String, _grade: Option<u8>) {
        self.grades.insert(_course, _grade);
    }
    fn display_information(&self){

    }
}

impl fmt::Display for Students{
    fn fmt (&self,f:&mut fmt::Formatter<'_>) ->fmt::Result{
        write!(f,"information:names{}\ngrades:{}",self.name,self.grades)
    }
}

// impl Students{
//     fn new() -> Self {
//         Students {
//             students: Vec::new(),
//         }
//     }

//     fn add_students(&mut self, students: Vec<Students>) {
//         self.student
//     }
// }

// impl fmt::Display for GradeBook{
//     fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
//         write!(f,"information,{}",self.person)
//     }
// }

// fn new_person(name: String, grade: u8) -> Result<HashMap<String, Vec<Option<u8>>>, io::Error> {
//     if name.is_empty() {
//         return Err(io::Error::new(io::ErrorKind::NotFound, "cannot be empty"));
//     }
//     let person_1_name = String::from(name);
//     let mut person_1_container = Vec::new();
//     let person_1_grade = Some(grade);
//     person_1_container.push(person_1_grade);
//     let mut person: HashMap<String, Vec<Option<u8>>> = HashMap::new();
//     person.insert(person_1_name, person_1_container);
//     Ok(person)
//     // let grade_1=GradeBook::new(person);
// }
