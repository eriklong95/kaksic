use shakmaty::{Chess, Position as _};

use crate::search::Searcher;

/// what is the value at the root of the game tree? returns (value, nodes visited)
pub fn negamax(position: Chess, depth: u8, eval: fn(&Chess) -> i32) -> (i32, u64) {
    if depth == 0 {
        return (eval(&position), 1);
    } else {
        // loop over legal moves
        let mut max_value = 0;
        let mut total_nodes = 0;
        for mv in position.legal_moves() {
            let result_position = position.clone().play(mv).unwrap();
            let (value, nodes) = negamax(result_position, depth - 1, eval);
            let negated_value = -value;
            if negated_value > max_value {
                max_value = negated_value;
            }
            total_nodes += nodes;
        }
        return (max_value, total_nodes);
    }
}
