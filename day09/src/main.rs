use std::io;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

struct HeightMap {
    values: Vec<u32>,
    width: usize,
    height: usize,
}
impl HeightMap {
    fn from_lines(lines: &[&str]) -> Self {
        let height = lines.len();
        let width = lines[0].chars().count();
        let values: Vec<u32> = lines
            .iter()
            .map(|line| line.chars())
            .flatten()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        return Self {
            values,
            width,
            height,
        };
    }

    fn neighbors(&self, i: usize, j: usize) -> Vec<u32> {
        let mut values = Vec::new();
        if i > 0 {
            values.push(self[(i - 1, j)]);
        }
        if j > 0 {
            values.push(self[(i, j - 1)]);
        }
        if i + 1 < self.height {
            values.push(self[(i + 1, j)]);
        }
        if j + 1 < self.width {
            values.push(self[(i, j + 1)]);
        }

        values
    }

    fn total_risk_level(&self) -> u32 {
        self.find_low_points()
            .iter()
            .map(|&coords| self[coords])
            .map(|center_height| 1 + center_height)
            .sum()
    }

    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let center_height = self[(i, j)];
                let any_lower_neighbors = self.neighbors(i, j).iter().any(|v| v <= &center_height);
                if !any_lower_neighbors {
                    low_points.push((i, j));
                }
            }
        }
        low_points
    }
}
impl Index<(usize, usize)> for HeightMap {
    type Output = u32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        return &self.values[i * self.width + j];
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .expect("Could not read from stdin");
    let lines: Vec<&str> = buf.lines().collect();
    println!("{:?}", lines);
    let heightmap = HeightMap::from_lines(&lines);
    println!("{:?}", heightmap.total_risk_level());

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_pt1() {
        let example = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        let heightmap = HeightMap::from_lines(&example);
        assert_eq!(heightmap.width, 10);
        assert_eq!(heightmap.height, 5);
        assert_eq!(heightmap.total_risk_level(), 15);
    }
}
