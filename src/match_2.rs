pub fn run_example() {
    let n = Some(1);

    match n {
        Some(n) => println!("{}", n),
        None => (),
    }

    if let Some(n) = n {
        println!("{}", n);
    }

    let mut flag = Some(3);
    while let Some(i) = flag {
        println!("in loop: {}", i);
        flag = None;
    }

    let mut numbers = 1..3;
    while let Some(n) = numbers.next() {
        println!("while let iter: {}", n);
    }
}
