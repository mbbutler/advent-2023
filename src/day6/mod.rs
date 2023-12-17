use std::{
    fs::File,
    io::{self, BufRead},
    iter::zip,
};

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 6A                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_6a() {
    let file: File = File::open(".\\src\\day6\\day-6.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let times = lines.next().unwrap();
    let times = times
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|n| n.parse::<usize>().ok());

    let distances = lines.next().unwrap();
    let distances = distances
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|n| n.parse::<usize>().ok());

    let total = zip(times, distances)
        .map(|(b, c)| {
            let sqrt = ((b * b - 4 * c) as f64).sqrt();
            let lower = (((b as f64) - sqrt) / (2.0)).ceil() as usize;
            let upper = (((b as f64) + sqrt) / (2.0)).floor() as usize;
            upper - lower + 1
        })
        .reduce(|acc, e| acc * e);

    println!("Day 6a answer = {}", total.unwrap());
}

/*******************************************************************************************/
/*                                                                                         */
/*                                        DAY 6B                                           */
/*                                                                                         */
/*******************************************************************************************/

pub fn day_6b() {
    let file: File = File::open(".\\src\\day6\\day-6.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().filter_map(|l| l.ok());

    let times = lines.next().unwrap();
    let b = times
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let distances = lines.next().unwrap();
    let c = distances
        .split(':')
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<usize>()
        .unwrap();

    let sqrt = ((b * b - 4 * c) as f64).sqrt();
    let lower = (((b as f64) - sqrt) / (2.0)).ceil() as usize;
    let upper = (((b as f64) + sqrt) / (2.0)).floor() as usize;
    let total = upper - lower + 1;

    println!("Day 6b answer = {}", total);
}
