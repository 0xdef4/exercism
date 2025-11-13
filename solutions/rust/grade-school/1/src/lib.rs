use std::collections::HashMap;

pub struct School {
    school: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            school: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let mut students = Vec::new();

        // check if student exists in multiple grades
        let is_multiple_grade: bool = self
            .school
            .values()
            .flat_map(|v| v.iter())
            .collect::<Vec<_>>()
            .contains(&&student.to_string());
        
        match self.school.get_mut(&grade) {
            Some(students) => {
                if !is_multiple_grade {
                    students.push(student.to_string());
                    students.sort();
                }
            }
            None => {
                if !is_multiple_grade {
                    students.push(student.to_string());
                    students.sort();
                    self.school.insert(grade, students);
                }
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.school.keys().cloned().collect::<Vec<_>>();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.school
            .get(&grade)
            .cloned()
            .unwrap_or(Vec::<String>::new())
    }
}
