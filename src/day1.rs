use std::{fs::File, io, u32};

pub fn aoc_01_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let mut sum_calibration_values = 0;
    let mut first = 0;
    let mut last = 0;

    for line in lines {
        let line = line?;
        println!("{}", line);
        for c in line.chars() {
            let char: u32 = c as u32;
            if char <= 57 {
                first = char - 48;
                break;
            }
        }
        for c in line.chars().rev() {
            let char: u32 = c as u32;
            if char <= 57 {
                last = char - 48;
                break;
            }
        }
        println!("{}, {}", first, last);
        sum_calibration_values += first * 10 + last;
    }

    Ok(sum_calibration_values)
}
