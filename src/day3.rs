use std::{char, fs::File, io, u32, usize};

#[derive(Debug)]
struct Number {
    val: u32,
    pos: (usize, usize),
}

#[derive(Debug)]
struct Symbol {
    c: char,
    pos: (usize, usize),
    connections: u32,
    val: u32,
}

impl Number {
    fn get_nerby_boundaries(&self) -> ((usize, usize), (usize, usize)) {
        let str_val: &str = &self.val.to_string();
        let start_y = if self.pos.0 == 0 { 0 } else { self.pos.0 - 1 };
        let end_y = self.pos.0 + 1;
        let start_x = if self.pos.1 == 0 { 0 } else { self.pos.1 - 1 };
        let end_x = self.pos.1 + str_val.len();

        ((start_y, end_y), (start_x, end_x))
    }
}

#[derive(Debug)]
struct EngineMap {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl EngineMap {
    fn gear_ratio(&mut self) -> u32 {
        for num in &self.numbers {
            let (y_boundaries, x_boundaries) = num.get_nerby_boundaries();
            // println!("{y_boundaries:?}    {x_boundaries:?}");
            for y_pos in y_boundaries.0..=y_boundaries.1 {
                for x_pos in x_boundaries.0..=x_boundaries.1 {
                    let pos = (y_pos, x_pos);
                    if let Some(symbol) = self.symbols.iter_mut().find(|s| (*s).pos == pos) {
                        if symbol.c == '*' {
                            if symbol.connections == 0 {
                                symbol.val = num.val
                            } else {
                                symbol.val *= num.val
                            }
                            symbol.connections += 1;
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        for symbol in &self.symbols {
            if symbol.connections == 2 {
                sum += symbol.val
            }
        }

        sum
    }

    fn from_str(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut numbers: Vec<Number> = Vec::new();
        let mut symbols: Vec<Symbol> = Vec::new();

        for (iy, line) in lines.enumerate() {
            let mut num_start: Option<(usize, usize)> = None;
            let mut num: u32 = 0;
            for (ix, char) in line.expect("no line??").chars().enumerate() {
                if let Some(digit) = char.to_digit(10) {
                    if num_start.is_some() {
                        num = num * 10 + digit
                    } else {
                        num_start = Some((iy, ix));
                        num = digit;
                    }
                } else {
                    if let Some(pos) = num_start {
                        numbers.push(Number { val: num, pos });
                        num = 0;
                        num_start = None;
                    }
                    if is_special_character(char) {
                        symbols.push(Symbol {
                            c: char,
                            pos: (iy, ix),
                            val: 0,
                            connections: 0,
                        });
                    }
                }
            }

            if let Some(pos) = num_start {
                numbers.push(Number { val: num, pos });
            }
        }

        EngineMap { numbers, symbols }
    }
}

pub fn aoc_03_part2_structs(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut map = EngineMap::from_str(lines);
    // println!("{map:?}");
    let gear_ration = map.gear_ratio();
    // println!("{map:?}");
    Ok(gear_ration.try_into().unwrap())
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_tuples_eq() {
        assert_eq!((1, 1), (1, 1));
        assert_ne!((1, 2), (1, 1))
    }
}
