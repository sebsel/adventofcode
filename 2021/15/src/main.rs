use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

// I have to admit I looked up a lot of Dijkstra and even then I had
// a lot of trouble mapping it all to Rust, but this challenge is
// about learning stuff so we should be able to use resources.

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let map = Map::from_string(&contents[..]);
    let mut walker = Walker::new(&map);
    let (path, risk) = walker.shortest_path((0, 0), (map.cols - 1, map.rows - 1));
    map.draw(path);
    println!("Risk: {}", risk);
}

struct Walker<'a> {
    map: &'a Map<'a>,
    queue: BinaryHeap<Node>,
    visited: HashMap<Point, Point>,
}

impl Walker<'_> {
    fn new<'a>(map: &'a Map) -> Walker<'a> {
        Walker {
            map: map,
            visited: HashMap::new(),
            queue: BinaryHeap::new(),
        }
    }

    fn shortest_path(&mut self, start: Point, end: Point) -> (Vec<Point>, u32) {
        self.queue.push(Node {
            point: start,
            risk: 0,
        });

        while let Some(node) = self.queue.pop() {
            if node.point == end {
                break;
            }
            for neighbor in self.map.neighbors(node.point) {
                if !self.visited.contains_key(&neighbor) {
                    self.visited.insert(neighbor, node.point);
                    self.queue.push(Node {
                        point: neighbor,
                        risk: node.risk + self.map.get_risk(neighbor),
                    });
                }
            }
        }

        let mut path: Vec<Point> = vec![];
        let mut risk = 0;
        let mut point = end;
        loop {
            path.push(point);
            if point == start {
                break;
            }
            risk += self.map.get_risk(point);
            match self.visited.get(&point) {
                Some(pnt) => point = *pnt,
                None => break,
            }
        }
        (path, risk)
    }
}

type Point = (usize, usize);

#[derive(Eq)]
struct Node {
    risk: u32,
    point: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

struct Map<'a> {
    data: &'a str,
    rows: usize,
    cols: usize,
}

impl Map<'_> {
    fn from_string<'a>(string: &'a str) -> Map<'a> {
        Map {
            data: string,
            rows: string.lines().count(),
            cols: string.lines().nth(0).unwrap().chars().count(),
        }
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let (col, row) = point;
        let mut vec = vec![];
        if col > 0 {
            vec.push((col - 1, row));
        }
        if row > 0 {
            vec.push((col, row - 1));
        }
        if col + 1 < self.cols {
            vec.push((col + 1, row));
        }
        if row + 1 < self.rows {
            vec.push((col, row + 1));
        }
        vec
    }

    fn get_risk(&self, point: Point) -> u32 {
        let pos = self.point_to_pos(point);
        self.data[pos..pos + 1].parse().unwrap()
    }

    fn point_to_pos(&self, point: Point) -> usize {
        let (col, row) = point;
        assert!(col < self.cols);
        assert!(row < self.rows);
        (row * self.cols) + col + (row + 1 / self.rows)
    }

    fn draw(&self, path: Vec<Point>) {
        let path: HashSet<usize> = path.iter().map(|point| self.point_to_pos(*point)).collect();

        for (addr, c) in self.data.chars().enumerate() {
            if path.contains(&addr) {
                print!(" \u{001b}[1m\u{001b}[34m{}\u{001b}[0m", c);
            } else {
                print!(" {}", c);
            }
        }
    }
}
