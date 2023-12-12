use std::env;
use std::fs;
use std::vec::Vec;
use std::ops::Range;

fn part1(sections: &Vec<&str>) {
    let seeds_str: Vec<&str> = sections[0].split(": ").collect();
    let seeds_str_vec: Vec<&str> = seeds_str[1].split(" ").collect();
    let mut seeds: Vec<i64> = seeds_str_vec.iter().map(|s| s.parse::<i64>().unwrap()).collect();

    let mut transformations: Vec<Vec<Vec<i64>>> = Vec::new();

    for i in 1..sections.len() {
        let section_str: Vec<&str> = sections[i].split(":").collect();
        let mut ranges_vec_str: Vec<&str> = section_str[1].split("\n").collect();
        ranges_vec_str.retain(|x: &&str| *x != "");
        let mut ranges: Vec<Vec<i64>> = Vec::new();
    
        for num_str in ranges_vec_str {
            let section_str_vec: Vec<&str> = num_str.split(" ").collect();
            let nums: Vec<i64> = section_str_vec.iter().map(|s| s.parse::<i64>().unwrap()).collect();

            ranges.push(nums);
        }
        
        transformations.push(ranges);
    
    }
    // println!("{:?}", transformations);    

    // println!("{:?}", seeds);
    for seed in &mut seeds {
        // println!("{}", seed);
        for ranges in &transformations {
            for tup in ranges {
                let range = tup[1]..tup[1]+tup[2];
                if range.contains(seed) {
                    *seed = tup[0] + (*seed - tup[1]);
                    break;
                }
            }
        }
        // println!("{}\n", seed);
    }

    println!("{}", seeds.iter().min().unwrap());
}

