use std::{fs::File, io::{BufReader, BufRead}};


fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 2 {
        println!("Usage: findText inputString inputFile");
    } else {
        let file = File::open(arguments[1].as_str())?;
        let buffer = BufReader::new(file);
        for line in buffer.lines()  {
            let holder: String = line?;
            if holder.contains(arguments[0].as_str()) {
                println!("{}", holder);
            }
        }
    }
    Ok(())
}