use std::error::Error;
use std::fs;
use day1::quicksort;

fn main() {
    let puzzle_input = match read_file("input.txt") {
        Ok(i) => i,
        Err(_) => panic!("Wrong file name idiot"),
    };

    let result = greatest_calories(&puzzle_input);
    println!("The elf carrying the most calories has {result} calories.");

    let top_three_elves = top_three_elves(&puzzle_input);
    let sum_top_three = sum_of_vec(&top_three_elves);
    println!("The three elves carrying the most calories have a total of {sum_top_three} calories.");
}

fn sum_of_vec(vec: &Vec<u32>) -> u32 {
    let mut result = 0;

    for item in vec {
        result += item;
    }

    result
}

fn top_three_elves(puzzle_input: &String) -> Vec<u32> {
    let items = split_by_item(puzzle_input);
    let mut calories_per_elf = split_by_elf(items);
    quicksort(&mut calories_per_elf[..]);

    let mut top_three: Vec<u32> = vec![];
    let mut end = calories_per_elf.len() - 1;

    for _ in 0..3 {
        top_three.push(calories_per_elf[end]);
        end -= 1;
    }

    top_three
} 

fn greatest_calories(puzzle_input: &String) -> u32 {
    let items = split_by_item(puzzle_input);
    let calories_per_elf = split_by_elf(items);
    find_greatest(calories_per_elf)
}

fn find_greatest(calories_per_elf: Vec<u32>) -> u32 {
    let mut current_largest = 0;

    for cal in calories_per_elf {
        if cal > current_largest {
            current_largest = cal;
        }
    }

    current_largest
}

fn split_by_item<'a>(puzzle_input: &'a String) -> Vec<&'a str> {
    let items: Vec<&str> = puzzle_input.split("\n").map(|item| item.trim()).collect();

    items
}

fn split_by_elf(items: Vec<&str>) -> Vec<u32> {
    let mut elf_count: Vec<u32> = Vec::new();
    let mut sum = 0;

    for item in items {
        if item == "" {
            elf_count.push(sum);
            sum = 0;

            continue;
        }

        let parsed: u32 = item.parse().unwrap_or_else(|err| {
            panic!("Failed to parse &str to u32. Err: {err}");
        });

        sum += parsed;
    }

    elf_count
}

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(file_name)?;

    Ok(input)
}