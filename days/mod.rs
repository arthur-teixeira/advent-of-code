pub mod day1;
pub mod day2;

type Day = fn(input: String) -> ();
pub const DAYS: &[Day] = &[
    day1::day1,
    day2::day2,
];
