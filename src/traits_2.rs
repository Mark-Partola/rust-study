trait Racer {
    fn go(&self);
    fn is_ready(&self) -> bool;
    fn checkpoint(&self, position: i32);
}

struct Person;

impl Racer for Person {
    fn go(&self) {
        println!("go");
    }
    fn is_ready(&self) -> bool {
        true
    }
    fn checkpoint(&self, position: i32) {
        println!("checkpoint: {}", position);
    }
}

fn sprint(racer: impl Racer) {
    if racer.is_ready() {
        racer.go();
    }

    for step in 1..=3 {
        racer.checkpoint(step);
    }
}

pub fn run_example() {
    sprint(Person {});
}
