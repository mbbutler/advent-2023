use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 1A                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_1a() {
    let file: File = File::open(".\\src\\day1\\day-1.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok());

    let total = lines
        .map(|line| {
            let chars: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            10 * chars.first().unwrap().to_digit(10).unwrap()
                + chars.last().unwrap().to_digit(10).unwrap()
        })
        .reduce(|acc, e| acc + e);

    println!("Day 1a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 1B                                           */
/*                                                                                         */
/*******************************************************************************************/

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
    static ref WORDS_MAP: HashMap<String, usize> = {
        let mut m = HashMap::new();
        for (i, word) in NUMBER_WORDS.iter().enumerate() {
            m.insert(word.to_string(), i + 1);
        }
        m
    };
}

struct NumberFinder<'a> {
    index: usize,
    line: &'a str,
}

impl<'a> NumberFinder<'a> {
    fn new(line: &'a str) -> Self {
        NumberFinder { index: 0, line }
    }

    fn try_parse_char(&self) -> Option<usize> {
        self.line
            .get(self.index..(self.index + 1))?
            .parse::<usize>()
            .ok()
    }

    fn try_parse_word(&self) -> Option<&'static usize> {
        for word in NUMBER_WORDS {
            if self.line[self.index..].starts_with(word) {
                return WORDS_MAP.get(word);
            }
        }
        None
    }
}

impl Iterator for NumberFinder<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.line.len() {
            if let Some(val) = self.try_parse_char() {
                self.index += 1;
                return Some(val);
            }
            if let Some(val) = self.try_parse_word() {
                self.index += 1;
                return Some(*val);
            }
            self.index += 1;
        }
        None
    }
}

pub fn day_1b() {
    let file: File = File::open(".\\src\\day1\\day-1.txt").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok());

    let total = lines
        .map(|line| {
            let vals: Vec<usize> = NumberFinder::new(&line).collect();
            10 * vals.first().unwrap() + vals.last().unwrap()
        })
        .reduce(|acc, e| acc + e);

    println!("Day 1b answer = {}", total.unwrap());
}
