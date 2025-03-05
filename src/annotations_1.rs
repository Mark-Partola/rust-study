use std::fmt;

#[derive(Debug)]
enum Role {
    Admin,
}

impl fmt::Display for Role  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let title = match *self {
            Role::Admin => "admin",
        };

        f.write_str(title)
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    role: Role,
}

pub fn run_example() {
    let person = Person{
        name: String::from("John"),
        age: 20,
        role: Role::Admin
    };

    println!(
        "User {} has the role of {} [age: {}]",
        person.name,
        person.role,
        person.age,
    );
    println!("{:?}", person);
}