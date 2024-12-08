mod util;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}
impl Operator {
    // There is unfortunately no way to iterate over all
    // elements in an enum, so just hardcode it here
    fn all(part2: bool) -> Vec<Operator> {
        if part2 {
            vec![Operator::Add, Operator::Multiply, Operator::Concat]
        } else {
            vec![Operator::Add, Operator::Multiply]
        }
    }
    fn permutations(count: usize, part2: bool) -> Vec<Vec<Operator>> {
        itertools::repeat_n(Operator::all(part2), count)
            .multi_cartesian_product()
            .collect()
    }
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn solvable(&self, part2: bool) -> bool {
        let num_ops = self.operands.len() - 1;
        let op_combinations = Operator::permutations(num_ops, part2);

        for op_combination in op_combinations {
            let mut pos = 0;
            let mut final_val = self.operands[pos];
            pos += 1;

            for op in op_combination {
                match op {
                    Operator::Add => final_val += self.operands[pos],
                    Operator::Multiply => final_val *= self.operands[pos],
                    Operator::Concat => final_val = concat(final_val, self.operands[pos]),
                }
                pos += 1;
            }

            if final_val == self.test_value {
                return true;
            }
        }

        false
    }
}

fn concat(a: u64, b: u64) -> u64 {
    // how many digits is b?
    let digits = b.ilog10() + 1;

    // shift a left by that many
    let a_prime = a * (u64::pow(10, digits));

    a_prime + b
}

fn parse_line(line: &String) -> Option<Equation> {
    let parts: Vec<_> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let test_value: u64 = parts[0].parse().expect("test value was not a number!");

    let operand_parts: Vec<_> = parts[1]
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("input was not a number!"))
        .collect();

    Some(Equation {
        test_value,
        operands: operand_parts,
    })
}

fn main() {
    println!("AoC 2024: Day 7");

    // let lines = util::get_lines_from_file("input/day7-test.txt");
    let lines = util::get_lines_from_file("input/day7.txt");

    let mut equations: Vec<Equation> = Vec::new();

    for line in &lines {
        if let Some(e) = parse_line(line) {
            equations.push(e);
        }
    }

    fn solve(equations: &Vec<Equation>, part2: bool) {
        let mut solvable_count = 0;
        let mut calibration_total = 0;
        for eq in equations {
            if eq.solvable(part2) {
                solvable_count += 1;
                calibration_total += eq.test_value;
            }
        }

        let part = if part2 { 2 } else { 1 };
        println!("Part {part}, solvable count: {}", solvable_count);
        println!(
            "Part {part}, total calibration result: {}",
            calibration_total
        );
    }

    solve(&equations, false); // part 1
    solve(&equations, true); // part 2
}
