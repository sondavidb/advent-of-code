use std::env;
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

fn create_game() -> HashMap<&'static str, i32> {
    let mut game: HashMap<&'static str, i32> = HashMap::new();
    game.insert("red", 0);
    game.insert("blue", 0);
    game.insert("green", 0);
    return game;
}

fn validate_game(criteria: &HashMap<&str, i32>, curr_game: &HashMap<&str, i32>) -> bool {
    return (criteria.get("red").unwrap() >= curr_game.get("red").unwrap())
        && (criteria.get("green").unwrap() >= curr_game.get("green").unwrap())
        && (criteria.get("blue").unwrap() >= curr_game.get("blue").unwrap())
}

fn power(curr_game: &HashMap<&str, i32>) -> i32 {
    return curr_game.get("red").unwrap() * curr_game.get("green").unwrap() * curr_game.get("blue").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let word_list = data.split('\n');

    let mut criteria = create_game();
    criteria.insert("red", 12);
    criteria.insert("green", 13);
    criteria.insert("blue", 14);

    let mut id = 1;
    let mut sum = 0;
    for line in word_list {
        if line == "" {
            continue;
        }

        let game_prefix = format!("Game {}: ", id);
        let game_trimmed = line[game_prefix.len()..].to_string();
        let draws = game_trimmed.trim().split("; ");
        
        let mut curr_game = create_game();

        for draw in draws {
            let cubes = draw.split(", ");
            for cube in cubes {
                let new_arr: Vec<&str> = cube.split(" ").collect();
                let num = new_arr.get(0).unwrap().parse::<i32>().unwrap();
                let color = *new_arr.get(1).unwrap();

                if num > *curr_game.get(color).unwrap() {
                    curr_game.insert(color, num);
                }
            }
        }

        sum += power(&curr_game);
        id += 1;
    }
    println!("{}", sum)
}
