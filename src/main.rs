use std::env;
use std::fs;

fn main() {
    println!();
    let args: &Vec<String> = &env::args().collect::<Vec<String>>();
    let contents = fs::read_to_string(&args[1]).expect("File should be read fine.");
    println!("File contents: {}", contents);

    let lines = contents.lines();

    for line in lines {
        println!("\nLine: {}\n", line);
        let tokens = line.split(" ");
        for token in tokens {
            println!("{}", token);
        }
    }
}
