use std::env;
use std::fs;
use std::vec::Vec;

fn is_symbol(chr: char) -> bool {
    return chr != '.' && !chr.is_numeric();
}

fn has_symbol_top_left (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y-1).unwrap().get(x-1).unwrap());
}

fn has_symbol_top_center (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y-1).unwrap().get(x).unwrap());
}

fn has_symbol_top_right (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y-1).unwrap().get(x+1).unwrap());
}

fn has_symbol_center_left (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y).unwrap().get(x-1).unwrap());
}

fn has_symbol_center_right (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y).unwrap().get(x+1).unwrap());
}

fn has_symbol_bottom_left (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y+1).unwrap().get(x-1).unwrap());
}

fn has_symbol_bottom_center (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y+1).unwrap().get(x).unwrap());
}

fn has_symbol_bottom_right (x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
    return is_symbol(*board.get(y+1).unwrap().get(x+1).unwrap());
}

fn has_adj_symbol(x: usize, y: usize, num_len: usize, board: &Vec<Vec<char>>) -> bool {
    let mut is_end = true;
    let mut is_beginning = false;

    let width = board.get(0).unwrap().len();

    // Start searching from the rightmost char to the leftmost char
    for i in 0..num_len {
        let mut checks: Vec<&dyn Fn(usize, usize, &Vec<Vec<char>>) -> bool> = Vec::new();
        let x_pos = x - i;

        if i == num_len-1 {
            is_beginning = true;
        }

        // If not bordering top
        if y != 0 {
            checks.push(&has_symbol_top_center);

            if x_pos != 0 && is_beginning {
                checks.push(&has_symbol_top_left);
            }
            
            if x_pos != width-1 && is_end {
                checks.push(&has_symbol_top_right);
            }
        }

        // If not bordering bottom
        if y != board.len()-1 {
            checks.push(&has_symbol_bottom_center);

            if x_pos != 0 && is_beginning {
                checks.push(&has_symbol_bottom_left);
            }
            
            if x_pos != width-1 && is_end {
                checks.push(&has_symbol_bottom_right);
            }
        }

        // If beginning and not bordering left
        if x_pos != 0 && is_beginning {
            checks.push(&has_symbol_center_left);
        }

        // If beginning and not bordering right
        if x_pos != width-1 && is_end {
            checks.push(&has_symbol_center_right);
        }

        // println!("{}, {}: {}", x_pos, y, checks.len());

        for func in checks {
            if func(x_pos, y, board) {
                return true;
            }
        }

        if i == 0 {
            is_end = false;
        }
    }

    return false;
}

fn scan_left(x: usize, y: usize, board: &Vec<Vec<char>>) -> String {
    let mut str = "".to_string();
    let mut len = 1;
    let mut chr = *board.get(y).unwrap().get(x-len).unwrap();

    while x-len != 0 && chr.is_numeric() {
        str = format!("{}{}", chr, str);
        len += 1;

        chr = *board.get(y).unwrap().get(x-len).unwrap();
    }
    return str;
}

fn scan_right(x: usize, y: usize, board: &Vec<Vec<char>>) -> String {
    let mut str = "".to_string();
    let mut len = 1;
    let mut chr = *board.get(y).unwrap().get(x+1).unwrap();

    while x+len != board.get(0).unwrap().len()-1 && chr.is_numeric() {
        str = format!("{}{}", str, chr);
        len += 1;

        chr = *board.get(y).unwrap().get(x+len).unwrap();
    }
    return str;
}

fn check_full_line(x: usize, y: usize, board: &Vec<Vec<char>>, num_list: &mut Vec<i32>) {
    let mut left_str = "".to_string();
    let mut right_str = "".to_string();
    let middle_char = *board.get(y).unwrap().get(x).unwrap();

    if x != 0 && *board.get(y).unwrap().get(x-1).unwrap() != '.' {
        left_str = scan_left(x, y, board);
    }

    if x != board.get(0).unwrap().len()-1 && *board.get(y).unwrap().get(x+1).unwrap() != '.' {
        right_str = scan_right(x, y, board);
    }

    // Treat whole top part as one num
    if middle_char.is_numeric() {
        num_list.push(format!("{}{}{}", left_str, middle_char, right_str)
            .parse::<i32>().unwrap());
    } else {
        if left_str != "" {
            num_list.push(left_str.parse::<i32>().unwrap());
        }

        if right_str != "" {
            num_list.push(right_str.parse::<i32>().unwrap());
        }
    }
}

fn get_adj_nums_ratio(x: usize, y: usize, board: &Vec<Vec<char>>) -> i32 {
    let width = board.get(0).unwrap().len();
    
    let mut num_list: Vec<i32> = Vec::new();
 
    // Check top
    if y != 0 {
        check_full_line(x, y-1, board, &mut num_list);
    }

    // Check bottom
    if y != board.len()-1 {
        check_full_line(x, y+1, board, &mut num_list);
    }
    
    if num_list.len() <= 2 {
       // Check left
        if x != 0 {
            let left_str = scan_left(x, y, board);
            if left_str != "" {
                num_list.push(left_str.parse::<i32>().unwrap());
            }
        }

        if num_list.len() <= 2 {
            // Check right
            if x != width-1 {
                let right_str = scan_right(x, y, board);
                if right_str != "" {
                    num_list.push(right_str.parse::<i32>().unwrap());
                }
            }
        } else {
            return 0;
        }
    }

    if num_list.len() != 2 {
        return 0;
    }

    println!("{} {} {}", num_list.get(0).unwrap(), num_list.get(1).unwrap(), num_list.get(0).unwrap() * num_list.get(1).unwrap());

    return num_list.get(0).unwrap() * num_list.get(1).unwrap();
}

fn part2(board: &Vec<Vec<char>>) {
    let mut sum = 0;
    
    for y in 0..board.len() {
        let line = board.get(y).unwrap();

        for x in 0..line.len() {
            let chr = line[x];

            if chr == '*' {
                println!("Is a star");
                sum += get_adj_nums_ratio(x, y, &board);
            }
        }
    }

    println!("{}", sum);
}

fn part1(board: &Vec<Vec<char>>) {
    let mut sum = 0;

    for y in 0..board.len() {
        let line = board.get(y).unwrap();
        let mut num: String = "".to_string();

        for x in 0..line.len() {
            let chr = line[x];

            if chr.is_numeric() {
                num += &chr.to_string();
            // Num finished
            } else if num != "" {
                if has_adj_symbol(x-1, y, num.len(), &board) {
                    sum += num.parse::<i32>().unwrap();
                }
                num = "".to_string();
            }
        }

        if num != "" {
            if has_adj_symbol(line.len()-1, y, num.len(), &board) {
                sum += num.parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let lines = data.split('\n');

    let mut board = Vec::<Vec::<char>>::new();

    for line in lines {
        if line != "" {
            board.push(line.chars().collect());
        } 
    }

    part1(&board);
    part2(&board);
}
