use lazy_static::lazy_static;
use std::cmp;
use std::iter::zip;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 2A                                           */
/*                                                                                         */
/*******************************************************************************************/

lazy_static! {
    static ref COLOR_LIMITS: HashMap<String, usize> = {
        let mut m = HashMap::new();
        m.insert("red".to_string(), 12);
        m.insert("green".to_string(), 13);
        m.insert("blue".to_string(), 14);
        m
    };
}

fn check_game(s: &str) -> bool {
    s.split(',')
        .map(|r| {
            let mut items = r.trim().split(' ');
            let count = items.next().unwrap().parse::<usize>().unwrap();
            let color = items.next().unwrap();
            &count <= COLOR_LIMITS.get(&color.to_string()).unwrap()
        })
        .reduce(|acc, e| acc && e)
        .unwrap()
}

fn check_line(s: &str) -> Option<usize> {
    let mut split_str = s.split(':');
    let game_info = split_str
        .next()
        .expect("Unable to get Game Info from line.");
    for game in split_str
        .next()
        .expect("Unable to get Game Items from line.")
        .split(';')
    {
        if !check_game(game) {
            return None;
        }
    }
    Some(
        game_info
            .split(' ')
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    )
}

pub fn day_2a() {
    let file: File = File::open(".\\src\\day2\\day-2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let total = lines
        .filter_map(|line| check_line(&line.unwrap()))
        .reduce(|acc, e| acc + e);
    println!("Day 2a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 2B                                           */
/*                                                                                         */
/*******************************************************************************************/

lazy_static! {
    static ref COLOR_INDICES: HashMap<String, usize> = {
        let mut m = HashMap::new();
        m.insert("red".to_string(), 0);
        m.insert("green".to_string(), 1);
        m.insert("blue".to_string(), 2);
        m
    };
}

fn color_counts(s: &str) -> Vec<usize> {
    let mut counts = vec![0; 3];
    for item in s.split(',') {
        let mut items = item.trim().split(' ');
        let count = items.next().unwrap().parse::<usize>().unwrap();
        let color = items.next().unwrap();
        if let Some(i) = COLOR_INDICES.get(color) {
            counts[*i] = count;
        }
    }
    counts
}

fn get_line_power(s: &str) -> usize {
    let mut split_str = s.split(':').skip(1);
    let max_counts = split_str
        .next()
        .unwrap()
        .split(';')
        .map(color_counts)
        .reduce(|acc, e| {
            zip(acc, e)
                .map(|(a, b)| cmp::max(a, b))
                .collect::<Vec<usize>>()
        })
        .unwrap();
    max_counts.into_iter().reduce(|acc, e| acc * e).unwrap()
}

pub fn day_2b() {
    let file: File = File::open(".\\src\\day2\\day-2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let total = lines
        .map(|line| get_line_power(&line.unwrap()))
        .reduce(|acc, e| acc + e);
    println!("Day 2b answer = {}", total.unwrap());
}
