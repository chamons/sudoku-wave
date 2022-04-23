use std::fmt::{Display, Write};

use anyhow::{anyhow, Result};
use rand::{prelude::SliceRandom, seq::IteratorRandom, thread_rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameCell {
    SuperState(u16),
    Fixed(u16),
}

impl Default for GameCell {
    fn default() -> Self {
        GameCell::SuperState(ALL_CELL_POSSIBILITIES)
    }
}

impl Display for GameCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameCell::SuperState(_) => f.write_char('.')?,
            GameCell::Fixed(v) => f.write_str(&v.to_string())?,
        }
        Ok(())
    }
}

impl GameCell {
    pub fn pop_count(&self) -> Option<u32> {
        match self {
            GameCell::SuperState(v) => Some(v.count_ones()),
            GameCell::Fixed(_) => None,
        }
    }

    fn value_to_bit(value: u16) -> u16 {
        match value {
            1 => 0b00000000_00000001,
            2 => 0b00000000_00000010,
            3 => 0b00000000_00000100,
            4 => 0b00000000_00001000,
            5 => 0b00000000_00010000,
            6 => 0b00000000_00100000,
            7 => 0b00000000_01000000,
            8 => 0b00000000_10000000,
            9 => 0b00000001_00000000,
            _ => panic!("Invalid value in value_to_bit"),
        }
    }

    fn unset_bit_pattern(value: u16) -> u16 {
        match value {
            1 => 0b00000001_11111110,
            2 => 0b00000001_11111101,
            3 => 0b00000001_11111011,
            4 => 0b00000001_11110111,
            5 => 0b00000001_11101111,
            6 => 0b00000001_11011111,
            7 => 0b00000001_10111111,
            8 => 0b00000001_01111111,
            9 => 0b00000000_11111111,
            _ => panic!("Invalid value in value_to_bit"),
        }
    }

    pub fn potential_values(&mut self) -> Option<Vec<u16>> {
        match self {
            GameCell::SuperState(v) => {
                let mut values = vec![];
                for i in 1..=9 {
                    let mask = GameCell::value_to_bit(i);
                    if mask & *v == mask {
                        values.push(i);
                    }
                }
                Some(values)
            }
            GameCell::Fixed(_) => None,
        }
    }

    pub fn random_potential(&mut self) -> Option<u16> {
        self.potential_values()
            .map(|v| *v.choose(&mut thread_rng()).unwrap())
    }

    pub fn constrain(&mut self, cell: &GameCell) {
        if let GameCell::Fixed(constraint) = cell {
            match self {
                GameCell::SuperState(v) => *v = *v & GameCell::unset_bit_pattern(*constraint),
                GameCell::Fixed(_) => {}
            }
        }
    }
}

pub const ALL_CELL_POSSIBILITIES: u16 = 0b00000001_11111111;

