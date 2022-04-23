use anyhow::Result;

use super::parser::*;

pub fn solve(problem: &str) -> Result<GameState> {
    let state = GameState::parse(problem)?;
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
