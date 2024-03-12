use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
    process::exit,
    time::Instant,
};

mod day1;
mod day2;
mod day3;
mod day4;
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
        "day3" => {
            let mut lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part1: {}",
                day3::aoc_03_part1(lines).expect("Error processing the input")
            );

            lines = read_lines(&input_file).expect("Could not read file");
            let mut start_time = Instant::now();
            println!(
                "part2: {}",
                day3::aoc_03_part2(lines).expect("Error processing the input")
            );
            let mut end_time = Instant::now();
            println!("(time: {:?}))", end_time - start_time);

            lines = read_lines(&input_file).expect("Could not read file");
            start_time = Instant::now();
            println!(
                "part2 data structures: {}",
                day3::aoc_03_part2_structs(lines).expect("Error processing the input")
            );
            end_time = Instant::now();
            println!("(time: {:?}))", end_time - start_time);
        }
        "day4" => {
            let mut lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part1: {}",
                day4::aoc_04_part1(lines).expect("Error processing the input")
            );

            lines = read_lines(&input_file).expect("Could not read file");
            println!(
                "part2: {}",
                day4::aoc_04_part2(lines).expect("Error processing the input")
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
