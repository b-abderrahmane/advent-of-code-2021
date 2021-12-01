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

fn main() {
    let content = read_input("input.txt");
    let input_array = parse_input_array(content);
    println!("{:?}", input_array);
}
