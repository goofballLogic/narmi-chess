use super::piece_type::*;

#[derive(Debug, PartialEq)]
pub struct Notation {
    text: String,
    rank: Option<u8>,
    file: Option<u8>,
    piece_type: PieceType,
    capture: bool,
    from_file: Option<u8>,
    from_rank: Option<u8>,
    check: bool,
    checkmate: bool,
    enpassant: bool,
    queen_side_castle: bool,
    king_side_castle: bool,
    promoted_to_piece_type: Option<PieceType>
}

/*
    Try to parse the character to 0..7
        If it's a rank (a..h) then we'll want to treat it as base 18 (0..h) and subtract 10
        If it's a file (1..8) then we'll want to treat it as base 10 (0..9) and subtract 1
*/
fn parse_coordinate(c: char, parse_radix: u32, parse_offset: u32) -> Option<u8> {
    let digit = c.to_digit(parse_radix)?; // will return None if this wasn't a digit
    // is it out of range?
    if (digit < parse_offset) || (digit > parse_offset + 7) {
        None
    } else {
        // calculate the resulting coordinate
        Some((digit - parse_offset) as u8)
    }
}

fn decode_coordinate(notation: &str, reverse_index: usize, parse_radix: u32, parse_offset: u32, coordinate_name: &str, valid_range: &str) -> u8 {

    // some panics that will come in handy
    let invalid = | what: &str | panic!("{} in move notation: {}. Must be {}", what, notation, valid_range);
    let invalid_coordinate = | c: char | invalid(&format!("Invalid {} {}", coordinate_name, c));

    // try to pick the character which is nth from the end
    match notation.chars().rev().nth(reverse_index) {

        // if we didn't find any thing in the required position, bail out:
        None => invalid(&format!("Missing {}", coordinate_name)),


        Some(c) => match parse_coordinate(c, parse_radix, parse_offset) {
            None => invalid_coordinate(c),
            Some(coordinate) => coordinate
        },
    }
}

// Ranks are rows that go from side to side across the chessboard and are referred to by numbers
fn decode_rank(notation: &str) -> u8 {
    decode_coordinate(notation, 0, 10, 1, "rank", "1..8")
}

// Files are columns that go up and down the chessboard, and each board has eight of them (A-H)
fn decode_file(notation: &str) -> u8 {
    decode_coordinate(notation, 1, 18, 10, "file", "a..h")
}

fn decode_piecetype_character(notation: &str, piecetype_character: char, is_capturing_move: bool) -> PieceType{

    let invalid = || panic!("Invalid piece {} in move notation: {}. Must be K, Q, B, R or N, or the source file a..h", piecetype_character, notation);
    match piecetype_character {
        'K' => PieceType::King,
        'Q' => PieceType::Queen,
        'B' => PieceType::Bishop,
        'R' => PieceType::Rook,
        'N' => PieceType::Knight,
        x => if is_capturing_move {
            match "abcdefgh".find(x) {
                None => invalid(),
                Some(_) => PieceType::Pawn
            }
        } else {
            invalid()
        }
    }
}

fn decode_piecetype(notation: &str) -> PieceType {
    let mut chars = notation.chars().rev();
    match chars.nth(2) {
        None => PieceType::Pawn,
        Some(x) => match x {
            'x' => match chars.nth(0) {
                None => panic!("Missing file in pawn capturing move notation: {}", notation),
                Some(y) => decode_piecetype_character(notation, y, true)
            },
            _ => decode_piecetype_character(notation, x, false)
        }
    }
}

fn decode_capture(notation: &str) -> (bool, Option<u8>) {
    let mut chars = notation.chars().rev();
    match chars.nth(2) {
        None => (false, None),
        Some(x) => match x {
            'x' => {
                let next = chars.nth(0);
                (true, decode_from_file(&next))
            },
            _ => (false, None)
        }
    }
}

