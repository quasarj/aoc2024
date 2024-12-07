mod util;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn generate_combinations(length: usize, current: Vec<Operator>, results: &mut Vec<Vec<Operator>>) {
    if current.len() == length {
        results.push(current);
        return;
    }

    // Add `Operator::Multiply` and recurse
    let mut multiply_current = current.clone();
    multiply_current.push(Operator::Multiply);
    generate_combinations(length, multiply_current, results);

    // Add `Operator::Add` and recurse
    let mut add_current = current.clone();
    add_current.push(Operator::Add);
    generate_combinations(length, add_current, results);

    // Add `Operator::Concat` and recurse
    let mut concat_current = current.clone();
    concat_current.push(Operator::Concat);
    generate_combinations(length, concat_current, results);
}


#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>
}

impl Equation {
    fn solvable(&self) -> bool {
        let num_ops = self.operands.len() - 1;
        // println!("solve {} with {}", self.test_value, num_ops);

        // get possible combinations of operators for this len
        let mut results = Vec::new();
        generate_combinations(num_ops, Vec::new(), &mut results);


        for op_combination in results {
            // println!("op comb ---{:?}", op_combination);
            let mut pos = 0;
            let mut final_val = self.operands[pos];
            pos += 1;

            for op in op_combination {
                match op {
                    Operator::Add => final_val += self.operands[pos],
                    Operator::Multiply => final_val *= self.operands[pos],
                    Operator::Concat => {
                        let n1 = final_val.to_string();
                        let n2 = self.operands[pos].to_string();
                        let concatenated = format!("{}{}", n1, n2);
                        let result: u64 = concatenated
                            .parse()
                            .expect("Failed to parse concatenated string as integer");
                        final_val = result;
                    }
                }
                pos += 1;
            }

            // dbg!(final_val);
            if final_val == self.test_value {
                return true;
            }
        }

        false
    }
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


    let mut solvable_count = 0;
    let mut calibration_total = 0;
    for eq in equations {
        if eq.solvable() {
            solvable_count += 1;
            calibration_total += eq.test_value;
        }
    }

    println!("Part 1, solvable count: {}", solvable_count);
    println!("Part 1, total calibration result: {}", calibration_total);


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

    Some(
        Equation {
            test_value,
            operands: operand_parts
        }
    )
}
