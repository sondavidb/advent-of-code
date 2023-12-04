use std::env;
use std::fs;
use std::vec::Vec;
use phf::phf_map;

static WORD_MAP: phf::Map<&'static str, char> = phf_map!{
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

fn convert_to_num_char(arr: [char;5]) -> char {
    let mut chars: Vec<char> = Vec::new();
    for chr in arr {
        if chr == ' ' {
            break;
        }
        chars.push(chr);
    }

    let mut str = String::from_iter(chars);
    while str.len() >= 3 {
        let ret_chr:Option<&char> = WORD_MAP.get(&str);
        
        if ret_chr != None {
            return *ret_chr.unwrap();
        }

        str.remove(0);
    }

    return ' ';
}

fn shift_and_insert(arr: &mut [char;5], a: char) {
    for chr in arr.iter_mut() {
        if *chr == ' ' {
            *chr = a;
            return
        }
    }
    // If we get here, we need to shift
    for i in 1..arr.len() {
        arr[i-1] = arr[i];
    }
    arr[arr.len()-1] = a;
}

fn clean_arr(arr: &mut [char;5]) {
    for chr in arr.iter_mut() {
        *chr = ' ';
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let word_list = data.split('\n');

    let mut sum = 0;

    for word in word_list {
        // println!("{}", word);

        let mut num_found = false;
        let mut first_num_found = ' ';
        let mut last_num_found = ' ';

        let mut curr_word = [' ', ' ', ' ', ' ', ' '];
        let mut word_len = 0;

        for chr in word.chars() {
            if chr.is_alphanumeric() {
                let mut num_chr = chr;

                if num_chr.is_alphabetic() {
                    shift_and_insert(&mut curr_word, chr);

                    if word_len < 5 {
                        word_len += 1;
                    }
                    
                    // Attempt to parse the last few letters
                    if word_len >= 3 {
                        num_chr = convert_to_num_char(curr_word);
                    }
                }

                // If above did not make a char, then this gets skipped.
                if num_chr.is_numeric() {
                    if !num_found {
                        first_num_found = num_chr;
                        num_found = true;
                    }
                    last_num_found = num_chr;
                }
                
                // If original char was numeric, clean
                if chr.is_numeric() {
                    clean_arr(&mut curr_word);
                    word_len = 0;
                }
            } else {
                clean_arr(&mut curr_word);
                word_len = 0;
            }
        }

        // println!("{}{}\n", first_num_found, last_num_found);
        if num_found {
            let final_num = format!("{first_num_found}{last_num_found}");
            sum += final_num.parse::<i32>().unwrap();
        }
    }

    println!("{sum}")
}