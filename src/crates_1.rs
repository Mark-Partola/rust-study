use humantime::format_duration;
use std::time::Duration;

pub fn run_example() {
    let duration = Duration::from_secs(9876);
    let formatted = format_duration(duration);
    println!("{}", formatted);
}
