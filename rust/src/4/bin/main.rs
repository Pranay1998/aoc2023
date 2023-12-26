fn main() {
    let input = std::fs::read_to_string("inputs/4.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut nums: Vec<Vec<i32>> = vec![line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect()];

        while !nums.last().unwrap().iter().all(|x| *x == 0) {
            nums.push(
                nums
                    .last().unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect()
            );
        }

        nums.last_mut().unwrap().push(0);

        for i in (1..nums.len() - 1).rev() {
            let to_add = nums[i-1].last().unwrap() + nums[i].last().unwrap();
            nums[i-1].push(to_add);
        }

        total += nums[0][nums[0].len() - 1]
    }
    total
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        let mut nums: Vec<Vec<i32>> = vec![line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect()];

        while !nums.last().unwrap().iter().all(|x| *x == 0) {
            nums.push(
                nums
                    .last().unwrap()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect()
            );
        }

        nums.last_mut().unwrap().insert(0, 0);

        for i in (1..nums.len() - 1).rev() {
            let to_add = nums[i-1][0] - nums[i][0];
            nums[i-1].insert(0, to_add);
        }

        total += nums[0][0]
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert!(part1(input) == 114);
    }

    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert!(part2(input) == 2);
    }
  
}