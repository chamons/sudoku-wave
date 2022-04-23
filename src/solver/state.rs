use std::fmt::{Display, Write};

use anyhow::{anyhow, Result};

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
}
