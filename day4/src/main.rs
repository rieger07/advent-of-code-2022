use core::num;
use std::default;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ElfRange {
    min_left: u8,
    max_left: u8,
    min_right: u8,
    max_right: u8,
}
impl std::fmt::Display for ElfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}..{},{}..{})",
            self.min_left, self.max_left, self.min_right, self.max_right
        )
    }
}
impl FromStr for ElfRange {
    type Err = std::num::ParseIntError;

    //Parses a min and max from the form xxxxx-yyyyy
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix converts a string slice in a given base to u8
        let sides: Vec<&str> = input_str.split(",").collect();
        let left: Vec<&str> = sides[0].split("-").collect();
        let right: Vec<&str> = sides[1].split("-").collect();

        let min_left = u8::from_str(left[0]).ok().unwrap();
        let max_left = u8::from_str(left[1]).ok().unwrap() + 1; //exclusive

        let min_right = u8::from_str(right[0]).ok().unwrap();
        let max_right = u8::from_str(right[1]).ok().unwrap() + 1; //exclusive
        Ok(ElfRange {
            min_left,
            max_left,
            min_right,
            max_right,
        })
    }
}

fn fully_contains(range: &ElfRange) -> Option<bool> {
    let left_series = range.min_left..range.max_left;
    let right_series = range.min_right..range.max_right;
    let a_bool = left_series.len() > right_series.len();
    let larger = match a_bool {
        true => &left_series,
        false => &right_series,
    };
    let smaller = match a_bool {
        true => &right_series,
        false => &left_series,
    };
    let num_contained = larger
        .clone()
        .map(|item| if smaller.contains(&item) { 1 } else { 0 })
        .reduce(|acc, e| acc + e)
        .ok_or(false);

    let smaller_len = smaller.len();

    if num_contained.ok().unwrap() == i32::try_from(smaller_len).unwrap() {
        Some(true)
    } else {
        Some(false)
    }
}

fn touches_at_all(range: &ElfRange) -> Option<bool> {
    let left_series = range.min_left..range.max_left;
    let right_series = range.min_right..range.max_right;
    let a_bool = left_series.len() > right_series.len();
    let larger = match a_bool {
        true => &left_series,
        false => &right_series,
    };
    let smaller = match a_bool {
        true => &right_series,
        false => &left_series,
    };
    let num_contained = larger
        .clone()
        .map(|item| if smaller.contains(&item) { 1 } else { 0 })
        .reduce(|acc, e| acc + e)
        .ok_or(false);

    Some(num_contained.ok() > Some(0))
}

fn read_file(path: String) -> Vec<ElfRange> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| ElfRange::from_str(line).unwrap())
        .collect()
}

fn part_1() {
    let elves = read_file(
        String::from_str(
            "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day4/input.txt",
        )
        .unwrap(),
    );

    let total: u32 = elves
        .iter()
        .map(fully_contains)
        .map(|fully_contained| if fully_contained == Some(true) { 1 } else { 0 })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Total Fully Contained Ranges: {total}");
}

fn main() {
    let elves = read_file(
        String::from_str(
            "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day4/input.txt",
        )
        .unwrap(),
    );

    let total: u32 = elves
        .iter()
        .map(touches_at_all)
        .map(|touched| if touched == Some(true) { 1 } else { 0 })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Total Touching Ranges: {total}");
}

#[test]
fn test_doesnt_contain_left() {
    let range = ElfRange::from_str("10-20,1-3").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(false))
}

#[test]
fn test_doesnt_contain_right() {
    let range = ElfRange::from_str("1-3,10-20").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(false))
}

#[test]
fn test_contains_left() {
    let range = ElfRange::from_str("2-2,1-3").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(true))
}

#[test]
fn test_contains_right() {
    let range = ElfRange::from_str("1-3,2-2").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(true))
}

#[test]
fn test_contains_right2() {
    let range = ElfRange::from_str("1-20,2-5").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(true))
}

#[test]
fn test_doesnt_contains_sizeone() {
    //81-81,80-80
    let range = ElfRange::from_str("81-81,80-80").ok().unwrap();
    assert_eq!(fully_contains(&range), Some(false))
}
#[test]
fn test_read_file() {
    let elves = read_file(
        String::from_str(
            "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day4/example.txt",
        )
        .unwrap(),
    );
    assert!(elves.len() == 6);
    for elf in &elves {
        println!("{}: {}", elf, fully_contains(elf).unwrap_or_default());
    }

    let total: u32 = elves
        .iter()
        .map(fully_contains)
        .map(|fully_contained| {
            if fully_contained.is_some_and(|x| x) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{total}");
    assert!(total == 2)
}

#[test]
fn test_read_file_part_2() {
    let elves = read_file(
        String::from_str(
            "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day4/example.txt",
        )
        .unwrap(),
    );
    assert!(elves.len() == 6);
    for elf in &elves {
        println!("{}: {}", elf, touches_at_all(elf).unwrap_or_default());
    }

    let total: u32 = elves
        .iter()
        .map(touches_at_all)
        .map(|touched| if touched == Some(true) { 1 } else { 0 })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Total Touching Ranges: {total}");
    println!("{total}");
    assert!(total == 4)
}