fn part2(sections: &Vec<&str>) {
    let seeds_str: Vec<&str> = sections[0].split(": ").collect();
    let seeds_str_vec: Vec<&str> = seeds_str[1].split(" ").collect();
    let seeds_i64: Vec<i64> = seeds_str_vec.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    let mut seed_ranges: Vec<Range<i64>> = Vec::new();

    for i in (0..seeds_i64.len()).step_by(2) {
        seed_ranges.push(seeds_i64[i]..seeds_i64[i]+seeds_i64[i+1]);
    }

    // println!("{}", seed_ranges.len());

    let mut transformations: Vec<Vec<Vec<i64>>> = Vec::new();

    // Linear instead of n^2
    for i in 1..sections.len() {
        let section_str: Vec<&str> = sections[i].split(":").collect();
        let mut ranges_vec_str: Vec<&str> = section_str[1].split("\n").collect();
        ranges_vec_str.retain(|x: &&str| *x != "");
        let mut ranges: Vec<Vec<i64>> = Vec::new();
    
        for num_str in ranges_vec_str {
            let section_str_vec: Vec<&str> = num_str.split(" ").collect();
            let nums: Vec<i64> = section_str_vec.iter().map(|s| s.parse::<i64>().unwrap()).collect();

            ranges.push(nums);
        }
        
        transformations.push(ranges);
    
    }

    let mut min_answers: Vec<i64> = Vec::new();

    // seed_range = input seeds
    for seed_range in &seed_ranges {
        // This will be the seed range to check for each transformation.
        // e.g. Starting with one range, when this iteration concludes,
        // seed_ranges will have all of the ranges.
        let mut final_seed_ranges: Vec<Range<i64>> = Vec::new();
        final_seed_ranges.push(seed_range.clone());

        // Transformation section, e.g. seed to soil.
        for trans_ranges in &transformations {
            // println!("\nTransformations: {:?}", trans_ranges);
            // New vector to keep the seed ranges we are working with
            let mut src_ranges: Vec<Range<i64>> = final_seed_ranges.clone();
            let mut dst_ranges: Vec<Range<i64>> = Vec::new();

            // println!("Seeds: {:?}", src_ranges);
            for trans_range in trans_ranges {
                // trans_range = transformation range
                let curr_trans_range: Range<i64> = trans_range[1]..trans_range[1]+trans_range[2];
                let new_trans_range: Range<i64> = trans_range[0]..trans_range[0]+trans_range[2];

                let (curr_trans_range_min, curr_trans_range_max) =
                    (curr_trans_range.start, curr_trans_range.end);

                // For each src_range, see if it intersects with trans_range.
                // If so, take out the part that is in that range and add it to dst_ranges.
                // Additionally, keep the part that is not in that range and append to src_ranges
                // in case the next transformation in this set contains this range.

                let mut keep_ranges: Vec<Range<i64>> = Vec::new();

                // Remove all ranges that are in the current set of transformation ranges
                src_ranges.retain(|src_range| {
                    let (src_range_min, src_range_max) =
                        (src_range.start, src_range.end);
                    
                    if src_range_min == src_range_max {
                        // println!("NO NO NO NO");
                        return false;
                    }

                    // src_range is in curr_trans_range_min
                    if src_range_min > curr_trans_range_min {
                        if src_range_max <= curr_trans_range_max {
                            // Entire src range is in the transformation
                            let min_diff = src_range_min - curr_trans_range_min;
                            let max_diff = curr_trans_range_max - src_range_max;

                            let temp_min = new_trans_range.start + min_diff;
                            let temp_max = new_trans_range.end - max_diff;
                            let temp = temp_min..temp_max;
                            // println!("Fully contained: {:?}", temp);
                            // if temp_min >= temp_max {
                            //     println!("NO NO NO NO");
                            // }
                            dst_ranges.push(temp);

                            return false;
                        } else { // src_range_max > curr_trans_range_max

                            // not in range at all
                            if curr_trans_range_max < src_range_min {
                                return true;
                            }

                            // src_range_min is in transformation, src_range_max is not
                            let min_diff = src_range_min - curr_trans_range_min;

                            let temp_min = new_trans_range.start + min_diff;
                            let temp_max = new_trans_range.end;
                            let temp = temp_min..temp_max;
                            // println!("Min contained, max not: {:?}", temp);
                            // if temp_min >= temp_max {
                            //     println!("NO NO NO NO");
                            // }
                            dst_ranges.push(temp);

                            let temp2_min = curr_trans_range_max;
                            let temp2_max = src_range_max;
                            let temp2 = temp2_min..temp2_max;
                            // println!("Keeping: {:?}", temp2);
                            // keep_ranges.push(temp2);
                            // if temp2_min >= temp2_max {
                            //     println!("NO NO NO NO");
                            // }
                            return false;
                        }
                    } else { // src_range_min < curr_trans_range_min
                        if src_range_max > curr_trans_range_min {

                            // src_range_max is in transformation, src_range_min is not
                            let max_diff = curr_trans_range_max - src_range_max;

                            let temp_min = new_trans_range.start;
                            let temp_max = new_trans_range.end - max_diff;
                            let temp = temp_min..temp_max;
                            // println!("Max contained, min not: {:?}", temp);
                            // if temp_min >= temp_max {
                            //     println!("NO NO NO NO");
                            // }
                            dst_ranges.push(temp);

                            let temp2_min = src_range_min;
                            let temp2_max = curr_trans_range_min;
                            let temp2 = temp2_min..temp2_max;
                            // println!("Keeping: {:?}", temp2);
                            // if temp2_min >= temp2_max {
                            //     println!("NO NO NO NO");
                            // }
                            keep_ranges.push(temp2);
                            return false;
                        } else { // src_range_max <= curr_trans_range_min
                            // src range not in trans range at all. return true
                            return true;
                        }
                    }
                });

                for range in keep_ranges {
                    src_ranges.push(range);
                }
            }

            // This section of transformations has concluded,
            // so the new set of ranges to look through can be updated.
            final_seed_ranges = dst_ranges;
            for range in src_ranges {
                final_seed_ranges.push(range);
            }
        }

        // Transformations have concluded. Give the min of all the ranges left.
        min_answers.push(final_seed_ranges.iter().map(|x:&Range<i64>| x.start).min().unwrap());
    }

    println!("{:?}", min_answers.iter().min().unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let mut data = fs::read_to_string(file_name).expect("Unable to read file");
    data = data.replace("\r", "");
    let mut sections:Vec<&str> = data.split("\n\n").collect();
    sections = sections.iter().map(|s| s.trim()).collect();
    sections.retain(|x: &&str| *x != "");

    part1(&sections);
    part2(&sections);
}
