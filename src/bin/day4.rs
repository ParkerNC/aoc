use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main(){
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./dat/input_day4.txt") {


        let mut factors: VecDeque<u32> = VecDeque::new();
        factors.push_back(1);
        let mut cards: u32 = 0;

        for line in lines {
            if let Ok(card) = line {
                let values = card.split(":");
                if let Some(value) = values.last(){
                    let mut card_points: u32 = 0;
                    let numbers = value.split("|");
                    let halves: Vec<&str> = numbers.collect();

                    let win_nums = halves[0].split_whitespace();
                    let have_nums = halves[1].split_whitespace();

                    let wins: Vec<&str> = win_nums.collect();
                    let haves: Vec<&str> = have_nums.collect();
                    
                    for num in haves{
                        if wins.contains(&num){
                            card_points +=1
                        }
                    }

                    let factor: u32;
                    
                    match factors.pop_front(){
                        None => {
                            factor = 1;
                            cards += 1;
                        },
                        Some(f) => {
                            cards += f;
                            factor = f;
                        }
                    }

                    for i in 0..card_points{
                        if i < factors.len().try_into().unwrap(){
                            factors[i.try_into().unwrap()] += factor 
                        }
                        else{
                            factors.push_back(1 + factor)
                        }

                    }                    
                }
            }
        }
        print!("{cards}\n")
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}