pub struct GameState {
    cells: [[GameCell; 9]; 9],
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            cells: [[GameCell::default(); 9]; 9],
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.iter() {
            for value in line.iter() {
                value.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl GameState {
    pub fn get(&self, pos: Point) -> GameCell {
        self.cells[pos.y][pos.x]
    }

    pub fn cells_in_row(&self, row: usize) -> impl Iterator<Item = &GameCell> + '_ {
        self.cells[row].iter()
    }

    pub fn cells_in_col(&self, col: usize) -> impl Iterator<Item = GameCell> + '_ {
        self.cells.iter().map(move |r| r[col])
    }

    pub fn cells_in_house(&self, pos: Point) -> Vec<GameCell> {
        fn point_to_house_coord(x: usize) -> usize {
            match x {
                0..=2 => 0,
                3..=5 => 1,
                6..=8 => 2,
                _ => panic!("Invalid position in cells_in_house"),
            }
        }

        let house_x = point_to_house_coord(pos.x);
        let house_y = point_to_house_coord(pos.y);

        let start_house_x = house_x * 3;
        let start_house_y = house_y * 3;

        let mut cells = vec![];
        for line in self.cells.iter().skip(start_house_y).take(3) {
            for value in line.iter().skip(start_house_x).take(3) {
                cells.push(value.clone());
            }
        }
        cells
    }

    pub fn parse(problem: &str) -> Result<GameState> {
        let mut state = GameState::default();

        if problem.lines().count() != 9 {
            return Err(anyhow!("Incorrect input number of lines"));
        }
        for (line_index, line) in problem.lines().enumerate() {
            if line.len() != 9 {
                return Err(anyhow!("Incorrect line length"));
            }
            for (char_index, char) in line.chars().enumerate() {
                if let Some(value) = char.to_digit(10) {
                    state.cells[line_index][char_index] = GameCell::Fixed(value as u16);
                } else if char != '.' {
                    return Err(anyhow!("Invalid character input"))?;
                }
            }
        }
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn parse() {
        let problem = "91..8....
4..279...
.73....4.
3...4...1
5..3.1..2
8...6...4
.4....63.
...527..9
....3..87";

        let solution = GameState::parse(&problem).unwrap();
        assert_eq!(solution.get(Point::new(0, 0)), GameCell::Fixed(9));
        assert_eq!(solution.get(Point::new(0, 1)), GameCell::Fixed(4));
        assert_eq!(solution.get(Point::new(1, 0)), GameCell::Fixed(1));
        assert_eq!(solution.get(Point::new(1, 2)), GameCell::Fixed(7));
        assert_eq!(solution.get(Point::new(0, 3)), GameCell::Fixed(3));
        assert_eq!(
            solution.get(Point::new(1, 1)),
            GameCell::SuperState(ALL_CELL_POSSIBILITIES)
        );
        assert_eq!(
            solution.get(Point::new(8, 2)),
            GameCell::SuperState(ALL_CELL_POSSIBILITIES)
        );
    }

    #[test]
    fn as_string() {
        let problem = "91..8....
4..279...
.73....4.
3...4...1
5..3.1..2
8...6...4
.4....63.
...527..9
....3..87";

        let solution = GameState::parse(&problem).unwrap();
        assert_eq!(solution.to_string(), format!("{}\n", problem));
    }

    #[test]
    fn neighbors() {
        let problem = "91..8....
4..279...
.73....4.
3...4...1
5..3.1..2
8...6...4
.4....63.
...527..9
....3..87";

        let solution = GameState::parse(&problem).unwrap();
        assert_eq!(
            solution.cells_in_row(1).copied().collect::<Vec<GameCell>>(),
            vec!(
                GameCell::Fixed(4),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(2),
                GameCell::Fixed(7),
                GameCell::Fixed(9),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES)
            )
        );
        assert_eq!(
            solution.cells_in_col(2).collect::<Vec<GameCell>>(),
            vec!(
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(3),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES)
            )
        );
        assert_eq!(
            solution.cells_in_house(Point::new(0, 2)),
            vec!(
                GameCell::Fixed(9),
                GameCell::Fixed(1),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(4),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(7),
                GameCell::Fixed(3),
            )
        );
        assert_eq!(
            solution.cells_in_house(Point::new(8, 4)),
            vec!(
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(1),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(2),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(4),
            )
        );
        assert_eq!(
            solution.cells_in_house(Point::new(0, 8)),
            vec!(
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::Fixed(4),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
                GameCell::SuperState(ALL_CELL_POSSIBILITIES),
            )
        );
    }

    #[test]
    fn pop_count() {
        assert_eq!(
            GameCell::SuperState(ALL_CELL_POSSIBILITIES).pop_count(),
            Some(9)
        );
        assert_eq!(
            GameCell::SuperState(0b00000001_10101010).pop_count(),
            Some(5)
        );
        assert_eq!(GameCell::Fixed(4).pop_count(), None);
    }

    #[test]
    fn constrain() {
        let mut cell = GameCell::SuperState(ALL_CELL_POSSIBILITIES);
        cell.constrain(&GameCell::SuperState(ALL_CELL_POSSIBILITIES));
        assert_eq!(cell, GameCell::SuperState(ALL_CELL_POSSIBILITIES));

        let mut cell = GameCell::SuperState(ALL_CELL_POSSIBILITIES);
        cell.constrain(&GameCell::Fixed(2));
        assert_eq!(cell, GameCell::SuperState(0b00000001_11111101));

        let mut cell = GameCell::SuperState(0b00000001_11111101);
        cell.constrain(&GameCell::Fixed(4));
        assert_eq!(cell, GameCell::SuperState(0b00000001_11110101));
    }

    #[test]
    fn potential_values() {
        assert_eq!(
            GameCell::SuperState(ALL_CELL_POSSIBILITIES).potential_values(),
            Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
        assert_eq!(
            GameCell::SuperState(0b00000001_11111101).potential_values(),
            Some(vec![1, 3, 4, 5, 6, 7, 8, 9])
        );
        assert_eq!(
            GameCell::SuperState(0b00000000_00000001).potential_values(),
            Some(vec![1])
        );
        assert_eq!(
            GameCell::SuperState(0b00000001_00000000).potential_values(),
            Some(vec![9])
        );
        assert_eq!(GameCell::Fixed(4).potential_values(), None);
    }

    #[test]
    fn random_potential_values() {
        let expected: HashSet<u16> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9].iter().copied().collect();
        for _ in 0..100 {
            assert!(expected.contains(
                &GameCell::SuperState(ALL_CELL_POSSIBILITIES)
                    .random_potential()
                    .unwrap()
            ));
        }

        let expected: HashSet<u16> = vec![8, 9].iter().copied().collect();
        for _ in 0..100 {
            assert!(expected.contains(
                &GameCell::SuperState(0b00000001_10000000)
                    .random_potential()
                    .unwrap()
            ));
        }

        let expected: HashSet<u16> = vec![9].iter().copied().collect();
        for _ in 0..100 {
            assert!(expected.contains(
                &GameCell::SuperState(0b00000001_00000000)
                    .random_potential()
                    .unwrap()
            ));
        }

        assert_eq!(None, GameCell::Fixed(2).random_potential());
    }
}
