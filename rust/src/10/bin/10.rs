use std::{collections::{HashSet, VecDeque}, vec};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
}

// Using signed int to avoid underflow when moving
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Coord {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return Coord { row: row as i32, col: col as i32 };
            }
        }
    }
    panic!("No start found");
}

fn valid_coord(grid: &Vec<Vec<char>>, c: Coord) -> bool {
    if c.row < 0 || c.col < 0 || c.row >= (grid.len() as i32) || c.col >= (grid[0].len() as i32)  {
        return false;
    }
    return true;
}

fn valid_connection(grid: &Vec<Vec<char>>, start: Coord, end: Coord, direction: Direction) -> bool {
    if !valid_coord(grid, start) || !valid_coord(grid, end) {
        return false;
    }

    let start = grid[start.row as usize][start.col as usize];
    let end = grid[end.row as usize][end.col as usize];
    match direction {
        Direction::Up => (start == 'S' || start == '|' || start == 'L' || start == 'J') && (end == 'S' || end == '|' || end == '7' || end == 'F'),
        Direction::Down => (start == 'S' || start == '|' || start == '7' || start == 'F') && (end == 'S' || end == '|' || end == 'L' || end == 'J'),
        Direction::Left => (start == 'S' || start == '-' || start == 'J' || start == '7') && (end == 'S' || end == '-' || end == 'L' || end == 'F'),
        Direction::Right => (start == 'S' || start == '-' || start == 'L' || start == 'F') && (end == 'S' || end == '-' || end == 'J' || end == '7'),
    }
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start: Coord = find_start(&grid);

    // Regular BFS
    let mut max_dist: usize = 0;
    let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
    let mut visited: HashSet<Coord> = HashSet::new();
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

        for (c, dir) in moves {
            if valid_connection(&grid, coord, c, dir) && !visited.contains(&c) {
                max_dist = std::cmp::max(max_dist, cost + 1);
                queue.push_back((c, cost + 1));
            }
        }
    }
   max_dist 
}

fn get_dir(start: Coord, end: Coord) -> Direction {
    if start.row == end.row {
        return if start.col < end.col { Direction::Right } else { Direction::Left };
    } else if start.col == end.col {
        return if start.row < end.row { Direction::Down } else { Direction::Up };
    }
    panic!("Invalid start and end, {:?}, {:?}", start, end)
}

// https://en.wikipedia.org/wiki/Shoelace_formula   
fn area(vertices: &Vec<Coord>) -> f32 {
    let sum: i32 = vertices 
        .iter().enumerate()
        .map(|(i, _)| (i, ((i+1) % vertices.len())))
        .map(|(i, j)| {
            (vertices[i].row*vertices[j].col) - (vertices[i].col*vertices[j].row) 
        })
        .sum();
    (sum.abs() as f32) / 2f32
}

// https://en.wikipedia.org/wiki/Pick%27s_theorem 
fn num_internal_points(area: f32, loop_path: &VecDeque<Coord>) -> f32 {
    return area + 1f32 - ((loop_path.len() as f32)/2f32);
}

fn part2(input: &str) -> f32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&grid); 

    // DFS until loop is detected
    // Queue is in format <coord, prev_coord, prev_direction>
    // Grid is guaranteed to only have one loop
    let mut queue: VecDeque<(Coord, Option<Coord>, Option<Direction>)> = VecDeque::new();
    let mut loop_path: VecDeque<Coord> = VecDeque::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    
    queue.push_front((start, None, None));
    'dfs: while queue.len() != 0 {
        let (coord, prev_coord, prev_dir) = queue.pop_front().unwrap();
        visited.insert(coord);
        
        if let Some(prev_coord) = prev_coord {
            while loop_path.len() > 0 && *loop_path.back().unwrap() != prev_coord {
                loop_path.pop_back();
            }
        }
        loop_path.push_back(coord);

        let moves = [
            (coord.up(), Direction::Up),
            (coord.down(), Direction::Down),
            (coord.left(), Direction::Left),
            (coord.right(), Direction::Right),
        ];

        for (c, dir) in moves {
            if !valid_connection(&grid, coord, c, dir) {
                continue;
            } 
            if c == start && prev_dir.is_some_and(|prev_dir| dir != prev_dir.rev()) {
                // Loop detected
                break 'dfs;
            }
            if !visited.contains(&c) {
                queue.push_front((c, Some(coord), Some(dir)));
            }
        }
    }
 
    let mut vertices: Vec<Coord> = vec![];
    let mut dir: Direction = get_dir(loop_path[loop_path.len() - 1], loop_path[0]);
    loop_path
        .iter()
        .enumerate()
        .map(|(i, _)| (i, ((i+1) % loop_path.len())))
        .map(|(i, j)| (i, get_dir(loop_path[i], loop_path[j])))
        .for_each(|(i, new_dir)| {
            if new_dir != dir {
                vertices.push(loop_path[i]);
                dir = new_dir;
            }
        });

    let area = area(&vertices);
    num_internal_points(area, &loop_path)
}

fn main() {
    let input = std::fs::read_to_string("inputs/10.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
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
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert!(part2(input) == 1f32);
    }

    #[test]
    fn test_part2_2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert!(part2(input) == 4f32);
    }

    #[test]
    fn test_part2_3() {
        let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
    assert!(part2(input) == 4f32);
    }

    #[test]
    fn test_part_2_4() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert!(part2(input) == 8f32);
    }
}