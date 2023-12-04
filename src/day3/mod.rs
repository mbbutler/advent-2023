use itertools::iproduct;
use std::collections::{BTreeSet, HashMap};
use std::iter::Enumerate;
use std::ops::Range;
use std::str::Chars;
use std::{
    fs::File,
    io::{self, BufRead},
};

/*******************************************************************************************/
/*                                                                                         */
/*                                DAY 3 GENERAL STRUCTS                                    */
/*                                                                                         */
/*******************************************************************************************/

struct Part {
    line: usize,
    bounds: Range<usize>,
    value: usize,
}

impl Part {
    fn gen_box_coords(&self) -> Vec<(usize, usize)> {
        let x: Vec<usize> = (self.line.saturating_sub(1)..=self.line + 1).collect();
        let y: Vec<usize> = (self.bounds.start.saturating_sub(1)..=self.bounds.end).collect();
        iproduct!(x, y).collect()
    }

    fn check_part(&self, set: &BTreeSet<(usize, usize)>) -> bool {
        self.gen_box_coords()
            .into_iter()
            .any(|k| set.get(&k).is_some())
    }

    fn add_part(&self, gear_map: &mut HashMap<(usize, usize), Vec<usize>>) {
        for k in self.gen_box_coords() {
            gear_map.entry(k).and_modify(|x| x.push(self.value));
        }
    }
}

struct PartsFinder<'a> {
    line_num: usize,
    chars: Enumerate<Chars<'a>>,
}

impl<'a> PartsFinder<'a> {
    fn new(line: &'a str, line_num: usize) -> Self {
        let chars = line.chars().enumerate();
        PartsFinder { line_num, chars }
    }
}

impl Iterator for PartsFinder<'_> {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num_chars = Vec::new();
        while let Some(c) = self.chars.next() {
            if c.1.is_numeric() {
                num_chars.push(c);
                while let Some(d) = self.chars.next() {
                    if !d.1.is_numeric() {
                        return Some(Part {
                            line: self.line_num,
                            bounds: num_chars.first().unwrap().0..d.0,
                            value: num_chars
                                .iter()
                                .map(|v| v.1)
                                .collect::<String>()
                                .parse::<usize>()
                                .unwrap(),
                        });
                    }
                    num_chars.push(d);
                }
            }
        }

        if !num_chars.is_empty() {
            return Some(Part {
                line: self.line_num,
                bounds: num_chars.first().unwrap().0..num_chars.last().unwrap().0 + 1,
                value: num_chars
                    .iter()
                    .map(|v| v.1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            });
        }
        None
    }
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 3A                                           */
/*                                                                                         */
/*******************************************************************************************/

fn find_symbols(line: &str, line_num: usize, set: &mut BTreeSet<(usize, usize)>) {
    for (i, c) in line.chars().enumerate() {
        if c != '.' && !c.is_numeric() {
            set.insert((line_num, i));
        }
    }
}

pub fn day_3a() {
    let file: File = File::open(".\\src\\day3\\day-3.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut symbol_set: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (i, line) in lines.iter().enumerate() {
        find_symbols(line, i, &mut symbol_set);
    }

    let total = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| PartsFinder::new(line, i).collect::<Vec<Part>>())
        .filter(|p| p.check_part(&mut symbol_set))
        .map(|p| p.value)
        .reduce(|acc, e| acc + e);

    println!("Day 3a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 3B                                           */
/*                                                                                         */
/*******************************************************************************************/

fn find_gears(line: &str, line_num: usize, map: &mut HashMap<(usize, usize), Vec<usize>>) {
    for (i, _) in line.chars().enumerate().filter(|(_, c)| *c == '*') {
        map.insert((line_num, i), Vec::new());
    }
}

pub fn day_3b() {
    let file: File = File::open(".\\src\\day3\\day-3.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    let mut gear_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        find_gears(line, i, &mut gear_map);
    }

    for (i, line) in lines.iter().enumerate() {
        for part in PartsFinder::new(line, i) {
            part.add_part(&mut gear_map);
        }
    }

    let total = gear_map
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .reduce(|acc, e| acc + e);

    println!("Day 3b answer = {}", total.unwrap());
}
