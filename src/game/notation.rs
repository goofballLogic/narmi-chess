use super::piece_type::*;

#[derive(Debug, PartialEq)]
pub struct Notation {
    text: String,
    rank: u8,
    file: u8,
    piece_type: PieceType,
    capture: bool,
    pawn_capture_source_file: u8,
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

    let invalid = || panic!("Invalid piece {} in move notation: {}. Must be K, Q, B, R or N, or for a pawn, the source file a..h", piecetype_character, notation);
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

fn decode_capture(notation: &str) -> (bool, u8) {
    let mut chars = notation.chars().rev();
    match chars.nth(2) {
        None => (false, 0),
        Some(x) => match x {
            'x' => {
                let next = chars.nth(0);
                (true, decode_pawn_capture_source_file(&next))
            },
            _ => (false, 0)
        }
    }
}

fn decode_pawn_capture_source_file(source_file_character: &Option<char>) -> u8 {
    match source_file_character {
        None => 0,
        Some(c) => match parse_coordinate(*c, 18, 10) {
            None => 0,
            Some(coordinate) => coordinate
        }
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
    let rank = decode_rank(cleaned);
    let file = decode_file(cleaned);
    let (capture, pawn_capture_source_file) = decode_capture(cleaned);
    let piece_type = decode_piecetype(cleaned);
    Notation {
        text: notation,
        rank: rank,
        file: file,
        piece_type: piece_type,
        capture: capture,
        pawn_capture_source_file: pawn_capture_source_file
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn build_expected(mutation: impl Fn(&mut Notation)) -> Notation {
        let mut result = Notation {
            text: "a1".to_string(),
            file: 0,
            rank: 0,
            piece_type: PieceType::Pawn,
            capture: false,
            pawn_capture_source_file: 0
        };
        mutation(&mut result);
        result
    }

    #[test]
    #[should_panic(expected="Missing file")]
    fn notation_less_than_two_characters_long_should_panic() {
        decode("4".to_string());
    }

    // rank tests

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

    // file tests

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
        let notation = "e4";
        let expected = build_expected(|x| {
            x.text = notation.to_string();
            x.rank = 3;
            x.file = 4;
            x.piece_type = PieceType::Pawn;
        });

        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn translate_rank_and_file_upper_bounds() {
        let notation = "h8";
        let expected = build_expected(|mut x| {
            x.text = notation.to_string();
            x.rank = 7;
            x.file = 7;
            x.piece_type = PieceType::Pawn;
        });
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn translate_rank_and_file_lower_bounds() {
        let notation = "a1";
        let expected = build_expected(|mut x| {
            x.text = notation.to_string();
            x.rank = 0;
            x.file = 0;
            x.piece_type = PieceType::Pawn;
        });
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn when_not_specified_piece_is_assumed_to_be_a_pawn() {
        let notation = "a1";
        let actual = decode(notation.to_string());
        assert_eq!(actual.piece_type, PieceType::Pawn);
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
    fn when_a_capture_symbol_is_used_for_a_named_piece() {
        let notation = "Kxa1";
        let expected = build_expected(|mut x| {
            x.text = notation.to_string();
            x.piece_type = PieceType::King;
            x.capture = true;
        });
        let actual = decode(notation.to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected="Invalid piece")]
    fn when_a_capture_symbole_is_used_but_named_piece_is_invalid() {
        decode("Zxa1".to_string());
    }

    #[test]
    #[should_panic(expected="Missing file in pawn capturing move")]
    fn when_a_capture_symbol_is_used_for_a_pawn_but_the_source_file_is_missing() {
        decode("xd5".to_string());
    }

    #[test]
    fn when_a_capture_symbol_is_used_for_a_pawn_including_source_file() {
        let notation = "exd5";
        let expected = build_expected(|mut x| {
            x.text = notation.to_string();
            x.piece_type = PieceType::Pawn;
            x.capture = true;
            x.pawn_capture_source_file = 4;
            x.rank = 4;
            x.file = 3;
        });
        let actual = decode(notation.to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn the_en_passant_suffix_can_be_ignored_for_a_capture() {
        let notation = "exd5e.p.";
        let expected = build_expected(|mut x | {
            x.text = notation.to_string();
            x.piece_type = PieceType::Pawn;
            x.capture = true;
            x.pawn_capture_source_file = 4;
            x.rank = 4;
            x.file = 3;
        });
        let actual = decode(notation.to_string());
        assert_eq!(actual, expected);
    }
}