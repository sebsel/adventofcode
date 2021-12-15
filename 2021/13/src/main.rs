use std::fmt;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");

    let (first, second) = contents.split_once("\n\n").unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    for line in second.lines() {
        let line = line.strip_prefix("fold along ").unwrap();
        let (direction, at_line) = line.split_once("=").unwrap();

        instructions.push(match direction {
            "x" => Instruction::FoldX(at_line.parse().unwrap()),
            "y" => Instruction::FoldY(at_line.parse().unwrap()),
            &_ => panic!("Unknown direction!"),
        });

        // I accidentally solved part 2 first, without knowing.
        // To get the answer to part 1, uncomment the next
        // line and see line 88 for more manual hacks.

        // break;
    }

    let mut map = map_from_instructions(&instructions);

    for line in first.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let mut x: usize = x.parse().unwrap();
        let mut y: usize = y.parse().unwrap();

        for instr in &instructions {
            match instr {
                Instruction::FoldX(i) => {
                    let i = *i as usize;
                    if x >= i {
                        x = i - (x - i);
                    }
                }
                Instruction::FoldY(i) => {
                    let i = *i as usize;
                    if y >= i {
                        y = i - (y - i);
                    }
                }
            }
        }
        map[x][y] = true;
    }

    let mut count = 0;
    for row in map {
        for col in row {
            if col {
                count += 1;
                print!(" #");
            } else {
                print!(" .");
            }
        }
        print!("\n");
    }

    println!("Answer: {}", count);
}

fn map_from_instructions(instructions: &Vec<Instruction>) -> Vec<Vec<bool>> {
    let mut x = -1;
    let mut y = -1;
    for instr in instructions {
        match instr {
            Instruction::FoldX(i) => {
                if x < *i || x != -1 {
                    x = *i
                }
            }
            Instruction::FoldY(i) => {
                if y < *i || y != -1 {
                    y = *i
                }
            }
        }
    }

    // Here I had to set my vectors up to a correct size, big enough
    // for the full half-folded map. I didn't want more parsing in
    // code so I just looked up the first folds: x=655, y=447

    // x = 655 * 2;
    // y = 447 * 2;

    let mut map = vec![];
    for i in 0..x {
        map.push(vec![]);
        for _ in 0..y {
            map[i as usize].push(false);
        }
    }
    map
}

#[derive(Debug)]
enum Instruction {
    FoldX(i32),
    FoldY(i32),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::FoldX(i) => write!(f, "Fold X: {}", i),
            Instruction::FoldY(i) => write!(f, "Fold Y: {}", i),
        }
    }
}
