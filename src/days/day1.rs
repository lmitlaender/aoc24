use crate::utils;
use std::collections::{BinaryHeap, HashMap};

pub fn run() {
    println!("Day 1 of Advent of Code!");

    let input = utils::read_file("inputs/day1.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    println!("Day 1 Part 1");

    let mut column1: BinaryHeap<i64> = BinaryHeap::new();
    let mut column2: BinaryHeap<i64> = BinaryHeap::new();

    input.iter().for_each(|line| {
        let split: Vec<&str> = line.split("   ").collect();
        column1.push(split[0].parse::<i64>().unwrap());
        column2.push(split[1].parse::<i64>().unwrap());
    });

    let column1 = column1.into_sorted_vec();
    let column2 = column2.into_sorted_vec();

    let mut summed_distance: i64 = 0;


    for i in 0..column1.len() {
        summed_distance += (column1[i] - column2[i]).abs();
    }

    println!("Summed distance: {}", summed_distance);
}

fn part2(input: &Vec<String>) {
    println!("Day 1 Part 2");

    let mut hash_map: HashMap<i64, i64> = HashMap::new();
    let mut column1: Vec<i64> = Vec::new();

    input.iter().for_each(|line| {
        let split: Vec<&str> = line.split("   ").collect();
        column1.push(split[0].parse::<i64>().unwrap());
        let y = split[1].parse::<i64>().unwrap();

        if hash_map.contains_key(&y) {
            let count = hash_map.get(&y).unwrap();
            hash_map.insert(y, count + 1);
        } else {
            hash_map.insert(y, 1);
        }
    });
    
    let mut similarity_score: i64 = 0;

    for i in 0..column1.len() {
        if hash_map.contains_key(&column1[i]) {
            similarity_score += hash_map.get(&column1[i]).unwrap() * column1[i];
        }
    }
    
    println!("Similarity score: {}", similarity_score);
}