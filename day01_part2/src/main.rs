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

fn get_collection_sum(depths: &mut Vec<i32>, index: usize, size: usize) -> i32 {
    let mut total: i32 = 0;
    for i in  index-size..index {
        total = total + depths[i];
    }
    total
}

fn is_collection_greater_than_previous(depths: &mut Vec<i32>, index: usize, size: usize) -> bool {
    if index <= size {
        return false
    } else {
        return get_collection_sum(depths, index, size) > get_collection_sum(depths, index-1, size)
    }
}

fn count_increased_depths(depths: & mut Vec<i32>) -> i32 {
    let mut increased_depths = 0;
    for index in  0..depths.len()+1 {
        if is_collection_greater_than_previous(depths, index, 3) {
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
