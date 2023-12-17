use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    iter::zip,
};

use lazy_static::lazy_static;

use itertools::Itertools;

/*******************************************************************************************/
/*                                                                                         */
/*                             Structs for both Questions                                  */
/*                                                                                         */
/*******************************************************************************************/

lazy_static! {
    static ref CARD_MAP: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('2', 2);
        m.insert('3', 3);
        m.insert('4', 4);
        m.insert('5', 5);
        m.insert('6', 6);
        m.insert('7', 7);
        m.insert('8', 8);
        m.insert('9', 9);
        m.insert('T', 10);
        m.insert('J', 11);
        m.insert('Q', 12);
        m.insert('K', 13);
        m.insert('A', 14);
        m
    };
}

lazy_static! {
    static ref OTHER_CARD_MAP: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('2', 2);
        m.insert('3', 3);
        m.insert('4', 4);
        m.insert('5', 5);
        m.insert('6', 6);
        m.insert('7', 7);
        m.insert('8', 8);
        m.insert('9', 9);
        m.insert('T', 10);
        m.insert('J', 1);
        m.insert('Q', 12);
        m.insert('K', 13);
        m.insert('A', 14);
        m
    };
}

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    counts: Vec<usize>,
    bid: usize,
}

impl Hand {
    fn new_7a(s: &str) -> Option<Self> {
        let mut items = s.split(' ');
        let cards = items
            .next()?
            .chars()
            .map(|c| CARD_MAP.get(&c).unwrap().to_owned())
            .collect();
        let bid = items.next()?.parse::<usize>().ok()?;
        let mut map: HashMap<&usize, usize> = HashMap::new();
        for c in &cards {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut counts = map.into_values().collect_vec();
        counts.sort_by(|a, b| b.cmp(a));

        Some(Self { cards, counts, bid })
    }

    fn new_7b(s: &str) -> Option<Self> {
        let mut items = s.split(' ');
        let cards = items
            .next()?
            .chars()
            .map(|c| OTHER_CARD_MAP.get(&c).unwrap().to_owned())
            .collect();
        let bid = items.next()?.parse::<usize>().ok()?;
        let mut map: HashMap<&usize, usize> = HashMap::new();
        let mut jokers = 0;
        for c in &cards {
            if c == &1 {
                jokers += 1;
            } else {
                map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        let mut counts = map.into_values().collect_vec();
        counts.sort_by(|a, b| b.cmp(a));
        if counts.is_empty() {
            counts = vec![jokers];
        } else {
            counts[0] += jokers;
        }

        Some(Self { cards, counts, bid })
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let len = cmp::min(self.counts.len(), other.counts.len());
        for i in 0..len {
            if self.counts[i] == other.counts[i] {
                continue;
            } else if self.counts[i] > other.counts[i] {
                return Some(Ordering::Greater);
            } else {
                return Some(Ordering::Less);
            }
        }
        for (s, o) in zip(&self.cards, &other.cards) {
            if s > o {
                return Some(Ordering::Greater);
            } else if o > s {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 7A                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_7a() {
    let file: File = File::open(".\\src\\day7\\day-7.txt").unwrap();
    let lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let mut hands: Vec<Hand> = lines.filter_map(|l| Hand::new_7a(&l)).collect();
    hands.sort();

    let total = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .reduce(|acc, e| acc + e);

    println!("Day 7a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 7B                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_7b() {
    let file: File = File::open(".\\src\\day7\\day-7.txt").unwrap();
    let lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let mut hands: Vec<Hand> = lines.filter_map(|l| Hand::new_7b(&l)).collect();
    hands.sort();

    let total = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .reduce(|acc, e| acc + e);

    println!("Day 7b answer = {}", total.unwrap());
}
