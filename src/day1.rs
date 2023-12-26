use std::{fs::File, i32, io, u8, usize};

const NUMBERS: [(&str, i32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

#[derive(Default, Debug)]
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    value: Option<i32>,
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    // create Trie
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str, value: i32) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            let index = (ch as u8 - b'0') as usize;

            if current.children.len() <= index {
                current.children.resize_with(index + 1, || None);
            }
            current = current.children[index].get_or_insert(Box::new(TrieNode::default()));
        }

        current.value = Some(value);
    }

    pub fn contains<I>(&self, chars: I) -> Option<i32>
    where
        I: IntoIterator<Item = char>,
    {
        let mut current = &self.root;

        for ch in chars {
            let index = (ch as u8 - b'0') as usize;

            match current.children.get(index) {
                Some(child) => match child {
                    Some(c) => {
                        current = c;
                        if let Some(value) = current.value {
                            return Some(value);
                        }
                        continue;
                    }
                    None => {
                        break;
                    }
                },
                None => {
                    break;
                }
            }
        }

        current.value
    }
}

fn reverse_string(input: &str) -> String {
    let reversed = input.chars().rev().collect();
    reversed
}

pub fn aoc_01_part1(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let mut sum_calibration_values = 0;
    let mut first = 0;
    let mut last = 0;

    for line in lines {
        let line = line?;
        // println!("{}", line);
        for c in line.chars() {
            if c <= '9' {
                first = c as usize - '0' as usize;
                break;
            }
        }
        for c in line.chars().rev() {
            if c <= '9' {
                last = c as usize - '0' as usize;
                break;
            }
        }
        // println!("{}, {}", first, last);
        sum_calibration_values += first * 10 + last;
    }

    Ok(sum_calibration_values)
}

pub fn aoc_01_part2(lines: io::Lines<io::BufReader<File>>) -> io::Result<i32> {
    let mut sum_calibration_values = 0;
    let mut first: i32;
    let mut last: i32;
    let mut trie_first = Trie::new();
    let mut trie_last = Trie::new();

    for str_num in NUMBERS {
        trie_first.insert(str_num.0, str_num.1);
        trie_last.insert(&reverse_string(str_num.0), str_num.1);
    }

    for line in lines {
        let line = line?;
        // println!("{}", line);
        let mut iter = line.chars();
        loop {
            match trie_first.contains(iter.clone()) {
                Some(value) => {
                    first = value;
                    break;
                }
                None => {
                    iter.next();
                    continue;
                }
            }
        }

        let mut rev_iter = line.chars().rev();
        loop {
            match trie_last.contains(rev_iter.clone()) {
                Some(value) => {
                    last = value;
                    break;
                }
                None => {
                    rev_iter.next();
                    continue;
                }
            }
        }
        // println!("{}", first * 10 + last);
        sum_calibration_values += first * 10 + last;
    }

    Ok(sum_calibration_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains1() {
        let mut trie = Trie::new();
        trie.insert("two", 2);
        assert_eq!(Some(2), trie.contains("two".chars()));
    }

    #[test]
    fn test_contains2() {
        let mut trie = Trie::new();
        trie.insert("two", 2);
        let ret = trie.contains("twa".chars());
        assert_eq!(ret, None)
    }

    #[test]
    fn test_contains3() {
        let mut trie_first = Trie::new();
        let mut trie_last = Trie::new();
        for str_numbers in NUMBERS {
            trie_first.insert(str_numbers.0, str_numbers.1);
            trie_last.insert(&reverse_string(str_numbers.0), str_numbers.1);
        }

        let first;
        let mut iter = "wowsqpfonezdtpqrfbhzgjmgv7".chars();

        loop {
            match trie_first.contains(iter.clone()) {
                Some(value) => {
                    first = value;
                    break;
                }
                None => {
                    iter.next();
                    continue;
                }
            }
        }
        assert_eq!(first, 1)
    }
}
