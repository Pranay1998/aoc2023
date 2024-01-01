use core::panic;
use std::collections::HashMap;

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| ((acc + c as u32)*17) % 256)
}

fn part1(input: &str) -> u32 {
    input
        .split(',')
        .map(|lens| hash(lens))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lens {
    name: String,
    focal_length: u32,
    hash: u32,
}

impl Lens {
    fn new(input: &str) -> Self {
        let (lens_name, focal_length) = input.split_once('=').unwrap();
        let focal_length = focal_length.parse::<u32>().unwrap();
        let hash = hash(lens_name);

        Self {
            name: lens_name.to_string(),
            focal_length,
            hash,
        }
    }
}

fn part2(input: &str) -> u32 {
    let mut lens_map: HashMap<u32, Vec<Lens>> = HashMap::new();
    for lens in input.split(',') {
        if lens.contains('=') {
            let lens = Lens::new(lens);

            if !lens_map.contains_key(&lens.hash) {
                lens_map.insert(lens.hash, vec![lens]);
            } else {
                let lens_list = lens_map.get_mut(&lens.hash).unwrap();

                let mut found = false;
                for i in 0..lens_list.len() {
                    if lens_list[i].name == lens.name {
                        lens_list[i] = lens.clone();
                        found = true;
                        break;
                    }
                }

                if !found {
                    lens_list.push(lens);
                }
            }
        } else if lens.contains('-') {
            let key = lens.split_once('-').unwrap().0;
            let hash = hash(key);

            match lens_map.get_mut(&hash) {
                Some(lens_list) => {
                    let mut index: Option<usize> = None;

                    for i in 0..lens_list.len() {
                        if lens_list[i].name == key {
                            index = Some(i);
                            break;
                        }
                    }

                    if index.is_some() {
                        lens_list.remove(index.unwrap());
                    }
                },
                None => (),
            }
        } else {
            panic!("Invalid input");
        }
    }

    let mut total: u32 = 0;

    for (hash, lens_list) in lens_map.iter() {
        for (index, lens) in lens_list.iter().enumerate() {
            total += (hash + 1) * (index as u32 + 1) * lens.focal_length;
        }
    }

    total
}

fn main() {
    let input = std::fs::read_to_string("inputs/15.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert!(part1(input) == 1320);
    }
}