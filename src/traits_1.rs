trait Noise {
    // can have default implementation
    fn make_noise(&self) {
        println!("default noise");
    }
}

fn hello(noisy: impl Noise) {
    println!("before");
    noisy.make_noise();
    println!("after");
}

struct Person;

impl Noise for Person {
    fn make_noise(&self) {
        println!("hello");
    }
}

struct Dog;

impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof");
    }
}

struct Cat;
impl Noise for Cat {}

pub fn run_example() {
    hello(Person {});
    hello(Dog {});
    hello(Cat {})
}
