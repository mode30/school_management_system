// use core::fmt;
use std::{collections::HashMap, io};
//Gradebook mapping to name and grade
#[derive(Debug)]
#[allow(dead_code)]
struct GradeBook {
    person: HashMap<String, Vec<Option<u8>>>,
}

fn main() {
    let person_1_name = String::from("benjamin");
    let mut person_1_container = Vec::new();
    let person_1_grade = Some(8);
    person_1_container.push(person_1_grade);
    let mut person: HashMap<String, Vec<Option<u8>>> = HashMap::new();
    person.insert(person_1_name, person_1_container);
    let grade_1 = GradeBook::new(person);

    println!("person:{:?}", grade_1);
    // println!("person:{}",grade_1);

    grade_1.add_grade("benjamin", Some(22));
}

#[allow(dead_code)]
impl GradeBook {
    fn new(person: HashMap<String, Vec<Option<u8>>>) -> Self {
        GradeBook { person }
    }

    #[allow(dead_code)]
    fn add_grade(&self, _name: &str, _grade: Option<u8>) {
        for names in &self.person.keys() {
            println!("names")

            // println!("{:?}",names)
        }
    }
}

// impl fmt::Display for GradeBook{
//     fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
//         write!(f,"information,{}",self.person)
//     }
// }

fn new_person(name: String, grade: u8) -> Result<HashMap<String, Vec<Option<u8>>>, io::Error> {
    if name.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "cannot be empty"));
    }
    let person_1_name = String::from(name);
    let mut person_1_container = Vec::new();
    let person_1_grade = Some(grade);
    person_1_container.push(person_1_grade);
    let mut person: HashMap<String, Vec<Option<u8>>> = HashMap::new();
    person.insert(person_1_name, person_1_container);
    Ok(person)
    // let grade_1=GradeBook::new(person);
}
