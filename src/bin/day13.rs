mod util;

// We might have to make this bigger, so
// start with a type alias
type Int = i64;

fn cost(solution: &(Int, Int)) -> Int {
    const a_cost: Int = 3;
    const b_cost: Int = 1;

    solution.0 * a_cost + solution.1 * b_cost
}

#[derive(Debug)]
struct Prize {
    a: (Int, Int),
    b: (Int, Int),
    prize: (Int, Int),
}
impl Prize {
    fn from_lines(lines: &[String]) -> Self {
        fn extract_xy(line: &str, prize_line: bool) -> (Int, Int) {
            let offset = if prize_line { 0 } else { 1 };
            let a_parts: Vec<_> = line.split_whitespace().collect();

            let a_x: Int = a_parts[offset + 1]
                .replace(&['X', '+', ',', '='], "")
                .parse()
                .expect("expected X value");
            let a_y: Int = a_parts[offset + 2]
                .replace(&['Y', '+', ',', '='], "")
                .parse()
                .expect("expected Y value");

            (a_x, a_y)
        }

        let a_line = &lines[0];
        let b_line = &lines[1];
        let prize_line = &lines[2];

        let a = extract_xy(a_line, false);
        let b = extract_xy(b_line, false);
        let prize = extract_xy(prize_line, true);

        Prize { a, b, prize }
    }
    fn solve_with_math(&self) -> Option<Int> {
        let maybe_solution = solve_eq(
            self.a.0,
            self.b.0,
            self.prize.0,
            self.a.1,
            self.b.1,
            self.prize.1,
        );

        if let Some(solution) = maybe_solution {
            Some(cost(&solution))
        } else {
            None
        }
    }
    fn solve(&self) -> Option<Int> {
        let mut solutions: Vec<(Int, Int)> = Vec::new();
        // testing only x here
        for i in 0..=100 {
            for j in 0..=100 {
                if self.a.0 * i + self.b.0 * j == self.prize.0
                    && self.a.1 * i + self.b.1 * j == self.prize.1
                {
                    solutions.push((i, j));
                }
            }
        }

        let min_val = solutions.iter().map(|x| cost(x)).min();
        min_val
    }
}

// Function to solve a system of 2 int linear equations
// Provided by ChatGPT
fn solve_eq(a1: Int, b1: Int, c1: Int, a2: Int, b2: Int, c2: Int) -> Option<(Int, Int)> {
    // Calculate the determinant (denominator)
    let denominator = a1 * b2 - a2 * b1;

    if denominator == 0 {
        None
    } else {
        // Calculate the numerators for x and y
        let x_numerator = c1 * b2 - c2 * b1;
        let y_numerator = a1 * c2 - a2 * c1;

        // Check if solutions are integers
        if x_numerator % denominator == 0 && y_numerator % denominator == 0 {
            let x = x_numerator / denominator;
            let y = y_numerator / denominator;

            Some((x, y))
        } else {
            None
        }
    }
}

fn main() {
    println!("AoC 2024: Day 13");

    // let lines = util::get_lines_from_file("input/day13-test.txt");
    // let lines = util::get_lines_from_file("input/day13-test2.txt");
    let lines = util::get_lines_from_file("input/day13.txt");

    let prizes: Vec<_> = lines
        .chunks(4)
        .map(|chunk| Prize::from_lines(chunk))
        .collect();
    let total: Int = prizes.iter().map(|x| x.solve_with_math()).flatten().sum();

    println!("Part 1, total cost: {}", total);

    // enlarge the prizes for part 2
    let big_prizes: Vec<_> = prizes
        .iter()
        .map(|p| Prize {
            a: p.a,
            b: p.b,
            prize: (p.prize.0 + 10000000000000, p.prize.1 + 10000000000000),
        })
        .collect();

    let big_total: Int = big_prizes
        .iter()
        .map(|x| x.solve_with_math())
        .flatten()
        .sum();

    println!("Part 2, total cost: {}", big_total);
}
