#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn get_lines_from_file(filename: &str) -> Vec<String> {
    let input_file = read_to_string(filename).unwrap();

    let lines = input_file.lines().map(|line| line.to_string()).collect();

    lines
}

pub fn count_frequencies(numbers: &[i32]) -> HashMap<i32, usize> {
    let mut frequency_map = HashMap::new();

    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    frequency_map
}
