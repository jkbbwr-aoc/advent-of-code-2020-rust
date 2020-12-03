use std::fs::read_to_string;

fn input() -> Vec<i64> {
    read_to_string("input/day1.txt").expect("Can't read day1")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

fn part1() -> Option<i64> {
    let mut values = input();
    values.sort();
    for x in &values {
        let y = 2020 - *x;
        if values.contains(&y) {
            return Some(*x * y);
        }
    }
    None
}

fn part2() -> Option<i64> {
    let mut values = input();
    values.sort();
    for x in &values {
        for y in &values {
            let z = 2020 - *x - *y;
            if values.contains(&z) {
                return Some(*x * *y * z);
            }
        }
    }
    None
}

pub fn solve() {
    println!("The answer to day 1 part 1 is: {}", part1().expect("Didn't find an answer."));
    println!("The answer to day 1 part 2 is: {}", part2().expect("Didn't find an answer."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1().unwrap(), 545379);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 257778836);
    }
}