use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let fuel = calc_all_fuels("../input.txt")?;
    println!("Fuel required: {}", fuel);

    Ok(())
}

fn calc_all_fuels(path: &str) -> Result<i64, Box<dyn Error>> {
    let inputs = File::open(path)?;
    let mut buf_reader = BufReader::new(inputs);

    let mut fuel = 0;

    loop {
        let mut line = String::new();
        if 0 == buf_reader.read_line(&mut line)? {
            break;
        }
        let mass = line.trim().parse::<i64>()?;
        fuel += calc_fuel(mass);
    }

    Ok(fuel)
}

fn calc_fuel(mass: i64) -> i64 {
    (mass as f64 / 3f64).floor() as i64 - 2
}
