use itertools::Itertools;
use std::{vec, cmp::{max, min}, collections::HashMap};

fn soln(input: &str, rate: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut row_map: HashMap<usize, usize> = HashMap::new();
    let mut col_map: HashMap<usize, usize> = HashMap::new();
    
    for i in 0..grid.len() {
        let row_empty = grid[i].iter().all(|&c| c == '.');
        let i_p = if i == 0 {
            if row_empty { rate - 1 } else { 0 }
        } else {
            if row_empty { row_map[&(i-1)] + rate } else { row_map[&(i-1)] + 1 }    
        };
        row_map.insert(i, i_p);
    }

    for j in 0..grid[0].len() {
        let col_empty = grid.iter().all(|row| row[j] == '.');
        let j_p = if j == 0 {
            if col_empty { rate - 1 } else { 0 }
        } else {
            if col_empty { col_map[&(j-1)] + rate } else { col_map[&(j-1)] + 1 }    
        };
        col_map.insert(j, j_p); 
    }

    let mut galaxy_map: Vec<(usize, usize)> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == '#' {
                galaxy_map.push((i, j));
            }
        }
    }
    
    galaxy_map
        .iter()
        .combinations(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            let (a, b) = ((row_map[&a.0], col_map[&a.1]), (row_map[&b.0], col_map[&b.1]));
            (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1))
        })
        .sum() 
}

fn main() {
    let input = std::fs::read_to_string("inputs/11.txt").unwrap();
    println!("Part 1 Answer: {}", soln(&input, 2));
    println!("Part 2 Answer: {}", soln(&input, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert!(soln(input, 2) == 374);
    }

    #[test]
    fn test2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert!(soln(input , 100) == 8410);
    }
  
}