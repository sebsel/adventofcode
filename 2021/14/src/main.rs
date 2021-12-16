use std::collections::HashMap;
use std::fs;

type Rules = HashMap<[char; 2], char>;
type Polymer = HashMap<[char; 2], u64>;
type Counts = HashMap<char, u64>;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let (polymer_template, rules) = contents.split_once("\n\n").unwrap();

    let rules: Rules = rules
        .lines()
        .map(|line| {
            let a: char = line.chars().nth(0).unwrap();
            let b: char = line.chars().nth(1).unwrap();
            let insert: char = line.chars().nth(6).unwrap();
            ([a, b], insert)
        })
        .collect();

    let mut polymer = parse_template(polymer_template);

    for _day in 1..=40 {
        apply_rules(&mut polymer, &rules);
    }

    let mut counts: Counts = HashMap::new();
    for ([a, b], count) in polymer {
        increment_count(&mut counts, &a, count);
        increment_count(&mut counts, &b, count);
    }

    counts = counts.iter().map(|(el, count)| (*el, count / 2)).collect();

    let first = polymer_template.chars().nth(0).unwrap();
    let last = polymer_template.chars().last().unwrap();
    increment_count(&mut counts, &first, 1);
    increment_count(&mut counts, &last, 1);

    for (element, count) in counts.iter() {
        println!("{}: {}", element, count);
    }

    let min = counts.iter().map(|(_el, count)| count).min().unwrap();
    let max = counts.iter().map(|(_el, count)| count).max().unwrap();
    println!("Answer: {} - {} = {}", max, min, max - min);
}

fn parse_template(template: &str) -> Polymer {
    let mut result = HashMap::new();
    let mut peekable = template.chars().peekable();

    while let Some(&a) = peekable.peek() {
        peekable.next();

        match peekable.peek() {
            None => break,
            Some(&b) => increment_count(&mut result, &[a, b], 1),
        }
    }

    result
}

fn apply_rules(polymer: &mut Polymer, rules: &Rules) {
    let old_polymer: Polymer = polymer.clone();
    polymer.clear();

    for (pattern, count) in old_polymer {
        match rules.get(&pattern) {
            Some(&insert) => {
                let [a, b] = pattern;
                increment_count(polymer, &[a, insert], count);
                increment_count(polymer, &[insert, b], count);
            }
            None => break,
        }
    }
}

fn increment_count<T>(map: &mut HashMap<T, u64>, key: &T, count: u64)
where
    T: Eq,
    T: std::hash::Hash,
    T: Copy,
{
    match map.get_mut(key) {
        Some(cnt) => {
            *cnt = *cnt + count;
        }
        None => {
            map.insert(*key, count);
        }
    }
}

fn _print_polymer(polymer: &Polymer) {
    for (elem, count) in polymer.iter() {
        print!("{}{} {}, ", elem[0], elem[1], count);
    }
    print!("\n");
}
