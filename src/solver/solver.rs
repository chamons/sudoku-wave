use anyhow::Result;
use rand::{prelude::IteratorRandom, thread_rng};

use super::state::*;

fn lowest_entropy(state: &GameState) -> Option<Point> {
    let mut cells = state.cells();
    println!("A: {:?}", state.get(&Point::new(3, 0)).pop_count());
    println!("B: {:?}", state.cells().count());
    let min = state
        .cells()
        .min_by_key(|(_, cell)| cell.pop_count().unwrap_or(u32::MAX));
    if let Some(min) = min {
        if let Some(min_value) = min.1.pop_count() {
            let choice = state
                .cells()
                .filter(|(_, cell)| cell.pop_count() == Some(min_value))
                .choose(&mut thread_rng());
            choice.map(|c| c.0)
        } else {
            None
        }
    } else {
        None
    }
}

fn affected_cells(source: &Point) -> Vec<Point> {
    let mut cells = vec![];
    cells.append(&mut GameState::in_col(source.x));
    cells.append(&mut GameState::in_row(source.y));
    cells.append(&mut GameState::in_house(source));
    cells
}

fn constrain(state: &mut GameState, source: &Point) {
    let mut queue = affected_cells(source);
    let constrained_value = state.get(source);
    assert!(matches!(constrained_value, GameCell::Fixed(_)));
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        if state.get_mut(&current).constrain(&constrained_value) {
            queue.append(&mut affected_cells(&current));
        }
    }
}

pub fn solve(problem: &str) -> Result<GameState> {
    let mut state = GameState::parse(problem)?;
    loop {
        let lowest = lowest_entropy(&state);
        if let Some(lowest) = lowest {
            println!("Chosen Spot: {:?}", lowest);
            let chosen_value = state.get(&lowest).random_potential().unwrap();
            println!("Chosen Value: {:?}", chosen_value);
            *state.get_mut(&lowest) = GameCell::Fixed(chosen_value);
            constrain(&mut state, &lowest);
        } else {
            break;
        }
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_simple_problem() {
        let problem = "91..8....
4..279...
.73....4.
3...4...1
5..3.1..2
8...6...4
.4....63.
...527..9
....3..87";

        let solution = "915483726
486279153
273156948
397842561
564391872
821765394
742918635
638527419
159634287";

        assert_eq!(solve(&problem).unwrap().to_string(), solution);
    }
}
