struct Student {
    name: String,
    grade: u32,
}

impl Student {
    fn new(name: &str, grade: u32) -> Self {
        Student {
            name: name.to_string(),
            grade,
        }
    }

    fn print_summary(&self) {
        println!("{} is in grade {}.", self.name, self.grade);
    }
}

fn main() {
    let student = Student::new("Amaka", 10);
    student.print_summary();
}