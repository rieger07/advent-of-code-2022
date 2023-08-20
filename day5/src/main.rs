// use core::num;
// use std::default;
use std::{fs::read_to_string, str::FromStr};
// use std::str::FromStr;
struct Crate {
    letter: String,
}
struct Tower {
    crates: Vec<Crate>,
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;
    //Parses amount, from and to for 'move # from # to #'
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix converts a string slice in a given base to u8
        let parts: Vec<&str> = input_str.split(" ").collect();
        let amount: usize = usize::from_str(parts[1]).ok().unwrap_or_default();
        let from: usize = usize::from_str(parts[3]).ok().unwrap_or_default() - 1;
        let to: usize = usize::from_str(parts[5]).ok().unwrap_or_default() - 1;

        Ok(Instruction { amount, from, to })
    }
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.letter)
    }
}

impl std::fmt::Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for my_crate in &self.crates {
            write!(f, "{}", my_crate)?;
        }
        write!(f, "")
    }
}

fn read_initial_crate_state(path: &String, header_len: i32) -> Vec<Tower> {
    //Init the tower vec
    let mut towers = Vec::<Tower>::new();
    //Read in the file
    let file_read_in = read_to_string(path).unwrap();
    //Get an iterator
    let mut header_iter = file_read_in.lines().into_iter();

    //Advance iterator to the line that has the tower labels
    for _ in 1..header_len {
        header_iter.next();
    }
    //Last line divided by 4 +1 equals num towers
    let num_towers = header_iter.next().unwrap_or_default().chars().count() / 4 + 1;

    //Push the empty towers onto the tower vector
    for t in 0..num_towers {
        towers.push(Tower {
            crates: Vec::<Crate>::new(),
        })
    }
    //Get new iterator
    let mut lines = file_read_in.lines().into_iter();
    //Read through the header
    for line_no in 0..header_len {
        //Get the raw chars
        let chars = lines.next().unwrap_or(" ").chars();
        let mut char_no = 0;
        for char in chars {
            char_no += 1;
            if char.is_alphabetic() {
                //For the given tower idx, splice a Crate into the front of the tower since we read top down
                towers[char_no / 4].crates.splice(
                    0..0,
                    vec![Crate {
                        letter: char.to_string(),
                    }],
                );
            }
        }
    }
    //Return towers
    towers
}

fn execute_instructions(towers: &mut Vec<Tower>, path: &String, header_len: i32) {
    //Read in the file
    let file_read_in = read_to_string(path).unwrap();
    //Get an iterator
    let mut instruction_iter = file_read_in.lines().into_iter();

    //Advance iterator to the line that has the start of the instructions
    for _ in 0..header_len + 1 {
        instruction_iter.next();
    }

    for instruction in instruction_iter {
        let i = Instruction::from_str(instruction).ok().unwrap();
        for _ in 0..i.amount {
            let to_move: Crate = towers[i.from].crates.pop().unwrap();
            towers[i.to].crates.push(to_move);
        }
    }
}

fn main() {
    let towers = read_initial_crate_state(
        &"/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day5/example.txt"
            .to_string(),
        5,
    );
    for tower in towers {
        println!("{}", tower)
    }
}

#[test]
fn test_1() {
    let path: String =
        "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day5/example.txt"
            .to_string();
    let header_len = 4;
    let mut towers = read_initial_crate_state(&path, header_len);

    execute_instructions(&mut towers, &path, header_len);

    for tower in towers {
        println!("{}", tower)
    }
}

#[test]
fn test_2() {
    let path: String =
        "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day5/input.txt"
            .to_string();
    let header_len = 9;
    let mut towers = read_initial_crate_state(&path, header_len);

    execute_instructions(&mut towers, &path, header_len);

    for tower in towers {
        println!("{}", tower.crates[tower.crates.len() - 1])
    }
}
