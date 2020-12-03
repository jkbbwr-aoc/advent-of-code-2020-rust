use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Password {
    lower: i32,
    upper: i32,
    letter: char,
    password: String,
}

fn input() -> Vec<Password> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?P<lower>\d+)\-(?P<upper>\d+) (?P<letter>\w): (?P<rest>.*)"#).unwrap();
    }
    let text = read_to_string("input/day2.txt").expect("Can't read day2.txt");
    RE.captures_iter(&text).map(|captures| {
        Password {
            lower: captures.name("lower").unwrap().as_str().parse().unwrap(),
            upper: captures.name("upper").unwrap().as_str().parse().unwrap(),
            letter: captures.name("letter").unwrap().as_str().chars().next().unwrap(),
            password: captures.name("rest").unwrap().as_str().to_owned(),
        }
    }).collect()
}

fn part1() -> Option<i64> {
    let passwords = input();
    let result = passwords.iter()
        .filter(|&i| {
            let count = i.password.matches(i.letter).count() as i32;
            (i.lower..=i.upper).contains(&count)
        })
        .count();
    Some(result as i64)
}

fn part2() -> Option<i64> {
    let passwords = input();
    let result = passwords.iter()
        .filter(|&i| {
            let a = i.password.chars().nth((i.lower - 1) as usize).unwrap();
            let b = i.password.chars().nth((i.upper - 1) as usize).unwrap();
            (a == i.letter) ^ (b == i.letter)
        })
        .count();
    Some(result as i64)
}

pub fn solve() {
    println!("The answer to day 2 part 1 is: {}", part1().expect("Didn't find an answer."));
    println!("The answer to day 2 part 2 is: {}", part2().expect("Didn't find an answer."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1().unwrap(), 445);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 491);
    }
}