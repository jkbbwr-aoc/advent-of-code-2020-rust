// flat grids with help from kirby

fn input() -> (i64, Vec<bool>) {
    let input = include_str!("../input/day3.txt");
    let width = input.chars().take_while(|&c| c != '\n').count() as i64;
    let map = input.chars().filter(|&c| c != '\n').map(|c| c == '#').collect();
    (width, map)
}

fn count_trees(width: i64, right: i64, down: i64, lines: &[bool]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;

    loop {
        match lines.get((y * width + x) as usize) {
            Some(true) => count += 1,
            Some(false) => {}
            None => return count,
        }
        x += right;
        if x >= width {
            x -= width;
        }
        y += down;
    }
}

fn part1() -> i64 {
    let (width, lines) = input();
    count_trees(width, 3, 1, &lines)
}

fn part2() -> i64 {
    let (width, lines) = input();
    let slopes = [(1i64, 1i64), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter()
        .map(|(right, down)| {
            count_trees(width, *right, *down, &lines)
        })
        .product()
}

pub fn solve() {
    println!("The answer to day 3 part 1 is: {}", part1());
    println!("The answer to day 3 part 2 is: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 171);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1206576000);
    }
}