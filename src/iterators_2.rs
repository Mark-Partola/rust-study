pub fn run_example() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers = numbers
        .iter()
        .map(|it| it * 3)
        .filter(|it| it > &10)
        .collect::<Vec<_>>();

    for n in &numbers {
        println!("{}", n);
    }
}
