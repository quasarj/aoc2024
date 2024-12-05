mod util;

fn main() {
    println!("AoC 2024: Day 2");
    // let lines = util::get_lines_from_file("input/day2-test.txt");
    let lines = util::get_lines_from_file("input/day2.txt");

    let mut num_safe = 0;
    let mut any_safe = 0;

    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // convert them to actual numbers
        let numbers: Vec<i32> = parts.iter()
            .map(|&s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers) {
            num_safe += 1;
        }

        // Logic for part 2, if we remove just one
        // element, are any of them safe?

        for index in 0..numbers.len() {
            // Explicitly combine slices before and after the removed element
            let remaining_slice = if !numbers.is_empty() {
                let (left, right) = numbers.split_at(index);
                let concat_slice = [left, &right[1..]].concat();
                concat_slice
            } else {
                vec![]
            };

            if is_safe(&remaining_slice) {
                any_safe += 1;
                break;
            }
        }
    }

    println!("Part 1, number safe: {num_safe}");
    println!("Part 2, number safe: {any_safe}");

}

fn is_safe(report: &Vec<i32>) -> bool {

    // loop over the elements and compare to eachother
    let res: Vec<_> = report.windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    // test that all differences are the same (negative or positive)
    let is_mixed = res.iter().any(|&x| x > 0) && 
                   res.iter().any(|&x| x < 0);

    if is_mixed {
        return false;
    }

    // make sure they are all within range
    let all_in_range = res.iter().all(|&x| (x > -4 && x < 0) || x < 4 && x > 0);

    all_in_range
}
