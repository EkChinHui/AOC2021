use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    sol_3a();
    sol_3b();
}

fn sol_3a() {
    let file = File::open("input.txt").unwrap();
    let mut lines = 0;
    let mut count_arr: [i32; 12] = [0;12];
    // let mut bin_str: &[u8];
    
    for line in BufReader::new(file).lines() {
        lines += 1;
        let bin_str = line.unwrap();
        let bytes_str = bin_str.as_bytes();
        for i in 0..12 {
            count_arr[i] = count_arr[i] + (bytes_str[i] - 48) as i32;
        }
    }
    // print!("Result arr: ");
    // for element in count_arr {
    //     print!("{}, ", element);
    // }
    // println!("Total Lines: {}", lines);
    let mut binary_most: String = "".to_string();
    let mut binary_least: String = "".to_string();
    for element in count_arr {
        if element as i32 > lines/2 {
            binary_most += "1";
            binary_least += "0";
        } else {
            binary_most += "0";
            binary_least += "1";
        }
    }

    let bin_most = isize::from_str_radix(&binary_most, 2).unwrap();
    let bin_least = isize::from_str_radix(&binary_least, 2).unwrap();
    println!("Solution 3A: {}", bin_most * bin_least);
}

fn sol_3b() {
    let file = File::open("input.txt").unwrap();
    let mut lines = 0;
    // let mut input_txt: Vec<String> = vec![String::new(); 1000];
    let mut input_txt: [String; 1000] = ["".to_string(); 1000];
    // let mut count_arr: [i32; 12] = [0;12];
    // let mut bin_str: &[u8];
    // use some sort of filtering and filter those that starts with ...
    
    for line in BufReader::new(file).lines() {
        let bin_str = line.unwrap();
        input_txt[lines] = bin_str;
        lines += 1;
    }
    let mut oxygen_bin: String = "".to_string();
    let mut co2_bin: String = "".to_string();

    // oxygen rating
    for i in 0..12 {
        let mut count = 0;
        for j in 0..lines+1 {
            if input_txt[j] == "1" {
                count += 1;
            } else {
                lines -= 1;
            }

        }
        if count >= lines/2 {
            oxygen_bin += "1";
        } else {
            oxygen_bin += "0";
        }
        let input_iter = input_txt.into_iter();
        // input_iter.filter(|&x| x.starts_with(oxygen_bin));
        input_iter.filter(|x| x[..i+1]==oxygen_bin);

    }

    
    let int_oxygen = isize::from_str_radix(&oxygen_bin, 2).unwrap();
    let int_co2 = isize::from_str_radix(&co2_bin, 2).unwrap();
    
    println!("Solution 3B: {}", int_oxygen * int_co2);
}
