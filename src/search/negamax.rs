use shakmaty::{Chess, Position as _};

/// what is the value at the root of the game tree?
pub fn negamax(position: Chess, depth: u8, eval: fn(&Chess) -> i32) -> i32 {
    if depth == 0 {
        return eval(&position);
    } else {
        // loop over legal moves
        let mut max_value = 0;
        for mv in position.legal_moves() {
            let result_position = position.clone().play(mv).unwrap();
            let value = -negamax(result_position, depth - 1, eval);
            if value > max_value {
                max_value = value;
            }
        }
        return max_value;
    }
}

/// Evaluate the "value" of the position for the player who is about to move
fn eval(position: &Chess) -> i32 {
    1
}
