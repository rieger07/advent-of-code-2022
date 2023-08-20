use std::collections::VecDeque;
use std::fs::read_to_string;

fn vector_has_no_dupes(deque: VecDeque<char>, size_of_marker: &u16) -> bool {
    //This seemed like the easiest way to check that there are no dupes in a vector
    let mut vector: Vec<_> = deque.into();
    vector.sort();
    vector.dedup();
    //If the legth after dedup == len before dedup, we are gucci
    u16::try_from(vector.len()).unwrap() == *size_of_marker
}

//Making this return a result helped with debugging a ton
fn find_start_of_packet_return_character_number(
    datastream: &String,
    size_of_marker: u16,
) -> Result<u16, u16> {
    let mut chars = datastream.chars();
    let mut window: VecDeque<char> = VecDeque::new();
    let mut index = 0;
    //Create the window
    for _ in 0..size_of_marker {
        index += 1;
        window.push_back(chars.next().unwrap());
    }
    //If we got lucky in the first marker size window
    if vector_has_no_dupes(window.clone(), &size_of_marker) {
        //Return the index
        return Ok(index);
    } else {
        //Iterate through the rest of the characters, removing from front and pushing back
        for character in chars {
            index += 1;
            window.pop_front();
            window.push_back(character);
            //Exit early when we have found our marker
            if vector_has_no_dupes(window.clone(), &size_of_marker) {
                return Ok(index);
            }
        }
    }
    Err(index)
}

fn main() {
    let s = read_to_string(
        "/home/srieger/Documents/0.Projects/Programming/advent-of-code-2022/day6/input.txt",
    )
    .unwrap();
    println!(
        "First start-of-packet marker after character {}",
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999)
    );
    println!(
        "First start-of-message marker: {}",
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999)
    );
}

#[test]
fn test1() {
    let s = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
    //Find the marker
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999),
        7
    );
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999),
        19
    );
}

#[test]
fn test2() {
    let s = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
    //Find the marker
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999),
        5
    );
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999),
        23
    );
}

#[test]
fn test3() {
    let s = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
    //Find the marker
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999),
        6
    );
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999),
        23
    );
}

#[test]
fn test4() {
    let s = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
    //Find the marker
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999),
        10
    );
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999),
        29
    );
}

#[test]
fn test5() {
    let s = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
    //Find the marker
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 4)
            .ok()
            .unwrap_or(9999),
        11
    );
    assert_eq!(
        find_start_of_packet_return_character_number(&s, 14)
            .ok()
            .unwrap_or(9999),
        26
    );
}
