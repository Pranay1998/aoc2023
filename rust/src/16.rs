use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn anti_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    i: usize,
    j: usize,
}

impl Coord {
    fn new(i: usize, j: usize) -> Self {
        Self {
            i,
            j,
        }
    }

    fn mv(&self, direction: Direction, grid: &Vec<Vec<char>>) -> Option<Self> {
        match direction {
            Direction::North => {
                if self.i > 0 {
                    Some(Self::new(self.i - 1, self.j))
                } else {
                    None
                }
            },
            Direction::South => {
                if self.i < grid.len() - 1 {
                    Some(Self::new(self.i + 1, self.j))
                } else {
                    None
                }
            },
            Direction::East => {
                if self.j < grid[0].len() - 1 {
                    Some(Self::new(self.i, self.j + 1))
                } else {
                    None
                }
            },
            Direction::West => {
                if self.j > 0 {
                    Some(Self::new(self.i, self.j - 1))
                } else {
                    None
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Ray {
    coord: Coord,
    direction: Direction,
}

impl Ray {
    fn new(coord: Coord, direction: Direction) -> Self {
        Self {
            coord,
            direction,
        }
    }
}

fn simulate_ray(memo: &mut HashSet<Ray>, ray: Ray, grid: &Vec<Vec<char>>) -> Vec<Ray> {
    if memo.contains(&ray) {
        return vec![];
    } else {
        memo.insert(ray);
    }

    return match grid[ray.coord.i][ray.coord.j] {
        '.' => {
            if let Some(next_coord) = ray.coord.mv(ray.direction, grid) {
                vec![Ray::new(next_coord, ray.direction)]
            } else {
                vec![]
            }
        },
        '|' => {
            if ray.direction == Direction::North || ray.direction == Direction::South {
                if let Some(next_coord) = ray.coord.mv(ray.direction, grid) {
                    vec![Ray::new(next_coord, ray.direction)]
                } else {
                    vec![]
                }
            } else {
                [
                    ray.coord.mv(Direction::North, grid).map(|c| Ray::new(c, Direction::North)),
                    ray.coord.mv(Direction::South, grid).map(|c| Ray::new(c, Direction::South))
                ]
                    .iter()
                    .filter_map(|c| *c)
                    .collect()
            }
        },
        '-' => {
            if ray.direction == Direction::East || ray.direction == Direction::West {
                if let Some(next_coord) = ray.coord.mv(ray.direction, grid) {
                    vec![Ray::new(next_coord, ray.direction)]
                } else {
                    vec![]
                }
            } else {
                [
                    ray.coord.mv(Direction::East, grid).map(|c| Ray::new(c, Direction::East)),
                    ray.coord.mv(Direction::West, grid).map(|c| Ray::new(c, Direction::West))
                ]
                    .iter()
                    .filter_map(|c| *c)
                    .collect()
            }
        },
        '/' => {
            if let Some(next_coord) = ray.coord.mv(ray.direction.clockwise(), grid) {
                vec![Ray::new(next_coord, ray.direction.clockwise())]
            } else {
                vec![]
            }
        },
        '\\' => {
            if let Some(next_coord) = ray.coord.mv(ray.direction.anti_clockwise(), grid) {
                vec![Ray::new(next_coord, ray.direction.anti_clockwise())]
            } else {
                vec![]
            }
        },
        _ => {
            panic!("Invalid char: {}", grid[ray.coord.i][ray.coord.j])
        }
    }
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut memo: HashSet<Ray> = HashSet::new();
    let mut rays: VecDeque<Ray> = VecDeque::new();
    let mut energized_coords: HashSet<Coord> = HashSet::new();

    rays.push_front(Ray { coord: Coord { i: 0, j: 0 }, direction: Direction::East });

    while !rays.is_empty() {
        let ray = rays.pop_front().unwrap();
        energized_coords.insert(ray.coord);
        
        for new_ray in simulate_ray(&mut memo, ray, &grid) {
            rays.push_back(new_ray);
        }
    }
    return energized_coords.len();
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();
    let mut max_energized: usize = 0;

    for j in 0..grid[0].len() {
        let mut memo: HashSet<Ray> = HashSet::new();
        let mut rays: VecDeque<Ray> = VecDeque::new();
        let mut energized_coords: HashSet<Coord> = HashSet::new();
    
        rays.push_front(Ray { coord: Coord { i: 0, j }, direction: Direction::South });
    
        while !rays.is_empty() {
            let ray = rays.pop_front().unwrap();
            energized_coords.insert(ray.coord);
            
            for new_ray in simulate_ray(&mut memo, ray, &grid) {
                rays.push_back(new_ray);
            }
        }
        if energized_coords.len() > max_energized {
            max_energized = energized_coords.len();
        }
    }

    for j in 0..grid[0].len() {
        let mut memo: HashSet<Ray> = HashSet::new();
        let mut rays: VecDeque<Ray> = VecDeque::new();
        let mut energized_coords: HashSet<Coord> = HashSet::new();
    
        rays.push_front(Ray { coord: Coord { i: grid.len() - 1, j }, direction: Direction::North });
    
        while !rays.is_empty() {
            let ray = rays.pop_front().unwrap();
            energized_coords.insert(ray.coord);
            
            for new_ray in simulate_ray(&mut memo, ray, &grid) {
                rays.push_back(new_ray);
            }
        }
        if energized_coords.len() > max_energized {
            max_energized = energized_coords.len();
        }
    }

    for i in 0..grid.len() {
        let mut memo: HashSet<Ray> = HashSet::new();
        let mut rays: VecDeque<Ray> = VecDeque::new();
        let mut energized_coords: HashSet<Coord> = HashSet::new();
    
        rays.push_front(Ray { coord: Coord { i, j: 0 }, direction: Direction::East });
    
        while !rays.is_empty() {
            let ray = rays.pop_front().unwrap();
            energized_coords.insert(ray.coord);
            
            for new_ray in simulate_ray(&mut memo, ray, &grid) {
                rays.push_back(new_ray);
            }
        }
        if energized_coords.len() > max_energized {
            max_energized = energized_coords.len();
        }
    }

    for i in 0..grid.len() {
        let mut memo: HashSet<Ray> = HashSet::new();
        let mut rays: VecDeque<Ray> = VecDeque::new();
        let mut energized_coords: HashSet<Coord> = HashSet::new();
    
        rays.push_front(Ray { coord: Coord { i, j: grid[0].len() - 1 }, direction: Direction::West });
    
        while !rays.is_empty() {
            let ray = rays.pop_front().unwrap();
            energized_coords.insert(ray.coord);
            
            for new_ray in simulate_ray(&mut memo, ray, &grid) {
                rays.push_back(new_ray);
            }
        }
        if energized_coords.len() > max_energized {
            max_energized = energized_coords.len();
        }
    }

    return max_energized;
}

fn main() {
    let input = std::fs::read_to_string("inputs/16.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
let input =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert!(part1(input) == 46);
    }

    #[test]
    fn test_part2() {
let input =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert!(part2(input) == 51);
    }
}