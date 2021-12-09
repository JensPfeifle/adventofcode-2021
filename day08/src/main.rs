use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// asdfas
// sdf
//
//
//
//
//
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
    let input = read("8.in").expect("Could not read input file.");
    let parsed = parse_input(&input);
    println!("{:?}", parsed);
    let mut count = 0;
    for line in parsed {
        let (inputs, outputs) = line;
        for output in outputs {
            let is_easy_digit = match output.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            };
            if is_easy_digit {
                println!("{:?}", output);
                count += 1;
            }
        }
    }
    println!("Count {:?}", count);
    Ok(())
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut parsed = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (input, output) = line
            .split('|')
            .map(|e| e.trim())
            .collect_tuple()
            .expect("Could not split line into input | output signals.");
        parsed.push((input.splitn(10, ' ').collect(), output.split(' ').collect()));
    }
    parsed
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = read("example.in").expect("Could not read example.in");
        let parsed = parse_input(&input);
        println!("{:?}", parsed);
    }
}
