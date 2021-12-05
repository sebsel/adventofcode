use std::fs;

const WORLD_SIZE: usize = 1000;

// Please be aware that I have no business writing Rust,
// but one has to start somewhere! And since I pushed
// the code to Github, it must've found the answer.

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut sea_map = [[0; WORLD_SIZE]; WORLD_SIZE];

    for line in contents.lines() {
        match parse(line) {
            Ok(((from_x, from_y), (to_x, to_y))) if from_x == to_x => {
                println!("Horizontal {},{} -> {},{}", from_x, from_y, to_x, to_y);
                let range = if from_y < to_y {
                    from_y..=to_y
                } else {
                    to_y..=from_y
                };
                for y in range {
                    sea_map[from_x][y] += 1;
                    println!("mark x {},{}", from_x, y);
                }
            }
            Ok(((from_x, from_y), (to_x, to_y))) if from_y == to_y => {
                println!("Vertical {},{} -> {},{}", from_x, from_y, to_x, to_y);
                let range = if from_x < to_x {
                    from_x..=to_x
                } else {
                    to_x..=from_x
                };
                for x in range {
                    sea_map[x][from_y] += 1;
                    println!("mark x {},{}", x, from_y);
                }
            }
            Ok(((from_x, from_y), (to_x, to_y))) => {
                println!("*Diagonal {},{} -> {},{}", from_x, from_y, to_x, to_y);
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
            print!(" {}", sea_map[x][y]);
        }
        print!("\n");
    }

    println!("Answer: {}", count);
}

fn parse(line: &str) -> Result<((usize, usize), (usize, usize)), String> {
    let mut it = line.chars().peekable();
    let mut number = String::from("");
    let mut i = 0;
    let mut result = [0, 0, 0, 0];

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

    return Ok(((result[0], result[1]), (result[2], result[3])));
}
