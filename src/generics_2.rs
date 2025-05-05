trait CheckIn {
    fn check_in(&self);
    fn process(&self);

    fn handle(&self) {
        self.check_in();
        self.process();
    }
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("check_in as pilot");
    }
    fn process(&self) {
        println!("pilot enters the cockpit");
    }
}

struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("check_in as passenger");
    }
    fn process(&self) {
        println!("passenger takes a seat");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in");
    }
    fn process(&self) {
        println!("cargo moved to storage");
    }
}

enum Item {
    Pilot(Pilot),
    Passenger(Passenger),
    Cargo(Cargo),
}

pub fn run_example() {
    let items = vec![
        Item::Pilot(Pilot),
        Item::Passenger(Passenger),
        Item::Cargo(Cargo),
    ];

    for item in &items {
        match item {
            Item::Pilot(item) => item.handle(),
            Item::Passenger(item) => item.handle(),
            Item::Cargo(item) => item.handle(),
        }
    }
}
