use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let path = Path::new("src/bin/day2/input.txt");
    File::open(path)?.read_to_string(&mut input)?;
    // let input = include_str!("./input.txt");
    let lines: Vec<&str> = input.split("\n").collect();
    part_one(&lines);
    part_two(&lines);
    Ok(())
}
fn part_one(lines: &Vec<&str>) {
    let scores = part_one_combinations();
    let mut total = 0;
    for line in lines {
        if let Some(score) = scores.get(line) {
            total += score
        }
    }
    println!("Part one {}", total);
}

fn part_two(lines: &Vec<&str>) {
    let scores = part_two_combinations();
    let mut total = 0;
    for line in lines {
        if let Some(score) = scores.get(line) {
            total += score
        }
    }
    // another way to solve this
    // let mut total = 0;
    // for line in lines {
    //     let line_items: Vec<&str> = line.split(" ").collect();
    //     if line_items.len() == 2 {
    //         let last = line_items[1];
    //         let first = line_items[0];
    //         match last {
    //             "X" => match first {
    //                 "A" => total += 3,
    //                 "B" => total += 1,
    //                 "C" => total += 2,
    //                 _ => (),
    //             },
    //             "Y" => match first {
    //                 "A" => total += 4,
    //                 "B" => total += 5,
    //                 "C" => total += 6,
    //                 _ => (),
    //             },
    //             "Z" => match first {
    //                 "A" => total += 8,
    //                 "B" => total += 9,
    //                 "C" => total += 7,
    //                 _ => (),
    //             },
    //             _ => (),
    //         }
    //     }
    // }
    println!("Part two {}", total);
}

fn part_one_combinations() -> HashMap<&'static str, u32> {
    let mut hm = HashMap::new();
    // get all possible combinations for player moves
    /*
    =====rules=====
    Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    =========Opponent=====
    A - Rock -> 1
    B - Paper -> 2
    C - Scissors -> 3
    ========You=========
    X - Rock -> 1
    Y - Paper -> 2
    Z - Scissors -> 3
    ======outcome score======
    lost - 0
    draw - 3
    win - 6
    */
    hm.insert("A X", 1 + 3);
    hm.insert("A Y", 2 + 6);
    hm.insert("A Z", 3 + 0);
    hm.insert("B X", 1 + 0);
    hm.insert("B Y", 2 + 3);
    hm.insert("B Z", 3 + 6);
    hm.insert("C X", 1 + 6);
    hm.insert("C Y", 2 + 0);
    hm.insert("C Z", 3 + 3);
    hm
}

fn part_two_combinations() -> HashMap<&'static str, u32> {
    let mut hm = HashMap::new();
    // get all possible combinations for player moves
    /*
    =====rules=====
    Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
    =========Opponent=====
    A - Rock -> 1
    B - Paper -> 2
    C - Scissors -> 3
    ========You=========
    X - loss
    Y - draw
    Z - win
    ======outcome score======
    lost - 0
    draw - 3
    win - 6
    */
    hm.insert("A X", 3);
    hm.insert("A Y", 4);
    hm.insert("A Z", 8);
    hm.insert("B X", 1);
    hm.insert("B Y", 5);
    hm.insert("B Z", 9);
    hm.insert("C X", 2);
    hm.insert("C Y", 6);
    hm.insert("C Z", 7);
    hm
}
