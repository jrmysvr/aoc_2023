pub mod day1;
pub mod day2;
pub mod day3;
pub mod input;

pub fn run_all() {
    let days = vec![
        day1::run,
        day2::run,
        day3::run,
    ];

    for day in days {
        day();
    }
}
