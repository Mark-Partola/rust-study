struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: String, age: u8) -> Result<Self, String> {
        if age < 21 {
            return Err(String::from("must be above 21."));
        }

        Ok(Self { name, age })
    }
}

pub fn run_example() {
    let young = Adult::new(String::from("John"), 19);

    match young {
        Ok(person) => println!("Ok: {}", person.name),
        Err(e) => println!("Err: {}", e),
    }

    let adult = Adult::new(String::from("Robert"), 25);

    match adult {
        Ok(person) => println!("Ok: {} {}", person.name, person.age),
        Err(e) => println!("Err: {}", e),
    }
}
