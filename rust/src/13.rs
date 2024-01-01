 
fn part1(input: &str) -> usize {
    let mut total: usize = 0;
    'block_loop: for block in input.split("\n\n") {
        let lines = block.split("\n").map(|str| str.to_string()).collect::<Vec<String>>();
        let mut vertical_flatten: Vec<String> = Vec::with_capacity(lines[0].len());

        for i in 0..lines[0].len() {
            let mut vertical = String::new();
            for j in 0..lines.len() {
                vertical.push(lines[j].chars().nth(i).unwrap());
            }
            vertical_flatten.push(vertical);
        }

        let horizontal_flatten = lines;
        let vertical_flatten = vertical_flatten;

        // Check horizontal lines between start and end
        for (i1, i2) in (0..horizontal_flatten.len()-1).map(|start| (start, start + 1)) {
            let mut start = i1 as i32;
            let mut end = i2 as i32;

            let mut valid = false;

            while start >= 0 && end < horizontal_flatten.len() as i32 {
                if horizontal_flatten[start as usize] == horizontal_flatten[end as usize] {
                    valid = true;
                } else {
                    valid = false;
                    break;
                }
                start -= 1;
                end += 1;
            }

            if valid {   
                total += 100*i2;
                continue 'block_loop;
            }
        }

        // Check vertical lines between start and end
        for (i1, i2) in (0..vertical_flatten.len()-1).map(|start| (start, start + 1)) {
            let mut start = i1 as i32;
            let mut end = i2 as i32;

            let mut valid = false;

            while start >= 0 && end < vertical_flatten.len() as i32 {
                if vertical_flatten[start as usize] == vertical_flatten[end as usize] {
                    valid = true;
                } else {
                    valid = false;
                    break;
                }
                start -= 1;
                end += 1;
            }

            if valid {   
                total += i2;
                continue 'block_loop;
            }
        }
    }
    total
}

fn str_mismatches(s1: &str, s2: &str) -> usize {
    let mut mismatches = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
            mismatches += 1;
        }
    }
    mismatches
}

fn part2(input: &str) -> usize {
    let mut total: usize = 0;
    'block_loop: for block in input.split("\n\n") {
        let lines = block.split("\n").map(|str| str.to_string()).collect::<Vec<String>>();
        let mut vertical_flatten: Vec<String> = Vec::with_capacity(lines[0].len());

        for i in 0..lines[0].len() {
            let mut vertical = String::new();
            for j in 0..lines.len() {
                vertical.push(lines[j].chars().nth(i).unwrap());
            }
            vertical_flatten.push(vertical);
        }

        let horizontal_flatten = lines;
        let vertical_flatten = vertical_flatten;

        // Check horizontal lines between start and end
        for (i1, i2) in (0..horizontal_flatten.len()-1).map(|start| (start, start + 1)) {
            let mut start_iter = horizontal_flatten[0..=i1].iter().rev();
            let mut end_iter = horizontal_flatten[i2..].iter();

            let mut mismatches = 0;

            while let (Some(start), Some(end)) = (start_iter.next(), end_iter.next()) {
                mismatches += str_mismatches(start, end);
                if mismatches > 1 {
                    break;
                }
            } 

            if mismatches == 1 {   
                total += 100*i2;
                continue 'block_loop;
            }
        }

        // Check vertical lines between start and end
        for (i1, i2) in (0..vertical_flatten.len()-1).map(|start| (start, start + 1)) {
            let mut start_iter = vertical_flatten[0..=i1].iter().rev();
            let mut end_iter = vertical_flatten[i2..].iter();

            let mut mismatches = 0;

            while let (Some(start), Some(end)) = (start_iter.next(), end_iter.next()) {
                mismatches += str_mismatches(start, end);
                if mismatches > 1 {
                    break;
                }
            }

            if mismatches == 1 {   
                total += i2;
                continue 'block_loop;
            }
        }
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("inputs/13.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert!(part1(input) == 405);
    }

    #[test]
    fn test_part2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert!(part2(input) == 400);
    }
}