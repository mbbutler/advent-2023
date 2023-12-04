use std::collections::HashSet;
use std::{
    fs::File,
    io::{self, BufRead},
};

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 4A                                           */
/*                                                                                         */
/*******************************************************************************************/

fn calc_game_points(line: &str) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut game = line.split(":").skip(1).next().unwrap().split('|');
    game.next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .for_each(|n| {
            set.insert(n);
        });
    let total = game
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .filter(|n| set.contains(n))
        .count();
    if total > 0 {
        2_usize.pow(total as u32 - 1)
    } else {
        0
    }
}

pub fn day_4a() {
    let file: File = File::open(".\\src\\day4\\day-4.txt").unwrap();
    let lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let total = lines.map(|l| calc_game_points(&l)).reduce(|acc, e| acc + e);

    println!("Day 4a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 4B                                           */
/*                                                                                         */
/*******************************************************************************************/

fn calc_game_wins(line: &str) -> usize {
    let mut set: HashSet<usize> = HashSet::new();
    let mut game = line.split(":").skip(1).next().unwrap().split('|');
    game.next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .for_each(|n| {
            set.insert(n);
        });
    game.next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .filter(|n| set.contains(n))
        .count()
}

pub fn day_4b() {
    let file: File = File::open(".\\src\\day4\\day-4.txt").unwrap();
    let lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let wins: Vec<usize> = lines.map(|l| calc_game_wins(&l)).collect();
    let mut cards = vec![1_usize; wins.len()];
    for (i, win) in wins.iter().enumerate() {
        for j in (i + 1)..=(i + win) {
            cards[j] += cards[i];
        }
    }
    let total = cards.into_iter().reduce(|acc, e| acc + e);
    println!("Day 4a answer = {}", total.unwrap());
}
