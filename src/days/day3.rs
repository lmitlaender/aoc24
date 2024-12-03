use crate::utils;
use std::collections::{BinaryHeap, HashMap};
use regex::{Regex, RegexSet};
use regex_automata::{dfa::{Automaton, dense}, Input};


pub fn run() {
    println!("Day 3 of Advent of Code!");

    let input = utils::read_file("inputs/day3.txt");

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    println!("Day 3 Part 1");

    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();

    let mut result: i64 = 0;
    let mut l_index: usize = 0;
    let mut r_index: usize = 0;

    for line in input {
        let matches = re.find_iter(line);
        for m in matches {
            l_index = m.as_str().find("(").unwrap();
            r_index = m.as_str().find(")").unwrap();
            result += m.as_str()[l_index+1..r_index].split(",").map(|x| x.parse::<i64>().unwrap()).fold(1, |acc, x| acc * x);
        }
    }

    println!("Result day 3 part 1: {}", result);
}

// I leave this here cause it's a kinda cool way to play with the DFA created for regex matching directly, but the solution just uses a more special regex
/*fn can_become_match(current: &str, pattern: &str) -> bool {
    // Compile the pattern into a DFA
    let dfa = match dense::DFA::new(pattern) {
        Ok(dfa) => dfa,
        Err(_) => return false,
    };

    // Start at the DFA's initial state
    let mut state = match dfa.start_state_forward(&Input::new(current)) {
        Ok(state) => state,
        Err(_) => return false,
    };

    let mut prev_state = state;

    // Process the current string character by character
    for &b in current.as_bytes().iter() {
        prev_state = state;
        state = dfa.next_state(state, b);
        println!("State: {:?}, Prev State: {:?}, Char: {}", state, prev_state, b as char);
    }

    // DFAs in this crate require an explicit
    // end-of-input transition if a search reaches
    // the end of a haystack.
    // DFAs in this crate require an explicit
    // end-of-input transition if a search reaches
    // the end of a haystack.
    state = dfa.next_eoi_state(state);
    println!(
        "State: {:?}, Prev State: {:?}, Accel: {}, Dead: {}, Match: {}, Quit: {}, Special: {}, Start: {}",
        state,
        prev_state,
        dfa.is_accel_state(state),
        dfa.is_dead_state(state),
        dfa.is_match_state(state),
        dfa.is_quit_state(state),
        dfa.is_special_state(state),
        dfa.is_start_state(state)
    );
    !dfa.is_accel_state(state)
}*/

fn part2(input: &Vec<String>) {
    println!("Day 3 Part 2");

    let re = Regex::new(r"mul\((\d*),(\d*)\)|(do\(\))|(don't\(\))").unwrap();

    let mut result: u128 = 0;
    let mut calculate = true;
    
    for line in input {
        re.captures_iter(&line)
            .for_each(|cap| {
                if cap.get(1).is_some() && calculate{
                    result += cap.get(1).unwrap().as_str().parse::<u128>().unwrap() * cap.get(02).unwrap().as_str().parse::<u128>().unwrap();
                } else if cap.get(0).unwrap().as_str() == "do()" {
                    calculate = true;
                } else if cap.get(0).unwrap().as_str() == "don't()" {
                    calculate = false;
                }
            });
    }


    println!("Result day 3 part 2: {}", result);
}