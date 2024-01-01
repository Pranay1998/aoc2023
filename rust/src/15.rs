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
            let lens_list = lens_map.entry(lens.hash).or_insert(vec![]);

            match lens_list.iter().position(|l| l.name == lens.name) {
                Some(index) => { lens_list[index] = lens; },
                None => { lens_list.push(lens.clone()); }
            }
        } else if lens.contains('-') {
            let key = lens.split_once('-').unwrap().0;
            let hash = hash(key);

            if let Some(lens_list) = lens_map.get_mut(&hash) {
                lens_list.retain(|lens| lens.name != key)
            }
        }
    }

    lens_map
        .iter()
        .map(|(hash, lens_list)| {
            lens_list
                .iter()
                .enumerate()
                .map(|(index, lens)| (hash + 1) * (index as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum()
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

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert!(part2(input) == 145);
    }
}