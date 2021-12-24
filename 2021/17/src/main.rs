use std::ops::RangeInclusive;

fn main() {
    let target = Target::new(288..=330, -96..=-50);

    println!("Answer Part 1: {}", find_highest_vy(&target));
    println!("Answer Part 2: {}", number_of_initials(&target));
}

fn number_of_initials(target: &Target) -> i32 {
    let mut initials = 0;
    for vx in -500..500 {
        for vy in -500..500 {
            if let Some(_) = simulate(target, vx, vy) {
                initials += 1;
            }
        }
    }
    initials
}

fn find_highest_vy(target: &Target) -> i32 {
    let mut highest = 0;
    for vx in 0..500 {
        for vy in 0..500 {
            if let Some(top_y) = simulate(target, vx, vy) {
                if top_y > highest {
                    highest = top_y;
                }
            }
        }
    }
    highest
}

fn simulate(target: &Target, vx: i32, vy: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;
    let mut highest = 0;

    loop {
        x = x + vx;
        y = y + vy;
        if vx > 0 {
            vx -= 1;
        }
        if vx < 0 {
            vx += 1;
        }
        vy -= 1;
        if y > highest {
            highest = y;
        }

        if target.is_hit(x, y) {
            return Some(highest);
        }
        if target.is_passed(x, y) {
            return None;
        }
    }
}

struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Target {
    fn new(x: RangeInclusive<i32>, y: RangeInclusive<i32>) -> Target {
        Target { x: x, y: y }
    }
    fn is_hit(&self, x: i32, y: i32) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
    fn is_passed(&self, x: i32, y: i32) -> bool {
        y < *self.y.start() || x > *self.x.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_part_1_like_examples() {
        let target = Target::new(20..=30, -10..=-5);
        assert_eq!(45, find_highest_vy(&target));
    }

    #[test]
    fn answer_part_2_like_examples() {
        let target = Target::new(20..=30, -10..=-5);
        assert_eq!(112, number_of_initials(&target));
    }

    #[test]
    fn simulation_like_examples() {
        let target = Target::new(20..=30, -10..=-5);
        assert!(simulate(&target, 7, 2).is_some());
        assert!(simulate(&target, 6, 3).is_some());
        assert!(simulate(&target, 9, 0).is_some());
        assert!(simulate(&target, 17, -4).is_none());
    }

    #[test]
    fn out_of_bounds() {
        let target = Target::new(20..=30, -10..=-5);
        assert!(!target.is_passed(0, 0), "0,0");
        assert!(!target.is_passed(25, -7), "25,-7");
        assert!(target.is_passed(50, -7), "50,-7");
        assert!(target.is_passed(25, -12), "25, -12");
    }
}
