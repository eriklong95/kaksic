use shakmaty::{ByRole, Chess, Position, Square};

/// Evaluate the "value" of the position for the player who is about to move
pub fn eval(position: &Chess) -> i32 {
    let score = eval_material(position) + eval_position(position) + eval_checkmate(position);

    return score;
}

fn eval_checkmate(position: &Chess) -> i32 {
    if position.is_checkmate() {
        return i32::MIN + 1;
    }
    return 0;
}

fn eval_position(position: &Chess) -> i32 {
    let mut to_move_pieces_in_center = 0;

    let mut other_pieces_in_center = 0;

    for (sq, pc) in position.board() {
        if is_center(sq) {
            if pc.color == position.turn() {
                to_move_pieces_in_center += 1;
            } else {
                other_pieces_in_center += 1;
            }
        }
    }

    return (to_move_pieces_in_center - other_pieces_in_center) * 100;
}

fn is_center(sq: Square) -> bool {
    sq.file().ge(&shakmaty::File::C)
        && sq.file().le(&shakmaty::File::F)
        && sq.rank().ge(&shakmaty::Rank::Third)
        && sq.rank().le(&shakmaty::Rank::Sixth)
}

fn eval_material(position: &Chess) -> i32 {
    let material = position.board().material();
    let to_move_material_score = material_score(material.get(position.turn()));
    let other_material_score = material_score(material.get(position.turn().other()));

    let diff = to_move_material_score as i32 - other_material_score as i32;

    return diff;
}

fn material_score(material: &ByRole<u8>) -> u8 {
    let ByRole {
        pawn,
        knight,
        bishop,
        rook,
        queen,
        king,
    } = material;

    return pawn + knight * 3 + bishop * 3 + rook * 5 + queen * 9 + king * 100;
}
