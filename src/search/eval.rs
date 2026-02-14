use shakmaty::{ByRole, Chess, Position};

/// Evaluate the "value" of the position for the player who is about to move
pub fn eval(position: &Chess) -> i32 {
    eval_material(position)
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
