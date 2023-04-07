#[derive(Debug)]
struct Student {
    stud_id: u32,
    first_name: String,
    last_name: String,
    email: String,
    contact_no: String,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "\nID     : {:05}\nName   : {} {}\nEmail  : {}\nContact: {}",
            self.stud_id, self.first_name, self.last_name, self.email, self.contact_no
        )
    }
}

fn sort_students_by_name(students: &mut [Student]) {
    students.sort_by(|a, b| a.first_name.cmp(&b.first_name));
}

fn sort_students_by_id(students: &mut [Student]) {
    students.sort_by(|a, b| a.stud_id.cmp(&b.stud_id));
}

fn main() {
    let mut students = vec![
        Student {
            stud_id: 102,
            first_name: String::from("Zack"),
            last_name: String::from("Lee"),
            email: String::from("lee.bobby@example.com"),
            contact_no: String::from("(420) 1234-4455"),
        },Student {
            stud_id: 100,
            first_name: String::from("Bob"),
            last_name: String::from("Smith"),
            email: String::from("bob.smith@example.com"),
            contact_no: String::from("(555) 1234-3250"),
        },
        Student {
            stud_id: 101,
            first_name: String::from("Amit"),
            last_name: String::from("Brown"),
            email: String::from("amit_brown@example.com"),
            contact_no: String::from("(651) 555-4433"),
        },
    ];
    
    sort_students_by_name(&mut students);
    println!("*** Sorted by Name: ***");
    for student in &students {
        println!("{}", student);
    }
    println!();
    
    sort_students_by_id(&mut students);
    println!("*** Sorted by ID: ***");
    for student in &students {
        println!("{}", student);
    }
}