fn decode_from_file(from_file_character: &Option<char>) -> Option<u8> {
    match from_file_character {
        None => None,
        Some(c) => parse_coordinate(*c, 18, 10)
    }
}

fn clean_notation(notation: &str) -> &str {
    if notation.ends_with("e.p.") {
        return &notation[..notation.len()-4];
    }
    return notation;
}

pub fn decode(notation: String) -> Notation {
    // todo: validate notation only contains low-value utf-8 characters

    let cleaned = clean_notation(&notation);
    let rank = Some(decode_rank(cleaned));
    let file = Some(decode_file(cleaned));
    let (capture, from_file) = decode_capture(cleaned);
    let piece_type = decode_piecetype(cleaned);
    Notation {
        text: notation,
        rank: rank,
        file: file,
        piece_type: piece_type,
        capture: capture,
        from_file: from_file,
        from_rank: None,
        check: false,
        checkmate: false,
        enpassant: false,
        queen_side_castle: false,
        king_side_castle: false,
        promoted_to_piece_type: None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_decode(notation: &str, configure_expected: impl Fn(&mut Notation)) {
        let mut expected = Notation {
            text: notation.to_string(),
            file: Some(0),
            rank: Some(0),
            piece_type: PieceType::Pawn,
            capture: false,
            from_file: None,
            from_rank: None,
            check: false,
            checkmate: false,
            enpassant: false,
            queen_side_castle: false,
            king_side_castle: false,
            promoted_to_piece_type: None
        };
        configure_expected(&mut expected);
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    // suffixes

    #[test]
    fn should_handle_check_symbol() {
        test_decode("a1+", |x| { x.check = true; });
    }

    #[test]
    fn should_handle_checkmate_symbol() {
        test_decode("a1#", |x| { x.checkmate = true; });
    }

    #[test]
    fn should_handle_enpassant_suffix() {
        test_decode("a2e.p.", |x| {
            x.enpassant = true;
            x.rank = Some(1);
        });
    }

    #[test]
    fn should_handle_promotion_suffix() {
        test_decode("e8=Q", |x| {
            x.promoted_to_piece_type = Some(PieceType::Queen);
            x.file = Some(7);
            x.rank = Some(7);
        });
    }

    // castling

    #[test]
    fn should_handle_kingside_castle() {
        test_decode("0-0", |x| { x.king_side_castle = true; });
    }

    #[test]
    fn should_handle_kingside_castle_PGN_variant() {
        test_decode("O-O", |x| { x.king_side_castle = true; });
    }

    #[test]
    fn should_handle_queenside_castle() {
        test_decode("0-0-0", |x| { x.queen_side_castle = true; });
    }

    #[test]
    fn should_handle_queenside_castle_PGN_variant() {
        test_decode("O-O-O", |x| { x.queen_side_castle = true; });
    }

    #[test]
    #[should_panic(expected="Missing file")]
    fn notation_less_than_two_characters_long_should_panic() {
        decode("4".to_string());
    }

    // destination rank and file

    #[test]
    #[should_panic(expected="Invalid rank q")]
    fn non_number_rank_should_panic() {
        decode("q".to_string());
    }

    #[test]
    #[should_panic(expected="Invalid rank 9")]
    fn rank_greater_than_8_should_panic() {
        decode("9".to_string());
    }

    #[test]
    #[should_panic(expected="Invalid rank 0")]
    fn rank_less_than_1_should_panic() {
        decode("0".to_string());
    }

    #[test]
    #[should_panic(expected="Invalid file !")]
    fn non_alpha_file_should_panic() {
        decode("!4".to_string());
    }

    #[test]
    #[should_panic(expected="Invalid file 9")]
    fn file_less_than_a_should_panic() {
        decode("94".to_string());
    }

    #[test]
    #[should_panic(expected="Invalid file i")]
    fn file_more_than_h_should_panic() {
        decode("i4".to_string());
    }

    #[test]
    fn translate_rank_and_file() {
        test_decode("e4", |x| {
            x.rank = Some(3);
            x.file = Some(4);
        });
    }

    #[test]
    fn translate_rank_and_file_upper_bounds() {
        test_decode("h8", |x| {
            x.rank = Some(7);
            x.file = Some(7);
        });
    }

    #[test]
    fn translate_rank_and_file_lower_bounds() {
        test_decode("a1", |x| {
            x.rank = Some(0);
            x.file = Some(0);
        });
    }

    // capturing
    #[test]
    fn note_a_capture_symbol() {
        test_decode("xa1", |x| {
            x.capture = true;
        });
    }

    // source rank and file
    #[test]
    fn note_source_rank() {
        test_decode("bRa1", |x| {
            x.piece_type = PieceType::Rook;
            x.from_rank = Some(1);
        });
    }

    #[test]
    fn note_source_file() {
        test_decode("2Ra1", |x| {
            x.piece_type = PieceType::Rook;
            x.from_file = Some(1);
        });
    }

    #[test]
    fn note_source_rank_and_file() {
        test_decode("c2Qa1", |x| {
            x.piece_type = PieceType::Queen;
            x.from_file = Some(1);
            x.from_rank = Some(2);
        });
    }

    #[test]
    #[should_panic(expected = "Invalid source rank")]
    fn invalid_source_rank() {
        decode("iRa1".to_string());
    }

    #[test]
    #[should_panic(expected = "Invalid source file")]
    fn invalid_source_file() {
        decode("9Ra1".to_string());
    }

    #[test]
    #[should_panic(expected = "Invalid source file")]
    fn invalid_source_rank_and_invalid_source_file() {
        decode("i9Ra1".to_string());
    }

    // moving piece identification

    #[test]
    fn when_not_specified_piece_is_assumed_to_be_a_pawn() {
        test_decode("a1", |x| {
            x.piece_type = PieceType::Pawn
        });
    }

    #[test]
    fn when_specified_piece_it_maps_correctly() {
        for (notation, piece_type) in [
            ("Ka1", PieceType::King),
            ("Qa1", PieceType::Queen),
            ("Ra1", PieceType::Rook),
            ("Ba1", PieceType::Bishop),
            ("Na1", PieceType::Knight)
        ].iter() {
            let actual = decode(notation.to_string());
            assert_eq!(actual.piece_type, *piece_type);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid piece")]
    fn when_specified_piece_is_invalid_should_panic() {
        decode("Se4".to_string());
    }

    // #[test]
    // fn when_a_capture_symbol_is_used_for_a_named_piece() {
    //     test_decode("Kxa1", |x| {
    //         x.piece_type = PieceType::King;
    //         x.capture = true;
    //     });
    // }

    // #[test]
    // #[should_panic(expected="Invalid piece")]
    // fn when_a_capture_symbole_is_used_but_named_piece_is_invalid() {
    //     decode("Zxa1".to_string());
    // }

    // #[test]
    // #[should_panic(expected="Missing file in pawn capturing move")]
    // fn when_a_capture_symbol_is_used_for_a_pawn_but_the_from_file_is_missing() {
    //     decode("xd5".to_string());
    // }

    // #[test]
    // fn when_a_capture_symbol_is_used_for_a_pawn_including_from_file() {
    //     test_decode("bxa1", |x| {
    //         x.piece_type = PieceType::Pawn;
    //         x.capture = true;
    //         x.from_file = Some(1);
    //     });
    // }

    // #[test]
    // fn the_en_passant_suffix_can_be_ignored_for_a_capture() {
    //     test_decode("exd6e.p.", |x| {
    //         x.capture = true;
    //         x.from_file = Some(4);
    //         x.rank = Some(5);
    //         x.file = Some(3);
    //     });
    // }

    // #[test]
    // fn when_necessary_a_disambiguating_file_may_be_specified() {
    //     test_decode("Bbc1", |x| {
    //         x.piece_type = PieceType::Bishop;
    //         x.from_file = Some(2);
    //         x.file = Some(3);
    //     });
    // }
}