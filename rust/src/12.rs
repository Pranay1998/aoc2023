use core::panic;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Spring {
    Dot,
    Hash,
    Unknown
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Dot,
            '#' => Spring::Hash,
            '?' => Spring::Unknown,
            _ => panic!("Invalid char")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>
}

impl Record {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Record { springs, groups }
    }
}

fn num_ways(memo: &mut HashMap<Record, usize>, record: &Record) -> usize {
    if let Some(value) = memo.get(record) {
        return *value;
    }

    if record.groups.is_empty() {
        let value = if record.springs.iter().any(|c| *c == Spring::Hash) { 0 } else { 1 };
        memo.insert(record.clone(), value);
        return value;
    }

    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        memo.insert(record.clone(), 0);
        return 0;
    }

    if record.springs[0] == Spring::Dot {
        let solutions = num_ways(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
        memo.insert(record.clone(), solutions);
        return solutions;
    }

    let mut solutions = 0;
    let cur = record.groups[0];
    let all_non_operational = record.springs[0..cur].iter().all(|c| *c != Spring::Dot);    
    let end = (cur + 1).min(record.springs.len());
    if all_non_operational && ((record.springs.len() > cur && record.springs[cur] != Spring::Hash)|| record.springs.len() <= cur) {
        solutions += num_ways(
            memo,
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec())
        );
    }

    if record.springs[0] == Spring::Unknown {
        solutions += num_ways(
            memo,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone())
        );
    }
    memo.insert(record.clone(), solutions);
    return solutions;
}

fn part1(input: &str) -> usize {
    let mut total: usize = 0;
    let mut memo: HashMap<Record, usize> = HashMap::new();
    for line in input.lines() {

        let splits: Vec<&str> = line.split(" ").collect();
        let springs: Vec<Spring> = splits[0].chars().map(|x| x.into()).collect();
        let groups: Vec<usize> = splits[1].split(",").map(|x| x.parse().unwrap()).collect();
        let record = Record { springs, groups };
        total += num_ways(&mut memo, &record);
        
    }
    total
}


fn part2(input: &str) -> usize {
    let mut total: usize = 0;
    let mut memo: HashMap<Record, usize> = HashMap::new();
    for line in input.lines() {

        let splits: Vec<&str> = line.split(" ").collect();
        let springs = format!("{}?{}?{}?{}?{}", splits[0], splits[0], splits[0], splits[0], splits[0]);
        let groups = format!("{},{},{},{},{}", splits[1], splits[1], splits[1], splits[1], splits[1]);
        let springs: Vec<Spring> = springs.chars().map(|x| x.into()).collect();
        let groups: Vec<usize> = groups.split(",").map(|x| x.parse().unwrap()).collect();
        let record = Record { springs, groups };
        total += num_ways(&mut memo, &record);
        
    }
    total
}


fn main() {
    let input = std::fs::read_to_string("inputs/12.txt").unwrap();
    println!("Part 1 Answer: {}", part1(&input));
    println!("Part 2 Answer: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        println!("{}", part1(input));
        assert!(part1(input) == 21);
    }

    #[test]
    fn test_part2() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert!(part2(input) == 525152);
    }
  
}