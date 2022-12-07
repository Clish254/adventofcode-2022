use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let path = Path::new("src/bin/day1/input.txt");
    File::open(path)?.read_to_string(&mut input)?;
    // let input = include_str!("./input.txt");
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &String) {
    let mut elf_food_groups: Vec<&str> = input.split("\n\n").collect();
    // remove last item which is an empty string
    elf_food_groups.pop();
    let mut largest: u32 = 0;
    for group in &elf_food_groups {
        let sum = get_sum(group);
        if largest < sum {
            largest = sum
        }
    }
    println!("part 1 {}", largest);
}

fn part_two(input: &String) {
    let mut elf_food_groups: Vec<&str> = input.split("\n\n").collect();
    // remove last item which is an empty string
    elf_food_groups.pop();
    let mut group_sums: Vec<u32> = Vec::new();
    for group in &elf_food_groups {
        let sum = get_sum(group);
        group_sums.push(sum);
    }
    group_sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let slice = &group_sums[0..=2];
    let sum: u32 = slice.iter().sum();
    println!("Part 2 {}", sum)
}

fn get_sum(elf_food_group: &str) -> u32 {
    let elf_food_group_arr: Vec<&str> = elf_food_group.split("\n").collect();
    let sum: u32 = elf_food_group_arr
        .iter()
        .map(|f| f.parse::<u32>().unwrap())
        .sum();
    sum
}
