pub mod arrays {
    pub fn chunk(arr: &Vec<i32>, size: usize) -> Vec<Vec<i32>> {
        let count = usize::div_ceil(arr.len(), size);
        let mut g = arr.iter();
        let mut result: Vec<Vec<i32>> = vec![];

        for _ in 0..count {
            let mut ch: Vec<i32> = vec![];
            for _ in 0..size {
                if let Some(v) = g.next() {
                    ch.push(v.clone());
                }
            }

            result.push(ch);
        }

        result
    }
}

mod maths {
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

mod msg {
    pub fn hello(name: &str) -> String {
        format!("Hello, {}", exiting(name))
    }
    pub fn bye(name: &str) -> String {
        format!("bye, {}", exiting(name))
    }
    fn exiting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

pub fn run_example() {
    use arrays::*;
    use msg::{bye, hello};

    let chunks = chunk(&vec![1, 2, 3, 4, 5], 2);
    println!("chunks: {:?}", chunks);

    println!("1 + 2 = {}; 1 - 2 = {}", maths::sum(1, 2), maths::sub(1, 2));

    println!("{}", hello("John"));
    println!("{}", bye("John"));
}
