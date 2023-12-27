use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {

    fn up(&self) -> Coord {
        Coord { row: self.row - 1, col: self.col }
    }

    fn down(&self) -> Coord {
        Coord { row: self.row + 1, col: self.col }
    }

    fn left(&self) -> Coord {
        Coord { row: self.row, col: self.col - 1 }
    }

    fn right(&self) -> Coord {
        Coord { row: self.row, col: self.col + 1 }
    }

    fn valid_coord(&self, grid: &Vec<Vec<char>>) -> bool {
        if self.row >= grid.len() || self.col >= grid[0].len() {
            return false;
        }
        return true;
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/5.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

fn valid_conection(start: char, end: char, direction: Direction) -> bool {
    let valid_tiles = ['+', '|', '-', 'L', 'J', '7', 'F', '.'];

    if !valid_tiles.contains(&start) || !valid_tiles.contains(&end) {
        panic!("Invalid start or end, start: {}, end: {}", start, end);
    }

    match direction {
        Direction::Up => {
            if start == '+' || start == '|' || start == 'L' || start == 'J' {
                if end == '+' || end == '|' || end == '7' || end == 'F' {
                    return true;
                }
            }
            return false;
        },
        Direction::Down => {
            if start == '+' || start == '|' || start == '7' || start == 'F' {
                if end == '+' || end == '|' || end == 'L' || end == 'J' {
                    return true;
                }
            }
            return false;
        },
        Direction::Left => {
            if start == '+' || start == '-' || start == 'J' || start == '7' {
                if end == '+' || end == '-' || end == 'L' || end == 'F' {
                    return true;
                }
            }
            return false;
        },
        Direction::Right => {
            if start == '+' || start == '-' || start == 'L' || start == 'F' {
                if end == '+' || end == '-' || end == 'J' || end == '7' {
                    return true;
                }
            }
            return false;
        },
    }
}

fn part1(input: &str) -> u32 {
    let binding = input.replace("S", "+");
    let grid: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();
    
    let mut start: Option<Coord> = None;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '+' {
                start = match start {
                    Some(_) => panic!("Multiple starts found"),
                    None => Some(Coord { row, col }),
                }
            }
        }
    }

    let mut max_cost = 0;
    let start = start.unwrap();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut queue: VecDeque<(Coord, u32)> = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (coord, cost) = queue.pop_front().unwrap();
        visited.insert(coord);
        
        let moves = [
            (coord.up(), Direction::Up),
            (coord.down(), Direction::Down),
            (coord.left(), Direction::Left),
            (coord.right(), Direction::Right),
        ];

        for (c, m) in moves {
            if c.valid_coord(&grid) && !visited.contains(&c) {
                if valid_conection(grid[coord.row][coord.col], grid[c.row][c.col], m) {
                    max_cost = std::cmp::max(max_cost, cost + 1);
                    queue.push_back((c, cost + 1));
                }
            }
        }
    }
    max_cost
}

fn part2(input: &str) -> u32 {
    // look into shoelace & picks theorem
    let _input: &str = input.replace("S", "+").as_str();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
assert!(part1(input) == 4);
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