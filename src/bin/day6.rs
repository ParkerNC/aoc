use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){

    let mut input: Vec<String> = Vec::new();
    //read in file
    if let Ok(lines) = read_lines("./dat/input_day6.txt") {
        for line in lines{
            if let Ok(ranges) = line {
                input.push(ranges);
            }
        }
    }
    else{
        println!("bad file");
    }

    let pairs = make_pairs(input);
}

fn make_pairs(input: Vec<String>) -> Vec<(i32, i32)>{

    let ret: Vec<(i32, i32)> = Vec::new();


    ret
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}