fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
        })
        .map(|mut iter| {
            let first = iter.next().expect("Should contain atleast one number");
            let last = iter.last();

            match last {
                Some(last) => 10*first + last,
                None => 10*first + first
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("zero", "z0o")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");

            let mut iter = line
                .chars()
                .filter_map(|c| c.to_digit(10));

            
            let first = iter.next().expect("Should contain atleast one number");
            let last = iter.last();

            match last {
                Some(last) => 10*first + last,
                None => 10*first + first
            }
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/1.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert!(part1(input) == 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert!(part2(input) == 281);
    }
  
}