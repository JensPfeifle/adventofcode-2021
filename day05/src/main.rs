use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read<P>(path: P) -> std::io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let input = read("5.in").expect("Could not read input file.");
    let parsed = parse_input(&input);
    let answer = part1(&parsed);
    println!("Part 1: {:?}", answer);
    let answer = part2(&parsed);
    println!("Part 2: {:?}", answer);
    Ok(())
}

fn parse_input(input: &str) -> Vec<((u16, u16), (u16, u16))> {
    let mut parsed = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let (start, _, end) = line
            .split(' ')
            .collect_tuple()
            .expect("Could not split line into (start, end) tuple.");
        let (sa, sb) = start
            .splitn(2, ',')
            .map(|n| n.parse::<u16>().expect("Could not parse integer."))
            .collect_tuple()
            .expect("Could not split start tuple into coordinates.");
        let (ea, eb) = end
            .splitn(2, ',')
            .map(|n| n.parse::<u16>().expect("Could not parse integer."))
            .collect_tuple()
            .expect("Could not split end tuple into coordinates.");
        parsed.push(((sa, sb), (ea, eb)));
    }
    parsed
}

fn part1(input: &Vec<((u16, u16), (u16, u16))>) -> u16 {
    let mut counts: HashMap<(u16, u16), u16> = HashMap::new();
    for (start, end) in input {
        // part1, only vertical or horizontal lines
        if start.0 == end.0 || start.1 == end.1 {
            // min/max because ranges must be smaller..larger
            for i in start.0.min(end.0)..=start.0.max(end.0) {
                for j in start.1.min(end.1)..=start.1.max(end.1) {
                    counts.insert((i, j), counts.get(&(i, j)).unwrap_or(&0) + 1);
                }
            }
        }
    }
    counts.values().filter(|&v| v >= &2).count() as u16
}

fn part2(input: &Vec<((u16, u16), (u16, u16))>) -> u16 {
    let mut counts: HashMap<(u16, u16), u16> = HashMap::new();
    for (start, end) in input {
        if start.0 == end.0 || start.1 == end.1 {
            // vertical or horizontal lines
            // min/max because ranges must be smaller..larger
            for i in start.0.min(end.0)..=start.0.max(end.0) {
                for j in start.1.min(end.1)..=start.1.max(end.1) {
                    counts.insert((i, j), counts.get(&(i, j)).unwrap_or(&0) + 1);
                }
            }
        } else {
            // diagonal lines
            let (mut i, mut j) = start;
            loop {
                counts.insert((i, j), counts.get(&(i, j)).unwrap_or(&0) + 1);
                if (i, j) == *end {
                    break;
                }
                if start.0 < end.0 {
                    i += 1;
                } else {
                    i -= 1;
                }
                if start.1 < end.1 {
                    j += 1;
                } else {
                    j -= 1;
                }
            }
        }
    }
    counts.values().filter(|&v| v >= &2).count() as u16
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = read("example.in").expect("Could not read example.in");
        let parsed = parse_input(&input);
        println!("{:?}", parsed);
        let answer = part1(&parsed);
        println!("answer: {:?}", answer);
        assert_eq!(answer, 5);
    }

    #[test]
    fn example_pt2() {
        let input = read("example.in").expect("Could not read example.in");
        let parsed = parse_input(&input);
        println!("{:?}", parsed);
        let answer = part2(&parsed);
        println!("answer: {:?}", answer);
        assert_eq!(answer, 12);
    }
}
