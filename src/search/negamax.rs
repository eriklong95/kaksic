use std::i32;

use shakmaty::{Chess, Move, Position as _};

pub struct Report {
    pub nodes_visited: u64,
    pub best_move: Option<Move>,
}

/// what is the value at the root of the game tree? returns (value, nodes visited)
pub fn negamax(position: Chess, depth: u8, eval: fn(&Chess) -> i32, report: &mut Report) -> i32 {
    report.nodes_visited += 1;
    if depth == 0 || position.is_game_over() {
        return eval(&position);
    } else {
        // loop over legal moves
        let mut max_value = i32::MIN;

        for mv in position.legal_moves() {
            let result_position = position.clone().play(mv).unwrap();
            let value = -negamax(result_position, depth - 1, eval, report);

            if value > max_value {
                max_value = value;
                report.best_move = Some(mv);
            }
        }
        return max_value;
    }
}
