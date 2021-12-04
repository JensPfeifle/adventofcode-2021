use ndarray::prelude::*;
use ndarray::Array;
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

struct BingoGame {
    numbers: Vec<u8>,
    current_number: usize,
    boards: Array<u8, Ix3>,
    marks: Array<u8, Ix3>,
}

impl BingoGame {
    fn new(input: &str) -> Self {
        let boards = parse_boards(&input);
        let marks = Array::<u8, _>::zeros(boards.raw_dim());
        BingoGame {
            numbers: parse_numbers(&input),
            current_number: 0,
            boards,
            marks,
        }
    }
    /// Play game until a winner is found.
    fn play(&mut self) -> u32 {
        loop {
            let result = self.next_round();
            if let Some(winner) = result {
                println!("Winner is {:?}!", winner);
                let score = self.calculate_score(winner);
                println!("Score is {:?}!", score);
                return score;
            }
        }
    }

    /// Returns Some(winning board index).
    fn next_round(&mut self) -> Option<usize> {
        println!("Round {:?}!", self.current_number);
        let number = self.numbers[self.current_number];
        println!("We've got a {:?}", number);
        let matches = self.boards.map(|&field| field == number);
        println!("{:?}", self.boards.slice(s![0, .., ..]));
        println!("{:?}", matches.slice(s![0, .., ..]));
        self.marks.zip_mut_with(&matches, |marked, &matched| {
            *marked = if *marked == 1 || matched { 1u8 } else { 0u8 };
        });
        if let Some(winner) = self.check_for_winners() {
            return Some(winner);
        };
        self.current_number += 1;
        None
    }

    fn calculate_score(&self, board_idx: usize) -> u32 {
        let board = self.boards.slice(s![board_idx, .., ..]);
        let marks = self.marks.slice(s![board_idx, .., ..]);
        println!("board: {:?}", board);
        println!("marks: {:?}", marks);
        let board: Vec<&u8> = board.iter().collect();
        let marks: Vec<&u8> = marks.iter().collect();
        println!("board: {:?}", board);
        println!("marks: {:?}", marks);
        // sum unmarked fields of board, multiply by most recent number called
        let sum_unmarked: u32 = board
            .iter()
            .zip(marks)
            .filter(|(_, &marked)| marked == 0)
            .map(|(&value, _)| *value as u32)
            .sum();
        println!("sum: {:?}", sum_unmarked);
        println!("number: {:?}", self.numbers[self.current_number]);
        sum_unmarked * self.numbers[self.current_number] as u32
    }

    /// Checks if any boards have won, return the board index.
    fn check_for_winners(&self) -> Option<usize> {
        for (idx, marks) in self.marks.axis_iter(Axis(0)).enumerate() {
            println!("marks: {:?}", marks);
            for row in marks.rows() {
                println!("r: {:?}", row);
                if row.sum() == 5 {
                    return Some(idx);
                }
            }
            for col in marks.columns() {
                println!("r: {:?}", col);
                if col.sum() == 5 {
                    return Some(idx);
                }
            }
        }

        None
    }
}

fn main() -> std::io::Result<()> {
    let input = read("4.in").expect("Could not read input file.");
    let mut game = BingoGame::new(&input);
    let score = game.play();
    println!("Score is {:?}!", score);
    Ok(())
}

fn parse_numbers(input: &str) -> Vec<u8> {
    let numberline = input.lines().nth(0).expect("Can't find numberline");
    numberline
        .split(',')
        .map(|n| n.parse().expect("Unable to parse u8"))
        .collect()
}

fn parse_boards(input: &str) -> Array<u8, Ix3> {
    let board_inputs: Vec<&str> = input.split("\n\n").skip(1).collect(); // skip number line
    let num_boards = board_inputs.len();

    let mut boards: Array<u8, Ix3> = Array::zeros((num_boards, 5, 5).f());
    for (idx, board) in board_inputs.iter().enumerate() {
        let rows: Vec<&str> = board.lines().collect();
        for (r_idx, row) in rows.iter().enumerate() {
            let split_row: Vec<&str> = row.split_whitespace().collect();
            let parsed_row: Vec<u8> = split_row.iter().map(|n| n.parse().unwrap()).collect();
            let arr = Array::from_vec(parsed_row);
            boards.slice_mut(s![idx, r_idx, ..]).assign(&arr);
        }
    }
    boards
}
//fn parse_boards(input: &str) ->  ()

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = read("example.in").expect("Could not read example.in");

        println!("{:?}", parse_numbers(&input));
        println!("{:?}", parse_boards(&input));
        let mut game = BingoGame::new(&input);
        let score = game.play();
        println!("Score is {:?}!", score);
        assert_eq!(score, 4512);
    }
}
