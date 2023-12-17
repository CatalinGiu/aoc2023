use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

mod day1;
const INPUT_DIR: &str = "inputs/";

fn main() {
    let input_file = INPUT_DIR.to_owned() + "aoc_01_1.txt";
    println!("reading file {}", input_file);

    let lines = read_lines(input_file).expect("Could not read file {input_file}");
    println!(
        "{}",
        day1::aoc_01_part1(lines).expect("Error processing the input")
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
