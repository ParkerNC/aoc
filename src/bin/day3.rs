use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./dat/input_day3.txt") {

        let mut graph: Vec<Vec<char>> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(elements) = line {
                let row: Vec<char> = elements.chars().collect();
                graph.push(row);
            }
        }
        let sum: u64 = adjacencies(graph);
        print!("final sum: {sum}\n")
    }
}

fn check_adj(x: usize, y: usize, graph: &Vec<Vec<char>>) -> bool {
    if x >= graph.len(){
        return false
    }
    else if y >= graph[x].len(){
        return false
    }
    else if !(graph[x][y].is_digit(10)) && graph[x][y] != '.'{
        return true
    }
    false
}

fn adjacencies(graph: Vec<Vec<char>>) -> u64{
    let mut chars: Vec<char> = Vec::new();

    let mut sum: u64 = 0;
    let mut machine: bool = false;

    for i in 0..graph.len(){
        for j in 0..graph[i].len(){
            if graph[i][j].is_digit(10){
                chars.push(graph[i][j]);
                if !(machine){
                    if check_adj(i, j+1, &graph){
                        machine = true;
                        continue;
                    }
                    if j != 0 {
                        if check_adj(i, j-1, &graph){
                            machine = true;
                            continue;
                        }
                        if check_adj(i+1, j-1, &graph){
                            machine = true;
                            continue;
                        }
                    }
                    if i != 0 {
                        if check_adj(i-1, j, &graph){
                            machine = true;
                            continue;
                        }
                        if check_adj(i-1, j+1, &graph){
                            machine = true;
                            continue;
                        }
                    }
                    if i != 0 && j != 0{
                        if check_adj(i-1, j-1, &graph){
                            machine = true;
                            continue;
                        }
                    }
                    if check_adj(i+1, j, &graph){
                        machine = true;
                        continue;
                    }

                    if check_adj(i+1, j+1, &graph){
                        machine = true;
                        continue;
                    }

                }

            }
            else{
                if machine {
                    let num_str: String = chars.iter().collect();
                    machine = false;
                    let input: u64 = num_str
                    .trim()
                    .parse()
                    .expect("Wanted a number");
                    sum += input;
                    print!("Input {input}, sum {sum}\n")
                }
                chars = Vec::new();
            }
        }
    }

    sum

}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
