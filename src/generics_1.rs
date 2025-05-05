pub fn run_example() {
    let python = Snake {};
    move_to(python);
    let python = Snake {};
    generic_move_to(python);
    let python = Snake {};
    where_generic_move_to(python);
}

trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("Snake move to ({}, {})", x, y);
    }
}

fn move_to(thing: impl Move) {
    thing.move_to(0, 0);
}

fn generic_move_to<T: Move>(thing: T) {
    thing.move_to(10, 10);
}

fn where_generic_move_to<T>(thing: T)
where
    T: Move,
{
    thing.move_to(20, 20);
}
