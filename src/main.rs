use std::{error::Error, fs::File, io::Read};
mod days;


fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    if args.len() < 2 {
        println!("Usage: {} <day> <input>", program);
        return Ok(());
    }

    let day_num: usize = args.next().unwrap().parse()?;
    let input_path = args.next().unwrap();

    let mut input = String::new();
    File::open(input_path)?.read_to_string(&mut input)?;

    if let Some(day) = days::DAYS.get(day_num - 1) {
        day(input);
    } else {
        println!("Day not implemented yet");
    };

    Ok(())
}
