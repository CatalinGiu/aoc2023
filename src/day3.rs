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
        match process_current_part1(&top, &current, &bottom) {
            Some(res) => total += res,
            None => continue,
        }
    }
    top = current;
    current = bottom;
    bottom = "".to_string();
    total += process_current_part1(&top, &current, &bottom).unwrap();

    Ok(total)
}

fn process_current_part1(top: &String, current: &String, bottom: &String) -> Option<usize> {
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

pub fn aoc_03_part2(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut top;
    let mut current = "".to_string();
    let mut bottom = "".to_string();
    let mut total = 0;

    for line in lines {
        top = current;
        current = bottom;
        bottom = line.expect("oh no");
        // println!("\n\nt: {top}\nc: {current}\nb: {bottom}");
        match process_current_part2(&top, &current, &bottom) {
            Some(res) => total += res,
            None => continue,
        }
    }
    top = current;
    current = bottom;
    bottom = "".to_string();
    total += process_current_part2(&top, &current, &bottom).unwrap();

    Ok(total)
}

fn process_current_part2(top: &String, current: &String, bottom: &String) -> Option<usize> {
    if current.len() == 0 {
        return None;
    }

    let mut total = 0;
    for ch in current.chars().enumerate() {
        if ch.1 == '*' {
            total += line_value(ch.0, &top, &current, &bottom)
        }
    }

    Some(total)
}

fn line_value(index: usize, top: &String, current: &String, bottom: &String) -> usize {
    let mut first: Option<usize> = Default::default();
    let mut second: Option<usize> = Default::default();

    let index_start: usize = if index > 0 { index - 1 } else { index };

    let index_end: usize = if index < current.len() - 1 {
        index + 1
    } else {
        index
    };

    for line in [top, current, bottom] {
        for ch in line[index_start..index_end + 1].chars().enumerate() {
            if ch.1.is_numeric()
                && ((line.len() - 1 == index_start + ch.0
                    || !line
                        .chars()
                        .nth(index_start + ch.0 + 1)
                        .unwrap()
                        .is_numeric())
                    || ch.0 == index_end - index_start)
            {
                let val = get_number_from_end_index(index_start + ch.0, line);
                if second.is_some() {
                    // not a gear
                    return 0;
                }

                if first.is_some() {
                    second = val;
                } else {
                    first = val;
                }
            }
        }
    }

    if first.is_some() && second.is_some() {
        return first.unwrap() * second.unwrap();
    }

    return 0;
}

fn get_number_from_end_index(mut index: usize, line: &String) -> Option<usize> {
    loop {
        if index == line.len() || !line.chars().nth(index).unwrap().is_numeric() {
            break;
        }
        index += 1;
    }

    let mut num_size = 0;
    loop {
        if index - num_size == 0 || !line.chars().nth(index - num_size - 1).unwrap().is_numeric() {
            break;
        }
        num_size += 1;
    }

    Some(
        line[index - num_size..index]
            .parse::<usize>()
            .expect("Error parsing int"),
    )
}

fn is_symbol_nearby(index: usize, top: &String, current: &String, bottom: &String) -> bool {
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
