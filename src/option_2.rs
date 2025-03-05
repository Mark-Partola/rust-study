struct Student {
    name: String,
    locker: Option<u8>,
}

pub fn run_example() {
    let mut lockers = Vec::from_iter(1..=3);
    lockers.reverse();

    let students = vec![
        Student{
            name: String::from("John"),
            locker: Some(lockers.pop().unwrap()),
        },
        Student{
            name: String::from("Jane"),
            locker: Some(lockers.pop().unwrap()),
        },
        Student{
            name: String::from("Harry"),
            locker: None
        },
        Student{
            name: String::from("Mary"),
            locker: None
        },
    ];

    for student in students {
        match student.locker {
            Some(it) => println!(
                "[{}]: locker {}",
                student.name,
                it,
            ),
            None => {
                match lockers.pop() {
                    Some(locker) =>
                        println!(
                            "[{}]: No assigned locker {}",
                            student.name,
                            locker,
                        ),
                    None => println!(
                        "[{}]: no lockers left",
                        student.name,
                    ),
                }
            },
        }
    }
}