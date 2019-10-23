use super::piece_type::*;

#[derive(Debug, PartialEq)]
pub struct Notation {
    text: String,
    rank: u8,
    file: u8,
    piece_type: PieceType,
}

fn decode_coordinate(notation: &str, reverse_index: usize, parse_radix: u32, parse_offset: u32, coordinate_name: &str, valid_range: &str) -> u8 {

    // some panics that will come in handy
    let invalid = | what: &str | panic!("{} in move notation: {}. Must be {}", what, notation, valid_range);
    let invalid_coordinate = | c: char | invalid(&format!("Invalid {} {}", coordinate_name, c));

    // try to pick the character which is nth from the end
    match notation.chars().rev().nth(reverse_index) {

        // if we didn't find any thing in the required position, bail out:
        None => invalid(&format!("Missing {}", coordinate_name)),

        /*
            If we did find a character in the needed position:
                Try to parse the character to 0..7
                If it's a rank (a..h) then we'll want to treat it as base 18 (0..h) and subtract 10
                If it's a file (1..8) then we'll want to treat it as base 10 (0..9) and subtract 1
        */
        Some(c) => match c.to_digit(parse_radix) {

            // if parsing succeeds:
            Some(x) => {

                // is it out of range?
                if (x < parse_offset) || (x > parse_offset + 7) { invalid_coordinate(c); }

                // return the result
                (x - parse_offset) as u8
            },

            // if it was not parseable as a digit
            None => invalid_coordinate(c),
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

fn decode_piecetype(notation: &str) -> PieceType {

    let invalid = | what: &str | panic!("Invalid piece {} in move notation: {}. Must be none, K, Q, R, B or N", what, notation);
    match notation.chars().rev().nth(2) {
        None => PieceType::Pawn,
        Some(x) => match x {
            'K' => PieceType::King,
            'Q' => PieceType::Queen,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'N' => PieceType::Knight,
            _ => invalid(&x.to_string())
        }
    }
}

pub fn decode(notation: String) -> Notation {
    // todo: validate notation only contains low-value utf-8 characters
    let rank = decode_rank(&notation);
    let file = decode_file(&notation);
    let piece_type = decode_piecetype(&notation);
    Notation {
        text: notation.to_string(),
        rank: rank,
        file: file,
        piece_type: piece_type
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
        let expected = Notation {
            text: notation.to_string(),
            rank: 3,
            file: 4,
            piece_type: PieceType::Pawn
        };
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn translate_rank_and_file_upper_bounds() {
        let notation = "h8";
        let expected = Notation {
            text: notation.to_string(),
            rank: 7,
            file: 7,
            piece_type: PieceType::Pawn
        };
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn translate_rank_and_file_lower_bounds() {
        let notation = "a1";
        let expected = Notation {
            text: notation.to_string(),
            rank: 0,
            file: 0,
            piece_type: PieceType::Pawn
        };
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
}