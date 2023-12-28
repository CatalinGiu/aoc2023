use std::{fs::File, i32, io, str::FromStr, usize};

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

pub fn aoc_02_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut result: i32 = 0;

    for line in lines {
        let line = line?;
        let mut game_is_valid = true;
        // println!("{}", line);

        let start_index = line
            .chars()
            .position(|c| c.is_digit(10))
            .expect("Game ID has unexpected format");
        let id: String = line
            .chars()
            .skip(start_index)
            .take_while(|c| c.is_digit(10))
            .collect();

        // +2 to skip semicolon and space
        let games = &line[start_index + id.len() + 2..];

        for hand in games.split(|c| c == ',' || c == ';') {
            let hand = hand.trim_start();
            let mut hand_iter = hand.split_whitespace();
            let number = hand_iter
                .next()
                .expect("wrong number format")
                .parse::<i32>()
                .unwrap();
            let colour: Color = hand_iter
                .next()
                .expect("wrong colour format")
                .parse()
                .unwrap();

            match colour {
                Color::Red => {
                    if number > MAX_RED {
                        game_is_valid = false;
                        break;
                    }
                }
                Color::Blue => {
                    if number > MAX_BLUE {
                        game_is_valid = false;
                        break;
                    }
                }
                Color::Green => {
                    if number > MAX_GREEN {
                        game_is_valid = false;
                        break;
                    }
                }
            }
        }

        if game_is_valid {
            result += id.as_str().parse::<i32>().unwrap();
        } else {
            println!("{games}");
        }
    }
    Ok(result as usize)
}

pub fn aoc_02_part2(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut result: i32 = 0;

    for line in lines {
        let line = line?;
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        let start_index = line
            .chars()
            .position(|c| c.is_digit(10))
            .expect("Game ID has unexpected format");
        let id: String = line
            .chars()
            .skip(start_index)
            .take_while(|c| c.is_digit(10))
            .collect();

        // +2 to skip semicolon and space
        let games = &line[start_index + id.len() + 2..];

        for hand in games.split(|c| c == ',' || c == ';') {
            let hand = hand.trim_start();
            let mut hand_iter = hand.split_whitespace();
            let number = hand_iter
                .next()
                .expect("wrong number format")
                .parse::<i32>()
                .unwrap();
            let colour: Color = hand_iter
                .next()
                .expect("wrong colour format")
                .parse()
                .unwrap();

            match colour {
                Color::Red => max_red = max_red.max(number),
                Color::Blue => max_blue = max_blue.max(number),
                Color::Green => max_green = max_green.max(number),
            }
        }

        result += max_green * max_red * max_blue;
    }
    Ok(result as usize)
}
