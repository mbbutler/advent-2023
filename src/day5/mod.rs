use std::ops::Range;
use std::{
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

/*******************************************************************************************/
/*                                                                                         */
/*                             Structs for both Questions                                  */
/*                                                                                         */
/*******************************************************************************************/

#[derive(Debug)]
struct RangeMap {
    dest: usize,
    src: Range<usize>,
}

#[derive(Debug)]
struct ItemMap {
    maps: Vec<RangeMap>,
}

fn parse_maps(lines: &mut impl Iterator<Item = String>) -> Vec<ItemMap> {
    lines.next();
    let mut item_maps = Vec::new();
    while let Some(_) = lines.next() {
        let mut maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                item_maps.push(ItemMap { maps });
                break;
            }
            let mut vals = line.split(' ').filter_map(|s| s.parse::<usize>().ok());
            let (dest, src, len) = vals.next_tuple().unwrap();
            let map = RangeMap {
                dest,
                src: src..(src + len),
            };
            maps.push(map);
        }
    }
    item_maps
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 5A                                           */
/*                                                                                         */
/*******************************************************************************************/

fn map_value(val: usize, map: &ItemMap) -> usize {
    map.maps
        .iter()
        .find_map(|m| {
            if m.src.contains(&val) {
                Some(m.dest + val - m.src.start)
            } else {
                None
            }
        })
        .unwrap_or(val)
}

pub fn day_5a() {
    let file: File = File::open(".\\src\\day5\\day-5.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let seed_line = lines.next().unwrap();
    let mut seeds: Vec<usize> = seed_line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let maps: Vec<ItemMap> = parse_maps(&mut lines);

    let lowest = seeds
        .iter_mut()
        .map(|s| {
            for map in &maps {
                *s = map_value(*s, &map);
            }
            s
        })
        .min();

    println!("Day 5a answer = {}", lowest.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 5A                                           */
/*                                                                                         */
/*******************************************************************************************/

struct MapResult {
    unmodifed: Vec<Range<usize>>,
    modified: Vec<Range<usize>>,
}
trait Map {
    fn apply_map(&self, map: &RangeMap) -> MapResult;
}

impl Map for Range<usize> {
    fn apply_map(&self, map: &RangeMap) -> MapResult {
        if self.end <= map.src.start || self.start >= map.src.end {
            return MapResult {
                unmodifed: vec![self.clone()],
                modified: vec![],
            };
        } else if self.start >= map.src.start && self.end <= map.src.end {
            return MapResult {
                unmodifed: vec![],
                modified: vec![Range {
                    start: map.dest + self.start - map.src.start,
                    end: map.dest + self.end - map.src.start,
                }],
            };
        } else if self.end <= map.src.end {
            return MapResult {
                unmodifed: vec![Range {
                    start: self.start,
                    end: map.src.start,
                }],
                modified: vec![Range {
                    start: map.dest,
                    end: map.dest + self.end - map.src.start,
                }],
            };
        } else if self.start >= map.src.start {
            return MapResult {
                unmodifed: vec![Range {
                    start: map.src.end,
                    end: self.end,
                }],
                modified: vec![Range {
                    start: map.dest + self.start - map.src.start,
                    end: map.dest + map.src.len(),
                }],
            };
        } else {
            return MapResult {
                unmodifed: vec![
                    Range {
                        start: self.start,
                        end: map.src.start,
                    },
                    Range {
                        start: map.src.end,
                        end: self.end,
                    },
                ],
                modified: vec![Range {
                    start: map.dest,
                    end: map.dest + map.src.len(),
                }],
            };
        }
    }
}

struct SeedRanges {
    ranges: Vec<Range<usize>>,
}

impl SeedRanges {
    fn apply_item_map(&mut self, item_map: &ItemMap) {
        let mut unmodified = self.ranges.clone();
        let mut modified = Vec::new();
        for map in &item_map.maps {
            let results: Vec<MapResult> = unmodified.iter().map(|r| r.apply_map(map)).collect();
            unmodified = results.iter().flat_map(|m| m.unmodifed.clone()).collect();
            modified.extend(results.iter().flat_map(|m| m.modified.clone()));
        }
        modified.extend(unmodified);
        self.ranges = modified;
    }

    fn get_min(&self) -> usize {
        self.ranges.iter().map(|r| r.start).min().unwrap()
    }
}

pub fn day_5b() {
    let file: File = File::open(".\\src\\day5\\day-5.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let seed_line = lines.next().unwrap();
    let seeds: Vec<usize> = seed_line
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let ranges: Vec<Range<usize>> = seeds.chunks_exact(2).map(|s| s[0]..(s[0] + s[1])).collect();
    let mut seed_ranges = SeedRanges { ranges };
    let item_maps: Vec<ItemMap> = parse_maps(&mut lines);

    for item_map in &item_maps {
        seed_ranges.apply_item_map(item_map);
    }

    let lowest = seed_ranges.get_min();
    println!("Day 5b answer = {}", lowest);
}
