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

fn filter_report<F>(report: Vec<Vec<char>>, filter: F) -> u32
where
    F: Fn(usize, usize) -> char,
{
    let mut report = report.clone();
    for col_idx in 0..report[0].len() {
        let column: Vec<char> = report.iter().map(|row| row[col_idx]).collect();
        let num_zeros = column.iter().filter(|&c| c == &'0').count();
        let num_ones = column.iter().filter(|&c| c == &'1').count();
        let to_keep = filter(num_zeros, num_ones);
        report = report
            .into_iter()
            .filter(|row| row[col_idx] == to_keep)
            .collect();

        if report.len() == 1 {
            let string = report[0].iter().collect::<String>();
            return u32::from_str_radix(&string, 2).expect("Could not convert bits.");
        }
    }
    panic!("Report did not reduce to single report value.")
}

fn oxygen_generator_rating(report: &str) -> u32 {
    let report: Vec<Vec<char>> = report.lines().map(|line| line.chars().collect()).collect();
    filter_report(report, |num_zeros, num_ones| {
        if num_zeros == num_ones {
            '1'
        } else if num_zeros > num_ones {
            '0'
        } else {
            '1'
        }
    })
}

fn co2_scrubber_rating(report: &str) -> u32 {
    let report: Vec<Vec<char>> = report.lines().map(|line| line.chars().collect()).collect();
    filter_report(report, |num_zeros, num_ones| {
        if num_zeros == num_ones {
            '0'
        } else if num_zeros < num_ones {
            '0'
        } else {
            '1'
        }
    })
}

fn power_consumption(report: &str) -> u32 {
    let report: Vec<Vec<char>> = report.lines().map(|line| line.chars().collect()).collect();
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for col_idx in 0..report[0].len() {
        let column: Vec<char> = report.iter().map(|row| row[col_idx]).collect();
        let num_zeros = column.iter().filter(|&c| c == &'0').count();
        let num_ones = column.iter().filter(|&c| c == &'1').count();
        if num_zeros > num_ones {
            gamma = gamma + "0";
            epsilon = epsilon + "1";
        } else {
            gamma = gamma + "1";
            epsilon = epsilon + "0";
        }
    }

    let gamma_int = u32::from_str_radix(&gamma, 2).expect("Could not convert bits.");
    let epsilon_int = u32::from_str_radix(&epsilon, 2).expect("Could not convert bits.");
    gamma_int * epsilon_int
}

fn main() -> std::io::Result<()> {
    let report = read("3.in").expect("Could not read input file.");
    println!("Power consumption: {:?}", power_consumption(&report));
    let ox = oxygen_generator_rating(&report);
    println!("Oxygen generator rating: {:?}", ox);
    let co2 = co2_scrubber_rating(&report);
    println!("CO2 scrubber rating: {:?}", co2);
    println!("Life support rating: {:?}", ox * co2);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    fn example() -> &'static str {
        "00100\n\
         11110\n\
         10110\n\
         10111\n\
         10101\n\
         01111\n\
         00111\n\
         11100\n\
         10000\n\
         11001\n\
         00010\n\
         01010\n"
    }
    #[test]
    fn test_part1() {
        assert_eq!(power_consumption(example()), 198);
    }
    #[test]
    fn test_part2_oxygen() {
        assert_eq!(oxygen_generator_rating(example()), 23);
    }
    #[test]
    fn test_part2_co2() {
        assert_eq!(co2_scrubber_rating(example()), 10);
    }
}
