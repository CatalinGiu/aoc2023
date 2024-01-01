use std::{fs::File, io, usize};

pub fn aoc_03_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut top;
    let mut current = "".to_string();
    let mut bottom = "".to_string();
    let mut total = 0;

    for line in lines {
        top = current;
        current = bottom;
        bottom = line.expect("oh no");
        // println!("\n\nt: {top}\nc: {current}\nb: {bottom}");
        match process_current(&top, &current, &bottom) {
            Some(res) => total += res,
            None => continue,
        }
    }
    top = current;
    current = bottom;
    bottom = "".to_string();
    // println!("\n\nt: {top}\nc: {current}\nb: {bottom}");
    total += process_current(&top, &current, &bottom).unwrap();

    Ok(total)
}

fn process_current(top: &String, current: &String, bottom: &String) -> Option<usize> {
    if current.len() == 0 {
        return None;
    }

    let mut num_start = 0;
    let mut num_end;
    let mut add_number = false;
    let mut new_number = false;
    let mut total = 0;
    for ch in current.chars().enumerate() {
        if ch.1.is_ascii_digit() {
            if !new_number {
                num_start = ch.0;
                new_number = true;
            }
            if !add_number && is_symbol_nearby(ch.0, &top, &current, &bottom) {
                add_number = true
            }
        } else {
            if add_number {
                num_end = ch.0;
                let num = current[num_start..num_end]
                    .parse::<usize>()
                    .expect("Error parsing int");
                // println!("{num}");
                total += num;
            }
            add_number = false;
            new_number = false;
        }
    }

    // we might still have a number in buffer
    if add_number {
        num_end = current.len();
        let num = current[num_start..num_end]
            .parse::<usize>()
            .expect("Error parsing int");
        // println!("{num}");
        total += num;
    }
    Some(total)
}

fn is_symbol_nearby(index: usize, top: &String, current: &String, bottom: &String) -> bool {
    // println!("--- {} ---", current.chars().nth(index).unwrap());
    let index_start: usize = if index > 0 { index - 1 } else { index };
    let index_end: usize = if index < current.len() - 1 {
        index + 1
    } else {
        index
    };

    if is_special_character(current.chars().nth(index_start).unwrap())
        || is_special_character(current.chars().nth(index_end).unwrap())
    {
        return true;
    }

    if top.len() > 0
        && top[index_start..index_end + 1]
            .chars()
            .any(|c| is_special_character(c))
    {
        return true;
    }

    if bottom.len() > 0
        && bottom[index_start..index_end + 1]
            .chars()
            .any(|c| is_special_character(c))
    {
        return true;
    }

    false
}

fn is_special_character(ch: char) -> bool {
    (ch != '.') && (!ch.is_ascii_alphanumeric())
}
