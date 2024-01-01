use std::collections::{HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}



fn tilt(grid: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::North => {
            for j in 0..grid[0].len() {
                for i in 0..grid.len() {
                    if grid[i][j] == 'O' && i > 0 {
                        for k in (0..=i-1).rev() {
                            if grid[k][j] == '.' {
                                grid[k][j] = 'O';
                                grid[k+1][j] = '.';
                            } else if grid[k][j] == '#' || grid[k][j] == 'O' {
                                break;
                            }
                        }
                    }
                    
                }
            }
        },
        Direction::South => {
            for j in 0..grid[0].len() {
                for i in (0..grid.len()).rev() {
                    if grid[i][j] == 'O' && i < grid.len() - 1 {
                        for k in i+1..grid.len() {
                            if grid[k][j] == '.' {
                                grid[k][j] = 'O';
                                grid[k-1][j] = '.';
                            } else if grid[k][j] == '#' || grid[k][j] == 'O' {
                                break;
                            }
                        }
                    }
                    
                }
            }
        },
        Direction::East => {
            for i in 0..grid.len() {
                for j in (0..grid[0].len()).rev() {
                    if grid[i][j] == 'O' && j < grid[0].len() - 1 {
                        for k in j+1..grid[0].len() {
                            if grid[i][k] == '.' {
                                grid[i][k] = 'O';
                                grid[i][k-1] = '.';
                            } else if grid[i][k] == '#' || grid[i][k] == 'O' {
                                break;
                            }
                        }
                    }
                    
                }
            }
        },
        Direction::West => {
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 'O' && j > 0 {
                        for k in (0..=j-1).rev() {
                            if grid[i][k] == '.' {
                                grid[i][k] = 'O';
                                grid[i][k+1] = '.';
                            } else if grid[i][k] == '#' || grid[i][k] == 'O' {
                                break;
                            }
                        }
                    }
                    
                }
            }
        },
    }
}

fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    
    tilt(&mut grid, Direction::North);

    grid
        .iter()
        .enumerate()
        .map(|(index, chars)| {
            (grid.len() - index) * (chars.iter().fold(0, |acc, c| if *c == 'O' { acc + 1 } else { acc }))
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let initial_grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();

    let mut grid = initial_grid.clone();
    let mut seen: HashSet<Vec<Vec<char>>> = HashSet::new();
    let mut num_loops: usize = 0;

    let repeated_grid: Vec<Vec<char>>;
    loop {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);

        if seen.contains(&grid) {
            repeated_grid = grid.clone();
            break;
        } else {
            seen.insert(grid.clone());
        }
        num_loops += 1;
    }

    let start = repeated_grid;
    let mut grid = start.clone();
    let mut period: usize = 1;
    loop {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);
        if grid == start {
            break;
        }
        period += 1;
    }

    let mut grid = initial_grid.clone();
    for _ in 0..(num_loops + ((1000000000 - num_loops) % period)) {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);
    }
    
    grid
        .iter()
        .enumerate()
        .map(|(index, chars)| {
            (grid.len() - index) * (chars.iter().fold(0, |acc, c| if *c == 'O' { acc + 1 } else { acc }))
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/14.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert!(part1(input) == 136);
    }

    #[test]
    fn test_part2() {
        let input = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert!(part2(input) == 64);
    }
  
}