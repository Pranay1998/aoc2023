use std::cmp::max;

#[derive(Debug, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

impl Cubes {
    fn max(c1: Self, c2: Self) -> Self {
        return Self {
            red: max(c1.red, c2.red),
            green: max(c1.green, c2.green),
            blue: max(c1.blue, c2.blue)
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut valid_games: u32 = 0;
    for (idx, line) in input.lines().enumerate() {
        let (_, game) = line.split_once(": ").unwrap();
        let draws: Vec<&str> = game.split("; ").collect();

        let mut min_cubes = Cubes::default();

        for draw in draws {
            let cubes: Vec<&str> = draw.split(", ").collect();

            let mut cubes_count = Cubes::default();

            for cube in cubes {
                let (amount, color) = cube.split_once(" ").unwrap();
                let amount: u32 = amount.parse().unwrap();
                match color {
                    "red" => cubes_count.red += amount,
                    "green" => cubes_count.green += amount,
                    "blue" => cubes_count.blue += amount,
                    _ => panic!("Unexpected colour")
                }
            }
            
            min_cubes = Cubes::max(min_cubes, cubes_count);
        }

        if min_cubes.red <= 12 && min_cubes.green <= 13 && min_cubes.blue <= 14 {
            valid_games += (idx + 1) as u32;
        }
    }
    valid_games
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let (_, game) = line.split_once(": ").unwrap();
        let draws: Vec<&str> = game.split("; ").collect();

        let mut min_cubes = Cubes::default();

        for draw in draws {
            let cubes: Vec<&str> = draw.split(", ").collect();

            let mut cubes_count = Cubes::default();

            for cube in cubes {
                let (amount, color) = cube.split_once(" ").unwrap();
                let amount: u32 = amount.parse().unwrap();
                match color {
                    "red" => cubes_count.red += amount,
                    "green" => cubes_count.green += amount,
                    "blue" => cubes_count.blue += amount,
                    _ => panic!("Unexpected colour")
                }
            }
            
            min_cubes = Cubes::max(min_cubes, cubes_count);
        }

        total += min_cubes.red * min_cubes.green * min_cubes.blue;
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("inputs/2.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert!(part1(input) == 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert!(part2(input) == 2286);
    }
  
}