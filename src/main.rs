mod annotations_1;
mod crates_1;
mod enums_1;
mod io_1;
mod iterators_1;
mod iterators_2;
mod match_1;
mod match_2;
mod modules;
mod modules_1;
mod option_1;
mod option_2;
mod result_1;

fn main() {
    annotations_1::run_example();
    enums_1::run_example();
    match_1::run_example();
    option_1::run_example();
    option_2::run_example();
    result_1::run_example();
    iterators_1::run_example();
    iterators_2::run_example();
    match_2::run_example();
    modules_1::run_example();
    crates_1::run_example();
    modules::auth::login();
    io_1::run_example(true);
}
