use super::piece_type::*;

fn validate_moves(moves: Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut valid_moves: Vec<(u8, u8)> = [].to_vec();
    for m in moves.iter() {
        let (m1, m2) = *m;
        if (m1 > 1 && m1 < 8) && (m2 < 8) {
            valid_moves.push((m1, m2))
        }
    }
    return valid_moves;
}

fn get_pawn_moves(start: (u8, u8), _is_white: bool) -> Vec<(u8, u8)> {
    let (m1, m2) = start;
    let mut possible_moves = [].to_vec();
    if _is_white {
        if m1 > 0 {
            possible_moves.push((m1 + 1, m2));
            possible_moves.push((m1 + 1, m2 + 1));
            if m1 == 1 {
                possible_moves.push((m1 + 2, m2));
            }
            if m2 > 0 {
                possible_moves.push((m1 + 1, m2 - 1));
            }
        }
    } else {
        if m1 < 7 {
            possible_moves.push((m1 - 1, m2));
            possible_moves.push((m1 - 1, m2 + 1));
            if m1 == 6 {
                possible_moves.push((m1 - 2, m2));
            }
            if m2 > 0 {
                possible_moves.push((m1 - 1, m2 - 1));
            }
        }
    }
    return validate_moves(possible_moves);
}

fn get_moves(_is_white: bool, start: (u8, u8), piece_type: PieceType) -> Vec<(u8, u8)> {
    match piece_type {
        PieceType::Pawn => get_pawn_moves(start, _is_white),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_move(is_white: bool, piece_type: PieceType, start: (u8, u8), end: (u8, u8)) {
        let moves = get_moves(is_white, start, piece_type);
        let found = moves
            .iter()
            .position(|(rank, file)| *rank == end.0 && *file == end.1);
        assert!(found.is_some());
    }

    fn assert_can_not_move(is_white: bool, piece_type: PieceType, start: (u8, u8), end: (u8, u8)) {
        let moves = get_moves(is_white, start, piece_type);
        let found = moves
            .iter()
            .position(|(rank, file)| *rank == end.0 && *file == end.1);
        assert!(found.is_none());
    }

    #[test]
    fn white_pawn_can_move_forward_one() {
        assert_move(true, PieceType::Pawn, (1, 0), (2, 0));
    }

    #[test]
    fn white_pawn_can_initially_move_forward_two() {
        assert_move(true, PieceType::Pawn, (1, 0), (3, 0));
    }

    #[test]
    fn white_pawn_can_capture_left() {
        assert_move(true, PieceType::Pawn, (4, 4), (5, 3));
    }

    #[test]
    fn white_pawn_can_capture_right() {
        assert_move(true, PieceType::Pawn, (4, 4), (5, 5));
    }

    #[test]
    fn white_pawn_move_forward_one_blocked_by_edge_of_board() {
        assert_can_not_move(true, PieceType::Pawn, (7, 7), (8, 7));
    }

    #[test]
    fn white_pawn_move_forward_two_blocked_by_edge_of_board() {
        assert_can_not_move(true, PieceType::Pawn, (7, 7), (9, 7));
    }

    // white_pawn_capture_left_blocked_by_edge_of_board is already impossible because file is an unsigned integer (thus can't be -1)

    #[test]
    fn white_pawn_capture_right_blocked_by_edge_of_board() {
        assert_can_not_move(true, PieceType::Pawn, (2, 7), (3, 8));
    }

    #[test]
    fn black_pawn_can_initially_move_forward_two() {
        assert_move(false, PieceType::Pawn, (6, 0), (4, 0));
    }

    #[test]
    fn black_pawn_can_move_forward_one() {
        assert_move(false, PieceType::Pawn, (6, 0), (5, 0));
    }

    #[test]
    fn black_pawn_can_capture_left() {
        assert_move(false, PieceType::Pawn, (5, 5), (6, 4));
    }

    #[test]
    fn black_pawn_can_capture_right() {
        assert_move(false, PieceType::Pawn, (5, 5), (6, 6));
    }

    // black_pawn_move_forward_one_blocked_by_edge_of_board is already impossible because rank is an unsigned integer (thus can't be -1)

    // black_pawn_move_forward_two_blocked_by_edge_of_board is already impossible because rank is an unsigned integer (thus can't be -1)

    // black_pawn_capture_left_blocked_by_edge_of_board is already impossible because file is an unsigned integer (thus can't be -1)

    #[test]
    fn black_pawn_capture_right_blocked_by_edge_of_board() {
        assert_can_not_move(false, PieceType::Pawn, (6, 7), (5, 8));
    }
}
