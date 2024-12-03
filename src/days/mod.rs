pub mod day1;
pub mod day2;
pub mod day3;

type Day = fn(input: String) -> ();
pub const DAYS: &[Day] = &[
    day1::day1,
    day2::day2,
    day3::day3,
];
