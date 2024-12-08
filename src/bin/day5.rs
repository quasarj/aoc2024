mod util;

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}
impl Rule {
    fn check(&self, update: &Update) -> bool {
        if !update.contains(&self.before) || !update.contains(&self.after) {
            true
        } else {
            let a_index = update.iter().position(|&x| x == self.before).unwrap();
            let b_index = update.iter().position(|&x| x == self.after).unwrap();

            a_index < b_index
        }
    }
    // "fix" the update for this rule only, by swapping the two elements
    fn fix(&self, update: &mut Update) -> bool {
        if !update.contains(&self.before) || !update.contains(&self.after) {
            false // no swap made
        } else {
            let a_index = update.iter().position(|&x| x == self.before).unwrap();
            let b_index = update.iter().position(|&x| x == self.after).unwrap();

            // println!("rule: {}->{}", self.before, self.after);
            // println!("before swap: {:?}", update);
            update.swap(a_index, b_index);
            // println!("after swap: {:?}", update);
            // dbg!(update);

            true // a swap was made
        }
    }
}

pub type Update = Vec<u32>;

fn main() {
    println!("AoC 2024: Day 5");

    // let lines = util::get_lines_from_file("input/day5-test.txt");
    let lines = util::get_lines_from_file("input/day5.txt");

    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Update> = Vec::new();

    let mut part1 = true;

    for line in lines {
        if line == "" {
            part1 = false;
            continue;
        }
        if part1 {
            rules.push(parse_rule(line));
        } else {
            updates.push(parse_update(line));
        }
    }

    let mut total = 0;
    let mut p2_total = 0;

    for update in &updates {
        // println!("{:?}", update);
        let pass = rules.iter().all(|x| x.check(&update));
        if pass {
            // find the middle number
            let middle_index = update.len() / 2;
            total += update[middle_index];
        } else {
            let mut fixable = update.clone();
            fix_all(&rules, &mut fixable);
            let middle_index = fixable.len() / 2;
            p2_total += fixable[middle_index];
        }
    }

    println!("Part 1: {total}");
    println!("Part 2: {p2_total}");

    // dbg!(rules);
    // dbg!(updates);
}
fn fix_all(rules: &Vec<Rule>, fixable: &mut Update) {
    loop {
        for rule in rules {
            if !rule.check(&fixable) {
                rule.fix(fixable);
            }
        }
        if all_rules_pass(&rules, &fixable) {
            // println!("breaking because all rules pass");
            // println!("{:?}", fixable);
            break;
        }
        // dbg!(fixable);
        // break;
    }
}
fn all_rules_pass(rules: &Vec<Rule>, update: &Update) -> bool {
    let pass = rules.iter().all(|x| x.check(&update));
    pass
}

fn parse_update(s: String) -> Update {
    let parts: Vec<&str> = s.split(',').collect();
    let num_parts: Update = parts
        .iter()
        .map(|x| x.parse::<u32>().expect("num"))
        .collect();

    num_parts
}

fn parse_rule(s: String) -> Rule {
    let parts: Vec<&str> = s.split('|').collect();
    let a: u32 = parts[0].parse().expect("should be a number");
    let b: u32 = parts[1].parse().expect("should be a number");

    Rule {
        before: a,
        after: b,
    }
}

#[test]
fn function_name_test() {
    let update: Update = vec![97, 13, 75, 29, 47];
    let rules: Vec<Rule> = vec![
        Rule {
            before: 97,
            after: 13,
        },
        Rule {
            before: 29,
            after: 13,
        },
    ];

    for rule in &rules {
        rule.fix(&mut update.clone());
    }
}
