use crate::utils;

pub fn run() {
    println!("Day 2 of Advent of Code!");

    let input = utils::read_file("inputs/day2.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut safe = 0;
    
    for i in 0..input.len() {
        let split_integers: Vec<i32> = input[i].split(" ").map(|x| x.trim().parse::<i32>().unwrap()).collect();

        if split_integers[1] > split_integers[0] {
            //println!("Ascend: {}", input[i]);
            if recurse_test_asc(&split_integers, 1) {
                safe += 1;
            }
        } else {
            //println!("Descend: {}", input[i]);
            if recurse_test_desc(&split_integers, 1) {
                safe += 1;
            }
        }
    }

    println!("Safe levels: {}", safe);
}

fn recurse_test_asc(input: &Vec<i32>, index: usize) -> bool {
    if index == input.len() {
        return true;
    }
    
    if input[index] > input[index - 1] && (input[index] - input[index - 1]).abs() <= 3 && (input[index] - input[index - 1]).abs() > 0 {
        return recurse_test_asc(input, index + 1);
    }

    return false;
}

fn recurse_test_desc(input: &Vec<i32>, index: usize) -> bool {
    if index == input.len() {
        return true;
    }

    if input[index] < input[index - 1] && (input[index] - input[index - 1]).abs() <= 3 && (input[index] - input[index - 1]).abs() > 0 {
        return recurse_test_desc(input, index + 1);
    }

    return false;
}

fn part2(input: &Vec<String>) {
    
}