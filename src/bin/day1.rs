mod util;

fn main() {
    println!("AoC 2024: Day 1");
    // let lines = util::get_lines_from_file("input/day1-test.txt");
    let lines = util::get_lines_from_file("input/day1.txt");

    // but we actually want them in two lists
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // convert them to actual numbers
        let numbers: Vec<i32> = parts.iter().map(|&s| s.parse::<i32>().unwrap()).collect();

        l1.push(numbers[0]);
        l2.push(numbers[1]);
    }

    l1.sort();
    l2.sort();

    // Part 1
    let mut total = 0;

    for (a, b) in l1.iter().zip(l2.iter()) {
        total += (b - a).abs();
    }

    println!("Part 1: {total}");

    // Part 2
    let l2_freq = util::count_frequencies(&l2);
    let mut similarity_score: i32 = 0;

    for i in &l1 {
        let freq = l2_freq.get(&i).unwrap_or(&0);
        let similarity: i32 = i * *freq as i32;
        // println!("{i}, {freq}, {}", similarity);
        similarity_score += similarity;
    }

    println!("Part 2 (Total Similarity Score): {similarity_score}");
}
