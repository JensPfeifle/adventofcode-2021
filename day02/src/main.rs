use itertools::Itertools;
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
    let contents = read("2.in").expect("Could not read input file.");
    let mut sub = Submarine { x: 0, y: 0 };
    let instructions = parse_instructions(&contents);
    sub.interpret(&instructions);
    println!("{:?}", sub.x * sub.y);

    Ok(())
}

struct Submarine {
    x: u32,
    y: u32,
}

enum Instruction {
    Down(u32),
    Up(u32),
    Forward(u32),
}

impl Submarine {
    fn interpret(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::Down(unit) => self.y = self.y + unit,
                Instruction::Up(unit) => self.y = self.y - unit,
                Instruction::Forward(unit) => self.x = self.x + unit,
            };
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (direction, units) = line.splitn(2, ' ').collect_tuple().unwrap();
            let units: u32 = units.parse().expect("Could not parse units.");
            match direction {
                "forward" => Instruction::Forward(units),
                "down" => Instruction::Down(units),
                "up" => Instruction::Up(units),
                _ => panic!("{}", format!("Unkown direction: {:?}", direction)),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = "forward 5\n\
                     down 5\n\
                     forward 8\n\
                     up 3\n\
                     down 8\n\
                     forward 2";

        let mut sub = Submarine { x: 0, y: 0 };

        let instructions: Vec<Instruction> = parse_instructions(&input);
        sub.interpret(&instructions);
        assert_eq!(sub.x * sub.y, 150);
    }

    #[test]
    fn example_pt2() {
        let input = "forward 5\n\
                     down 5\n\
                     forward 8\n\
                     up 3\n\
                     down 8\n\
                     forward 2";
    }
}
