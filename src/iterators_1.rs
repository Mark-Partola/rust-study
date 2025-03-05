pub fn run_example() {
    let numbers = vec![1, 2, 3, 4, 5];

    let numbers = numbers.iter().map(|it| it * it).collect::<Vec<_>>();

    println!("{:?}", numbers);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let people = vec![
        Person {
            name: String::from("Jane"),
            age: 14,
        },
        Person {
            name: String::from("John"),
            age: 40,
        },
        Person {
            name: String::from("Robert"),
            age: 31,
        },
    ];

    let people = people.iter().filter(|it| it.age >= 18).collect::<Vec<_>>();

    people
        .iter()
        .for_each(|it| println!("{} {}", it.name, it.age));

    let john = people.iter().find(|it| it.name == "John");

    println!("John's age is {}", john.unwrap().age);
}
