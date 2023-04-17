use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut stack= Vec::new();
    if arguments.len() != 2 {
        println!("Usage: findText inputString inputFile");
    } else {
        let file = File::open(arguments[1].as_str()).unwrap();
        let buffer = BufReader::new(file);
        for line in buffer.lines()  {
            stack.push(line.unwrap());
        }
        if arguments[0] == "-r" {
            //Taken from: https://stackoverflow.com/questions/55788725/how-do-i-sort-a-vector-of-strings-alphabetically
            stack.sort_by(|a, b| b.to_lowercase().cmp(&a.to_lowercase()));
        } else {
            //Taken from: https://stackoverflow.com/questions/55788725/how-do-i-sort-a-vector-of-strings-alphabetically
            stack.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        }
        for i in 0..stack.len() {
            println!("{}", stack[i]);
        }
    }
}