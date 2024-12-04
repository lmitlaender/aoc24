use crate::utils;
use std::collections::{BinaryHeap, HashMap};

pub fn run() {
    println!("Day 4 of Advent of Code!");

    let input = utils::read_file("inputs/day4.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    println!("Day 4 Part 1");

    // Make Vec of Strings to Vec<Vec<char>>
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let direction_change: Vec<(i64, i64)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];
    let wanted = vec!['X', 'M', 'A', 'S'];
    
    let mut count = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'X' {
                for (dx, dy) in &direction_change {
                    if i as i64 + 3*dx >= 0 && i as i64 + 3*dx < input.len() as i64 && j as i64 + 3*dy >= 0 && j as i64 + 3*dy < input[0].len() as i64 {
                        for step in 1..=3 {
                            if input[(i as i64 + step*dx) as usize][(j as i64 + step*dy) as usize] != wanted[step as usize] {
                                break;
                            }
                            if step == 3 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Count: {}", count);
}

fn part2(input: &Vec<String>) {
    println!("Day 4 Part 2");

    // Make Vec of Strings to Vec<Vec<char>>
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

    let direction_change: Vec<(i64, i64)> = vec![(1, 1), (-1, 1)];
    
    let mut count = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            if input[i][j] == 'A' {
                let mut correct = 0;
                for (dx, dy) in &direction_change {
                    if input[(i as i64 + dx) as usize][(j as i64 + dy) as usize] == 'S' && input[(i as i64 - dx) as usize][(j as i64 - dy) as usize] == 'M' {
                        correct += 1;
                    } else if input[(i as i64 + dx) as usize][(j as i64 + dy) as usize] == 'M' && input[(i as i64 - dx) as usize][(j as i64 - dy) as usize] == 'S' {
                        correct += 1;
                    } else {
                        break;
                    }
                }
                if correct == 2 {
                    count += 1;
                }
            }
        }
    }

    println!("Count: {}", count);
    
}