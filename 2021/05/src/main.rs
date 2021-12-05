use std::fs;

const WORLD_SIZE: usize = 1000;

// This is a cleaned up version, see part1.rs and part2.rs
// for the versions that I used to find the actual answers
// to the problem of today. (But this one works too!)

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");

    let mut sea_map = [[0; WORLD_SIZE]; WORLD_SIZE];

    for line in contents.lines() {
        match parse(line) {
            Ok(line) => {
                for (x, y) in line.iter() {
                    sea_map[x][y] += 1;
                }
            }
            _ => println!("Not good!"),
        }
    }

    let mut count = 0;
    for y in 0..WORLD_SIZE {
        for x in 0..WORLD_SIZE {
            if sea_map[x][y] >= 2 {
                count += 1;
            }
        }
    }

    println!("Answer: {}", count);
}

fn parse(line: &str) -> Result<Line, String> {
    let mut it = line.chars().peekable();
    let mut number = String::from("");
    let mut i = 0;
    let mut result: [usize; 4] = [0, 0, 0, 0];

    while let Some(&c) = it.peek() {
        match c {
            ' ' | '>' => {
                it.next();
            }
            '0'..='9' => {
                number.push(c);
                it.next();
            }
            ',' | '-' => {
                result[i] = number.parse().unwrap();
                i += 1;
                number.truncate(0);
                it.next();
            }
            _ => return Err(format!("Unexpected {}", c)),
        }
    }
    result[i] = number.parse().unwrap();

    return Ok(Line {
        from: (result[0], result[1]),
        to: (result[2], result[3]),
    });
}

struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

impl Line {
    fn iter(self) -> LineIter {
        LineIter {
            from: self.from,
            to: self.to,
            stop: false,
        }
    }
}

struct LineIter {
    from: (usize, usize),
    to: (usize, usize),
    stop: bool,
}

impl Iterator for LineIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.stop {
            return None;
        }

        let result = self.from;

        if self.from == self.to {
            self.stop = true;
            return Some(result);
        }

        match (self.from.0, self.to.0) {
            (from, to) if from < to => self.from.0 += 1,
            (from, to) if from > to => self.from.0 -= 1,
            (_from, _to) => (),
        };

        match (self.from.1, self.to.1) {
            (from, to) if from < to => self.from.1 += 1,
            (from, to) if from > to => self.from.1 -= 1,
            (_from, _to) => (),
        };

        return Some(result);
    }
}
