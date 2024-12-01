use std::io;

mod days;
mod utils;

fn main() {
    let mut input = String::new();
    println!("Enter the day of Advent of Code:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let day: u32 = input.trim().parse().expect("Please enter a valid number");

    match day {
        1 => days::day1::run(),
        _ => println!("Day {} is not yet reached or implemented", day),
    }
}