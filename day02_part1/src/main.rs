use std::fs::File;
use std::io::prelude::*;



fn read_input(file_path: &str)-> String{
    let mut contents: String = String::new();
    let mut file: File = match File::open(file_path) {
        Err(why) => panic!("couldn't open the input file: {}", why),
        Ok(file) => file,
    };
    
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read the input file: {}", why),
        Ok(_) => println!("file content read correctly"),
    }
    contents
}

fn evaluate_instruction(mut current_position: Vec<i32>, instruction: &Vec<String>) -> Vec<i32>{
    let mut position = current_position;
    if instruction.len() == 2 {
        match &instruction[0][..] {
            "forward" => {
                position[0] = position[0] + instruction[1].parse::<i32>().unwrap();
            },
            "up" => {
                position[1] = position[1] - instruction[1].parse::<i32>().unwrap()
            },
            "down" => {
                position[1] = position[1] + instruction[1].parse::<i32>().unwrap()
            },
            _ => println!("ERROR"),
        };
    }
    position
}

fn parse_input_array(text: String) -> Vec<Vec<String>> {
    let split = text.split("\n");
    let mut vecs:  Vec<Vec<String>> = vec![];
    for s in split {
        let sp = s.split(" ");
        vecs.push( sp.map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect());
    }
    vecs
}

fn main() {
    let content = read_input("input.txt");
    let input_array = parse_input_array(content);
    let mut current_position = vec![0, 0];

    for index in  0..input_array.len() {
        current_position = evaluate_instruction(current_position, &input_array[index]);
    }
    println!("{:?}", current_position[0]*current_position[1]);
}
