use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;
use regex::Regex;

/*******************************************************************************************/
/*                                                                                         */
/*                             Structs for both Questions                                  */
/*                                                                                         */
/*******************************************************************************************/

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 7A                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_8a() {
    let file: File = File::open(".\\src\\day8\\day-8.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let mut map = HashMap::new();

    let moves = lines.next().unwrap();
    let _ = lines.next();
    lines.for_each(|line| {
        let caps = re.captures(&line).unwrap();
        map.insert(
            caps[1].to_string(),
            (caps[2].to_string(), caps[3].to_string()),
        );
    });

    let mut curr = String::from("AAA");
    let mut total = 0;
    'outer: loop {
        for c in moves.chars() {
            if curr == "ZZZ" {
                break 'outer;
            }
            total += 1;
            let options = map.get(&curr).unwrap();
            match c {
                'L' => curr = options.0.to_owned(),
                'R' => curr = options.1.to_owned(),
                _ => unreachable!(),
            }
        }
    }

    println!("Day 8a answer = {}", total);
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 7B                                           */
/*                                                                                         */
/*******************************************************************************************/

#[derive(Debug, Clone)]
struct Loop {
    loop_size: usize,
    z_offset: usize,
}

fn analyze_loop(start: &str, moves: &str, map: &HashMap<String, (String, String)>) -> Loop {
    let mut curr = start.to_string();
    let len_moves = moves.chars().count();
    let mut steps: usize = 0;
    let mut set: HashMap<(usize, String), usize> = HashMap::new();
    let mut z_offsets = Vec::new();
    loop {
        for c in moves.chars() {
            let key = (steps % len_moves, curr.clone());
            if let Some(&base) = set.get(&key) {
                return Loop {
                    loop_size: steps - base,
                    z_offset: *z_offsets.get(0).unwrap(),
                };
            }
            set.insert(key, steps);
            if curr.ends_with("Z") {
                z_offsets.push(steps);
            }
            steps += 1;
            let options = map.get(&curr).unwrap();
            match c {
                'L' => curr = options.0.to_owned(),
                'R' => curr = options.1.to_owned(),
                _ => unreachable!(),
            }
        }
    }
}

pub fn day_8b() {
    let file: File = File::open(".\\src\\day8\\day-8.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let re = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    let mut map = HashMap::new();

    let moves = lines.next().unwrap();
    let _ = lines.next();

    let mut nodes = Vec::new();
    lines.for_each(|line| {
        let caps = re.captures(&line).unwrap();
        if caps[1].ends_with("A") {
            nodes.push(caps[1].to_string());
        }
        map.insert(
            caps[1].to_string(),
            (caps[2].to_string(), caps[3].to_string()),
        );
    });

    let mut loops = nodes
        .iter()
        .map(|n| analyze_loop(n, &moves, &map))
        .collect_vec();

    loops.sort_by(|a, b| {
        if a.loop_size < b.loop_size {
            Ordering::Greater
        } else if a.loop_size > b.loop_size {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let largest_loop = loops.drain(0..1).next().unwrap();

    let mut steps: usize = 0;
    for n in 0.. {
        steps = largest_loop.z_offset + n * largest_loop.loop_size;
        if loops
            .iter()
            .all(|l| (steps - l.z_offset) % l.loop_size == 0)
        {
            break;
        }
    }

    println!("Day 8b answer = {}", steps);
}
