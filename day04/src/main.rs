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
    scores: Vec<u32>,
    winners: Vec<usize>,
    boards: Array<u8, Ix3>,
    marks: Array<u8, Ix3>,
}

impl BingoGame {
    fn new(input: &str) -> Self {
        let boards = parse_boards(&input);
        let marks = Array::<u8, _>::zeros(boards.raw_dim());
        let num_boards = boards.dim().0;
        BingoGame {
            numbers: parse_numbers(&input),
            scores: vec![0; num_boards],
            boards,
            marks,
            winners: vec![],
        }
    }

    /// Play game.
    fn play(&mut self) {
        for number in self.numbers.iter_mut() {
            let matches = self.boards.map(|field| field == number);
            self.marks.zip_mut_with(&matches, |marked, &matched| {
                *marked = if *marked == 1 || matched { 1u8 } else { 0u8 };
            });
            let mut round_winners = vec![];
            for (idx, marks) in self.marks.axis_iter(Axis(0)).enumerate() {
                if self.winners.contains(&idx) {
                    // board has already won, don't check again
                    continue;
                }
                for row in marks.rows() {
                    if row.sum() == 5 {
                        round_winners.push(idx);
                    }
                }
                for col in marks.columns() {
                    if col.sum() == 5 {
                        round_winners.push(idx);
                    }
                }
            }
            for board_idx in round_winners {
                let board = self.boards.slice(s![board_idx, .., ..]);
                let marks = self.marks.slice(s![board_idx, .., ..]);
                // sum unmarked fields of board
                let sum_unmarked: u32 = board
                    .iter()
                    .zip(&marks)
                    .filter(|(_, &marked)| marked == 0)
                    .map(|(&value, _)| value as u32)
                    .sum();
                // multiply by previously called number
                let score = sum_unmarked * *number as u32;
                self.scores[board_idx] = score;
                self.winners.push(board_idx);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = read("4.in").expect("Could not read input file.");
    let mut game = BingoGame::new(&input);

    game.play();
    let &winner = game.winners.iter().nth(0).expect("No winners!");
    println!("Part1: Score of first winner is {:?}!", game.scores[winner]);
    let &winner = game.winners.iter().last().expect("No winners!");
    println!("Part2: Score of last winner is {:?}!", game.scores[winner]);

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let input = read("example.in").expect("Could not read example.in");
        let mut game = BingoGame::new(&input);
        game.play();
        println!("{:?}", game.winners);
        println!("{:?}", game.scores);
        let &winner = game.winners.iter().nth(0).expect("No winners!");
        let score = game.scores[winner];
        assert_eq!(score, 4512);
    }

    #[test]
    fn example_pt2() {
        let input = read("example.in").expect("Could not read example.in");
        let mut game = BingoGame::new(&input);
        game.play();
        let &winner = game.winners.iter().last().expect("No winners!");
        let score = game.scores[winner];
        assert_eq!(score, 1924);
    }
}
