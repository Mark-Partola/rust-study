struct Person {
    name: String,
    age: Option<u8>,
}

pub fn run_example() {
    let people = vec![
        Person{ name: String::from("John"), age: None },
        Person{ name: String::from("Jane"), age: Some(30) },
    ];

    for person in people {
        match person.age {
            Some(age) => println!("{}: {}", person.name, age),
            None => println!("{}", person.name),
        }
    }
}