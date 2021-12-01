use std::fs::File;
use std::io::prelude::*;


fn read_input(file_path: &str)-> String{
    let mut contents = String::new();
    let mut file = match File::open(file_path) {
        Err(why) => panic!("couldn't open the input file: {}", why),
        Ok(file) => file,
    };
    
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read the input file: {}", why),
        Ok(_) => println!("file content read correctly"),
    }
    contents
}

fn parse_input_array(text: String) -> Vec<i32> {
    let split = text.split("\n");
    let vecs: Vec<i32> = split.map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect(); 
    vecs
}

fn is_greater_than_previous(depths: &mut Vec<i32>, index: usize) -> bool {
    if index == 0 {
        return false
    } else {
        return depths.get(index) > depths.get(index-1)
    }
}

fn count_increased_depths(depths: & mut Vec<i32>) -> i32 {
    let mut increased_depths = 0;
    for index in  0..depths.len() {
    
        if is_greater_than_previous(depths, index) {
            increased_depths = increased_depths + 1;
        }
    }
    
    increased_depths
}


fn main() {
    let content = read_input("input.txt");
    let mut input_array = parse_input_array(content);
    let increases = count_increased_depths(&mut input_array);
    println!("{:?}", increases);
}
