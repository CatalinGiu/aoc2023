use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
    process::exit,
};

mod day1;
mod day2;
const INPUT_DIR: &str = "inputs/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let selected = &args[1];
    let input_file = INPUT_DIR.to_owned() + selected + ".txt";

    println!("reading file {}", input_file);
    match selected.as_str() {
        "day1" => {
            let mut lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part1: {}",
                day1::aoc_01_part1(lines).expect("Error processing the input")
            );

            lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part2: {}",
                day1::aoc_01_part2(lines).expect("Error processing the input")
            );
        }
        "day2" => {
            let mut lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part1: {}",
                day2::aoc_02_part1(lines).expect("Error processing the input")
            );

            lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part2: {}",
                day2::aoc_02_part2(lines).expect("Error processing the input")
            );
        }
        &_ => exit(1),
    }
}

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
