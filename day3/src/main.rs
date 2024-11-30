use std::fs;
use std::collections::HashSet;

fn main() {
    let puzzle_input = fs::read_to_string("input.txt").unwrap();
    let puzzle_input: Vec<&str> = puzzle_input.lines().collect();
    
    let groups = puzzle_input.chunks(3).collect::<Vec<_>>();
    let mut group_duplicates: Vec<char> = Vec::new();

    for group in groups {
        group_duplicates.push(find_duplicate_group(group).unwrap());
    }

    let mut priority_sum = 0;

    for duplicate in group_duplicates {
        priority_sum += find_priority(duplicate) as u32;
    }

    println!("{priority_sum}");
}

fn find_duplicate_group(lines: &[&str]) -> Option<char> {
    let lines = lines
        .iter()
        .map(|line| {
            let mut map: HashSet<char> = HashSet::new();

            line.chars().for_each(|c| {
                map.insert(c);
            });

            map
        }).collect::<Vec<HashSet<_>>>();

    for c in lines[0].iter() {
        if lines[1].contains(c) && lines[2].contains(c) {
            return Some(*c);
        }
    }

    None
}

fn find_priority(input: char) -> u8 {
    if !input.is_alphabetic() {
        panic!("Character must be alphabetic.");
    }

    if input.is_lowercase() {
        (input as u8) - b'a' + 1
    } else {
        (input as u8) - b'A' + 27
    }
}