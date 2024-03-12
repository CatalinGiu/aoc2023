use std::{fs::File, io, usize};

fn partition(arr: &mut [u32]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index];

    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn quicksort(arr: &mut [u32]) {
    let len = arr.len();
    if len < 2 {
        // nothing to sort
        return;
    }

    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..len]);
}

pub fn aoc_04_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut score = 0;
    for line in lines {
        let mut start_winning_numbers = 0;
        let mut start_elf_numbers = 0;
        let line = line.expect("no line??");
        if let Some(index) = line.find(":") {
            start_winning_numbers = index + 1
        }

        if let Some(index) = line.find("|") {
            start_elf_numbers = index + 1
        }

        let mut winning_numbers = str_to_vec(&line[start_winning_numbers..start_elf_numbers]);
        quicksort(&mut winning_numbers);
        let mut elf_numbers = str_to_vec(&line[start_elf_numbers..line.len()]);
        quicksort(&mut elf_numbers);

        let mut winner_ix = 0;
        let mut e_ix = 0;
        let mut curret_score = 0;
        loop {
            if winner_ix == winning_numbers.len() || e_ix == elf_numbers.len() {
                break;
            }
            let w_num = winning_numbers[winner_ix];
            let e_num = elf_numbers[e_ix];
            if w_num == e_num {
                if curret_score == 0 {
                    curret_score = 1
                } else {
                    curret_score *= 2
                }
                winner_ix += 1;
                e_ix += 1;
            } else if e_num < w_num {
                e_ix += 1;
            } else {
                winner_ix += 1;
            }
        }
        score += curret_score;
    }

    Ok(score)
}

#[derive(Debug)]
struct Scratchpads {
    cards: Vec<u32>,
}

impl Scratchpads {
    fn from_lines(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut cards: Vec<u32> = vec![0; 203];

        for (ix, line) in lines.enumerate() {
            let mut start_winning_numbers = 0;
            let mut start_elf_numbers = 0;
            let line = line.expect("no line??");
            cards[ix] += 1;

            if let Some(index) = line.find(":") {
                start_winning_numbers = index + 1
            }

            if let Some(index) = line.find("|") {
                start_elf_numbers = index + 1
            }

            let mut winning_numbers = str_to_vec(&line[start_winning_numbers..start_elf_numbers]);
            quicksort(&mut winning_numbers);
            let mut elf_numbers = str_to_vec(&line[start_elf_numbers..line.len()]);
            quicksort(&mut elf_numbers);

            let mut winner_ix = 0;
            let mut e_ix = 0;
            let mut matches = 0;
            loop {
                if winner_ix == winning_numbers.len() || e_ix == elf_numbers.len() {
                    break;
                }
                let w_num = winning_numbers[winner_ix];
                let e_num = elf_numbers[e_ix];
                if w_num == e_num {
                    matches += 1;
                    winner_ix += 1;
                    e_ix += 1;
                } else if e_num < w_num {
                    e_ix += 1;
                } else {
                    winner_ix += 1;
                }
            }

            let current_cards = cards[ix];
            for i in ix + 1..=ix + matches {
                cards[i] += current_cards
            }
        }
        Scratchpads { cards }
    }
}

pub fn aoc_04_part2(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let scratchpad = Scratchpads::from_lines(lines);
    Ok(scratchpad.cards.iter().sum())
}
fn str_to_vec(str: &str) -> Vec<u32> {
    let mut arr = Vec::new();
    let mut num = 0;
    for ch in str.chars() {
        if let Some(digit) = ch.to_digit(10) {
            if num == 0 {
                num = digit
            } else {
                num = num * 10 + digit
            }
        } else if num != 0 {
            arr.push(num);
            num = 0;
        }
    }

    if num != 0 {
        arr.push(num);
    }

    arr
}

#[cfg(test)]
mod tests {
    use crate::day4::quicksort;

    #[test]
    fn test_quicksort() {
        let mut arr = [83, 86, 6, 31, 17, 9, 48, 53].to_vec();
        quicksort(&mut arr);
        assert_eq!(arr, [6, 9, 17, 31, 48, 53, 83, 86])
    }
}
