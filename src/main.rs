use std::io;

mod days;
mod utils;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut input = String::new();
    println!("Enter the day of Advent of Code:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let day: u32 = input.trim().parse().expect("Please enter a valid number");

    match day {
        1 => days::day1::run(),
        2 => days::day2::run(),
        3 => days::day3::run(),
        _ => println!("Day {} is not yet reached or implemented", day),
    }
}