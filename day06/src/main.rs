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
    let input = read("6.in").expect("Could not read input file.");
    let parsed: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().expect("Failed to parse integer"))
        .collect();

    let mut fishes = [0usize; 9];
    for n in 0..9usize {
        fishes[n] = parsed.iter().filter(|&&timer| timer == n).count();
    }
    let mut gen: usize = 0;
    while gen < 80 {
        let sum = reproduce(&mut fishes);
        println!("{:2}: {:?} -> {:?}", gen, fishes, sum);
        gen += 1;
    }
    let sum: usize = fishes.iter().sum();
    println!("Part1: {:?} fishies", sum);
    while gen < 256 {
        let sum = reproduce(&mut fishes);
        println!("{:2}: {:?} -> {:?}", gen, fishes, sum);
        gen += 1;
    }
    let sum: usize = fishes.iter().sum();
    println!("Part2: {:?} fishies", sum);
    Ok(())
}

fn reproduce(fishes: &mut [usize; 9]) -> usize {
    fishes[7] += fishes[0]; // assign to 7 (pre-shift)
    fishes.rotate_left(1);
    let sum = fishes.iter().sum::<usize>();
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = "3,4,3,1,2";
        let parsed: Vec<usize> = input
            .split(',')
            .map(|v| v.parse().expect("Failed to parse usize"))
            .collect();
        println!("{:?}", parsed);

        let mut fishes = [0usize; 9];
        for n in 0..9usize {
            fishes[n] = parsed.iter().filter(|&&timer| timer == n).count();
        }
        let mut gen: usize = 0;
        while gen < 18 {
            let sum = reproduce(&mut fishes);
            println!("{:2}: {:?} -> {:?}", gen, fishes, sum);
            gen += 1;
        }
        assert_eq!(fishes.iter().sum::<usize>(), 26);
        while gen < 80 {
            let sum = reproduce(&mut fishes);
            println!("{:2}: {:?} -> {:?}", gen, fishes, sum);
            gen += 1;
        }
        assert_eq!(fishes.iter().sum::<usize>(), 5934);
    }
}
