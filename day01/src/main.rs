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
    let contents = read("1.in").expect("Could not read input file.");
    let input: Vec<usize> = contents
        .lines()
        .map(|x| x.parse::<usize>().expect("Could not parse value."))
        .collect();
    println!("Part1: {:?}", count_depth_increases(&input, 1));
    println!("Part2: {:?}", count_depth_increases(&input, 3));

    Ok(())
}

fn count_depth_increases(measurements: &Vec<usize>, window_size: usize) -> usize {
    let mut count = 0;

    let mut window_a: &[usize];
    let mut window_b: &[usize];

    for i in 0..measurements.len() - window_size {
        window_a = &measurements[i..i + window_size];
        window_b = &measurements[i + 1..i + 1 + window_size];

        let sum_a: usize = window_a.iter().sum();
        let sum_b: usize = window_b.iter().sum();
        if sum_b > sum_a {
            count = count + 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_pt1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&input, 1), 7);
    }

    #[test]
    fn example_pt2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_depth_increases(&input, 3), 5);
    }
}
