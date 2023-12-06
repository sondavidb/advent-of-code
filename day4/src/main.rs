use std::env;
use std::fs;
use std::vec::Vec;

fn part2(lines: &Vec<&str>) {
    let mut multiplier: Vec<i32> = vec![1; lines.len()];

    for i in 0..lines.len() {
        let line = *lines.get(i).unwrap();
        if line == "" {
            continue;
        }

        let game_trimmed: Vec<String> = line[10..].to_string().split(" | ").map(|s| s.to_string()).collect();

        let mut card: Vec<String> = game_trimmed.get(0).unwrap().to_string().split(" ")
            .map(|s| s.to_string()).collect();
        card.retain(|x: &String| *x != "");
        let mut card_nums: Vec<i32> = card.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        card_nums.sort();

        let mut winners: Vec<String> = game_trimmed.get(1).unwrap().to_string().split(" ")
            .map(|s| s.to_string()).collect();
        winners.retain(|x: &String| *x != "");
        let mut winning_nums: Vec<i32> = winners.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        winning_nums.sort();

        let mut matches_found = 0;

        for num in card_nums {
            let index = winning_nums.binary_search(&num);
            if index.is_ok() {
                matches_found += 1;
                winning_nums.remove(index.unwrap());
            }
        }

        println!("{}", matches_found);
        println!("{:?}", multiplier);
        let factor = *multiplier.get(i).unwrap();
        for j in 1..matches_found+1 {
            if i+j < multiplier.len() {
                multiplier[i+j] += factor;
            }
        }
        println!("{:?}\n", multiplier);
    }
    let sum: i32 = multiplier.iter().sum();
    println!("{}", sum);
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;

    for line in lines {
        if line == &"" {
            continue;
        }

        let game_trimmed: Vec<String> = line[10..].to_string().split(" | ").map(|s| s.to_string()).collect();

        let mut card: Vec<String> = game_trimmed.get(0).unwrap().to_string().split(" ")
            .map(|s| s.to_string()).collect();
        card.retain(|x: &String| *x != "");
        let mut card_nums: Vec<i32> = card.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        card_nums.sort();

        let mut winners: Vec<String> = game_trimmed.get(1).unwrap().to_string().split(" ")
            .map(|s| s.to_string()).collect();
        winners.retain(|x: &String| *x != "");
        let mut winning_nums: Vec<i32> = winners.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        winning_nums.sort();

        let mut score = 0;
        // println!("{:?}", winning_nums);
        for num in card_nums {
            // print!("{} ", num);
            let index = winning_nums.binary_search(&num);
            if index.is_ok() {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
                
                winning_nums.remove(index.unwrap());
            }
        }
        // println!();
        // println!("{}\n", score);
        sum += score;
    }
    println!("{}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let lines:Vec<&str> = data.split('\n').collect();

    part1(&lines);
    part2(&lines);
}