
use super::end_of_game_type::*;
use super::piece_type::*;
pub struct NotationError {
    pub reason: String,
}

#[derive(Debug, PartialEq)]
pub struct Notation {
    text: String,
    to_rank: Option<u8>,
    to_file: Option<u8>,
    piece_type: Option<PieceType>,
    capture: bool,
    from_file: Option<u8>,
    from_rank: Option<u8>,
    check: bool,
    checkmate: bool,
    enpassant: bool,
    queen_side_castle: bool,
    king_side_castle: bool,
    promoted_to_piece_type: Option<PieceType>,
    end_of_game: Option<EndOfGameType>,
}

impl Notation {

    fn invalid(&self, why: &str) -> Result<(), NotationError> {
        Err(NotationError {
            reason: why.to_string(),
        })
    }

    fn validate_parsing(&self) -> Result<(), NotationError> {

        if !(self.queen_side_castle || self.king_side_castle || self.end_of_game.is_some()) {
            match (self.to_rank, self.to_file) {
                (None, None) => self.invalid("Both rank and file are missing (or invalid)"),
                (None, _) => self.invalid("Rank is missing (or invalid)"),
                (_, None) => self.invalid("File is missing (or invalid)"),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }
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

fn decode_piecetype(piecetype_character: &str) -> Option<PieceType> {
    match piecetype_character {
        "K" => Some(PieceType::King),
        "Q" => Some(PieceType::Queen),
        "B" => Some(PieceType::Bishop),
        "R" => Some(PieceType::Rook),
        "N" => Some(PieceType::Knight),
        _ => None,
    }
}

fn process_piece_type(notation: &str) -> (Option<PieceType>, &str) {
    match notation.len() {
        0 => (Some(PieceType::Pawn), notation),
        _ => match decode_piecetype(&notation[notation.len() - 1..]) {
            None => (Some(PieceType::Pawn), notation),
            some_piece => (some_piece, &notation[..notation.len() - 1]),
        },
    }
}

fn process_capture(notation: &str) -> (bool, &str) {
    match notation.ends_with("x") {
        true => (true, &notation[..notation.len() - 1]),
        false => (false, notation),
    }
}

fn parse_and_trim_coordinate_suffix(
    last_char: Option<char>,
    notation: &str,
    parse_radix: u32,
    parse_offset: u32,
) -> (Option<u8>, &str) {
    match last_char {
        Some(x) => match parse_coordinate(x, parse_radix, parse_offset) {
            None => (None, notation),
            some_parsed_coordinate => (some_parsed_coordinate, &notation[..notation.len() - 1]),
        },
        None => (None, notation),
    }
}

fn process_coordinates(notation: &str) -> (Option<u8>, Option<u8>, &str) {
    // destination is always file, rank
    let mut chars = notation.chars().rev();
    let rank_char = chars.nth(0);
    let (rank, without_rank) = parse_and_trim_coordinate_suffix(rank_char, notation, 10, 1);
    let file_char = if rank.is_some() {
        chars.nth(0)
    } else {
        rank_char
    };
    let (file, without_file_and_rank) =
        parse_and_trim_coordinate_suffix(file_char, without_rank, 18, 10);
    (rank, file, without_file_and_rank)
}

fn process_promotion(notation: &str) -> (Option<PieceType>, &str) {
    match notation.len() {
        0 | 1 => (None, notation),
        len => match &notation[len - 2..len - 1] {
            "=" => (decode_piecetype(&notation[len - 1..]), &notation[..len - 2]),
            _ => (None, notation),
        },
    }
}

fn process_castling(notation: &str) -> (bool, bool) {
    match notation {
        "0-0-0" => (false, true),
        "O-O-O" => (false, true),
        "0-0" => (true, false),
        "O-O" => (true, false),
        _ => (false, false),
    }
}

fn process_suffix(notation: &str) -> (bool, bool, bool, &str) {
    let mut check = false;
    let mut checkmate = false;
    let mut enpassant = false;
    let mut working = notation;
    let mut complete = false;
    while !complete {
        if working.ends_with("e.p.") {
            enpassant = true;
            working = &working[..working.len() - 4];
            continue;
        }
        if working.ends_with("+") {
            check = true;
            working = &working[..working.len() - 1];
            continue;
        }
        if working.ends_with("#") {
            checkmate = true;
            working = &working[..working.len() - 1];
            continue;
        }
        complete = true;
    }
    (checkmate, check, enpassant, working)
}

fn process_end_of_game(notation: &str) -> Option<EndOfGameType> {
    match notation {
        "1-0" => Some(EndOfGameType::WhiteWin),
        "0-1" => Some(EndOfGameType::BlackWin),
        "½–½" => Some(EndOfGameType::Draw),
        _ => None,
    }
}

pub fn decode(notation: String) -> Result<Notation, NotationError> {

    // todo: validate notation only contains low-value utf-8 characters

    let mut parsed = Notation {
        text: "".to_string(), // assigned after parsing
        to_rank: None,
        to_file: None,
        piece_type: None,
        capture: false,
        from_rank: None,
        from_file: None,
        check: false,
        checkmate: false,
        enpassant: false,
        queen_side_castle: false,
        king_side_castle: false,
        promoted_to_piece_type: None,
        end_of_game: None,
    };
    let (king_side_castle, queen_side_castle) = process_castling(&notation);
    let end_of_game = process_end_of_game(&notation);
    if king_side_castle || queen_side_castle || end_of_game.is_some() {
        parsed.king_side_castle = king_side_castle;
        parsed.queen_side_castle = queen_side_castle;
        parsed.end_of_game = end_of_game;
    } else {
        let (checkmate, check, enpassant, ex_suffix) = process_suffix(&notation);
        let (promoted_to_piece_type, ex_promotion) = process_promotion(ex_suffix);
        let (rank, file, ex_destination) = process_coordinates(ex_promotion);
        let (capture, ex_capture) = process_capture(ex_destination);
        let (piece_type, ex_piece_type) = process_piece_type(ex_capture);
        let (from_rank, from_file, ex_from_coordinates) = process_coordinates(ex_piece_type);
        match ex_from_coordinates {
            "" => {
                parsed.to_rank = rank;
                parsed.to_file = file;
                parsed.piece_type = piece_type;
                parsed.capture = capture;
                parsed.from_rank = from_rank;
                parsed.from_file = from_file;
                parsed.check = check;
                parsed.checkmate = checkmate;
                parsed.enpassant = enpassant;
                parsed.promoted_to_piece_type = promoted_to_piece_type;
            }
            _ => {
                return Err(NotationError {
                    reason: format!("Invalid notation: {}", notation),
                });
            }
        }
    }
    parsed.text = notation;
    match parsed.validate_parsing() {
        Ok(()) => Ok(parsed),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_decode(notation: &str, configure_expected: impl Fn(&mut Notation)) {
        let mut expected = Notation {
            text: notation.to_string(),
            to_file: Some(0),
            to_rank: Some(0),
            piece_type: Some(PieceType::Pawn),
            capture: false,
            from_file: None,
            from_rank: None,
            check: false,
            checkmate: false,
            enpassant: false,
            queen_side_castle: false,
            king_side_castle: false,
            promoted_to_piece_type: None,
            end_of_game: None,
        };
        configure_expected(&mut expected);
        let actual = decode(notation.to_string()).unwrap_or_else(|err| {
            panic!(err);
        });
        assert_eq!(expected, actual);
    }

    fn test_decode_failure(notation: &str, expected_failure_reason: &str) {
        let actual = decode(notation.to_string());
        match actual {
            Ok(_) => {
                panic!("Notation parsed without error");
            }
            Err(err) => {
                assert_eq!(err.reason, expected_failure_reason);
            }
        }
    }

    // castling

    #[test]
    fn should_handle_kingside_castle() {
        test_decode("0-0", |x| {
            x.king_side_castle = true;
            x.piece_type = None;
            x.to_file = None;
            x.to_rank = None;
        });
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_handle_kingside_castle_PGN_variant() {
        test_decode("O-O", |x| {
            x.king_side_castle = true;
            x.piece_type = None;
            x.to_file = None;
            x.to_rank = None;
        });
    }

    #[test]
    fn should_handle_queenside_castle() {
        test_decode("0-0-0", |x| {
            x.queen_side_castle = true;
            x.piece_type = None;
            x.to_file = None;
            x.to_rank = None;
        });
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_handle_queenside_castle_PGN_variant() {
        test_decode("O-O-O", |x| {
            x.queen_side_castle = true;
            x.piece_type = None;
            x.to_file = None;
            x.to_rank = None;
        });
    }

    #[test]
    fn should_fail_for_castling_with_invalid_characters() {
        test_decode_failure("xO-O-O", "Invalid notation: xO-O-O");
    }

    #[test]
    fn should_record_end_of_game_white_win() {
        test_decode("1-0", |x| {
            x.end_of_game = Some(EndOfGameType::WhiteWin);
            x.piece_type = None;
            x.to_rank = None;
            x.to_file = None;
        });
    }

    #[test]
    fn should_record_end_of_game_black_win() {
        test_decode("0-1", |x| {
            x.end_of_game = Some(EndOfGameType::BlackWin);
            x.piece_type = None;
            x.to_rank = None;
            x.to_file = None;
        });
    }

    #[test]
    fn should_record_end_of_game_draw() {
        test_decode("½–½", |x| {
            x.end_of_game = Some(EndOfGameType::Draw);
            x.piece_type = None;
            x.to_rank = None;
            x.to_file = None;
        });
    }


    // check/check-mate/en-passant

    #[test]
    fn should_handle_check_symbol() {
        test_decode("a1+", |x| {
            x.check = true;
        });
    }

    #[test]
    fn should_handle_checkmate_symbol() {
        test_decode("a1#", |x| {
            x.checkmate = true;
        });
    }

    #[test]
    fn should_handle_enpassant_suffix() {
        test_decode("a2e.p.", |x| {
            x.enpassant = true;
            x.to_rank = Some(1);
        });
    }

    #[test]
    #[allow(non_snake_case)]
    fn should_handle_promotion_suffix_PGN() {
        test_decode("e8=Q", |x| {
            x.promoted_to_piece_type = Some(PieceType::Queen);
            x.to_file = Some(4);
            x.to_rank = Some(7);
        });
    }

    //destination rank and file

    #[test]
    fn translate_rank_and_file() {
        test_decode("e4", |x| {
            x.to_rank = Some(3);
            x.to_file = Some(4);
        });
    }

    #[test]
    fn translate_rank_and_file_upper_bounds() {
        test_decode("h8", |x| {
            x.to_rank = Some(7);
            x.to_file = Some(7);
        });
    }

    #[test]
    fn translate_rank_and_file_lower_bounds() {
        test_decode("a1", |x| {
            x.to_rank = Some(0);
            x.to_file = Some(0);
        });
    }

    #[test]
    fn notation_less_than_two_characters_long_should_fail() {
        test_decode_failure("4", "File is missing (or invalid)");
    }

    #[test]
    fn non_number_rank_should_fail() {
        test_decode_failure("q", "Invalid notation: q");
    }

    #[test]
    fn rank_greater_than_8_should_fail() {
        test_decode_failure("9", "Invalid notation: 9");
    }

    #[test]
    fn rank_less_than_1_should_fail() {
        test_decode_failure("0", "Invalid notation: 0");
    }

    #[test]
    fn non_alpha_file_should_fail() {
        test_decode_failure("!4", "Invalid notation: !4");
    }

    #[test]
    fn file_less_than_a_should_fail() {
        test_decode_failure("94", "Invalid notation: 94");
    }

    #[test]
    fn file_more_than_h_should_fail() {
        test_decode_failure("i4", "Invalid notation: i4");
    }

    // capturing
    #[test]
    fn note_a_capture_symbol() {
        test_decode("xa1", |x| {
            x.capture = true;
        });
    }

    // moving piece identification

    #[test]
    fn when_not_specified_piece_is_assumed_to_be_a_pawn() {
        test_decode("a1", |x| x.piece_type = Some(PieceType::Pawn));
    }

    #[test]
    fn when_specified_piece_it_maps_correctly() {
        for (notation, piece_type) in [
            ("Ka1", PieceType::King),
            ("Qa1", PieceType::Queen),
            ("Ra1", PieceType::Rook),
            ("Ba1", PieceType::Bishop),
            ("Na1", PieceType::Knight),
        ]
        .iter()
        {
            let actual = decode(notation.to_string()).unwrap_or_else(|err| {
                panic!(err);
            });
            assert_eq!(actual.piece_type.unwrap(), *piece_type)
        }
    }

    #[test]
    fn when_specified_piece_is_invalid_should_fail() {
        test_decode_failure("Se4", "Invalid notation: Se4");
    }

    // source rank and file
    #[test]
    fn note_from_file() {
        test_decode("bRa1", |x| {
            x.piece_type = Some(PieceType::Rook);
            x.from_file = Some(1);
        });
    }

    #[test]
    fn note_from_rank() {
        test_decode("2Ra1", |x| {
            x.piece_type = Some(PieceType::Rook);
            x.from_rank = Some(1);
        });
    }

    #[test]
    fn note_from_rank_and_file() {
        test_decode("c2Qa1", |x| {
            x.piece_type = Some(PieceType::Queen);
            x.from_rank = Some(1);
            x.from_file = Some(2);
        });
    }

    #[test]
    fn invalid_from_rank() {
        test_decode_failure("iRa1", "Invalid notation: iRa1");
    }

    #[test]
    fn invalid_from_file() {
        test_decode_failure("9Ra1", "Invalid notation: 9Ra1");
    }

    #[test]
    fn invalid_from_rank_and_invalid_from_file() {
        test_decode_failure("i9Ra1", "Invalid notation: i9Ra1");
    }

}