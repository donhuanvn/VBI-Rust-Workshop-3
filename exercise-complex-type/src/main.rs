use std::collections::HashMap;
use itertools::Itertools;

struct School {
    students: HashMap<String, u32>
}

impl School {
    pub fn new() -> School {
        Self { students: HashMap::new() }
    }
    pub fn add(&mut self, student: &str, grade: u32) {
        self.students.entry(String::from(student)).or_insert(grade);
    }
    pub fn print_students(&self) {
        let count = self.students.len();
        println!("There is {count} students.");
        for (student, grade) in &self.students {
            println!("{student}: '{grade}'");
        }
    }
    pub fn grades(&self) -> Vec<u32> {
        let scores: Vec<u32> = self.students.values().cloned().collect();
        let scores: Vec<u32> = scores.into_iter().unique().sorted().collect();
        scores
    }
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut temp = self.students.clone();
        temp.retain(|_, v| *v == grade);
        let temp = temp.into_keys().sorted().collect_vec();
        temp
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    let mut school = School::new();
    // Testcase 1: Khoi tao danh sach rong
    println!("\n\nTestcase 1: Creating an empty school (vector)");
    school.print_students();

    println!("\n\nTestcase 2: Adding new students into the school");
    school.add("Lee", 2);
    school.add("Nancy", 3);
    school.print_students();
    
    println!("\n\nTestcase 3: Filtering students with a specified grade");
    school.add("Bob", 4);
    school.add("Alice", 4);
    school.add("Tom", 5);
    let students_with_grade_4 = school.grade(4);
    println!("Students with grade of 4: {:?}", students_with_grade_4);
}
