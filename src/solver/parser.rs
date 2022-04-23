use std::fmt::{Display, Write};

use anyhow::{anyhow, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellValue {
    SuperState(u16),
    Fixed(u16),
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::SuperState(ALL_CELL_POSSIBILITIES)
    }
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellValue::SuperState(_) => f.write_char('.')?,
            CellValue::Fixed(v) => f.write_str(&v.to_string())?,
        }
        Ok(())
    }
}

pub const ALL_CELL_POSSIBILITIES: u16 = 0b00000001_11111111;

pub struct GameState {
    cells: [[CellValue; 9]; 9],
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            cells: [[CellValue::default(); 9]; 9],
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

impl GameState {
    pub fn get(&self, x: usize, y: usize) -> CellValue {
        self.cells[y][x]
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
                    state.cells[line_index][char_index] = CellValue::Fixed(value as u16);
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
        assert_eq!(solution.get(0, 0), CellValue::Fixed(9));
        assert_eq!(solution.get(0, 1), CellValue::Fixed(4));
        assert_eq!(solution.get(1, 0), CellValue::Fixed(1));
        assert_eq!(solution.get(1, 2), CellValue::Fixed(7));
        assert_eq!(solution.get(0, 3), CellValue::Fixed(3));
        assert_eq!(
            solution.get(1, 1),
            CellValue::SuperState(ALL_CELL_POSSIBILITIES)
        );
        assert_eq!(
            solution.get(8, 2),
            CellValue::SuperState(ALL_CELL_POSSIBILITIES)
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
}
