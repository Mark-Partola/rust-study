trait Shape {
    fn calc(&self) -> u32;
    fn get_name(&self) -> &str;
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn calc(&self) -> u32 {
        self.side * 4
    }
    fn get_name(&self) -> &str {
        "Square"
    }
}

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Shape for Triangle {
    fn calc(&self) -> u32 {
        self.a + self.b + self.c
    }
    fn get_name(&self) -> &str {
        "Triangle"
    }
}

fn print_perimeter(shape: impl Shape) {
    println!("perimeter of {} is {}", shape.get_name(), shape.calc())
}

pub fn run_example() {
    print_perimeter(Square { side: 1 });
    print_perimeter(Triangle { a: 1, b: 2, c: 3 });
}
