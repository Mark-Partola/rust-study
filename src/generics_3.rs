use std::fmt::Debug;

pub fn run_example() {
    accept(ImportantGuest);
    accept(Guest);
}

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

fn accept<T>(guest: T)
where
    T: Priority + Debug,
{
    println!(
        "accepting {:?} guest with {:?} priority",
        guest,
        guest.get_priority()
    );
}
