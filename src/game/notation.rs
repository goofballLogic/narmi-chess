#[derive(Debug, PartialEq)]
pub struct Notation {
    text: String,
    rank: u8,
    file: u8
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

pub fn decode(notation: String) -> Notation {
    // todo: validate notation only contains low-value utf-8 characters
    let rank = decode_rank(&notation);
    let file = decode_file(&notation);
    Notation {
        text: notation.to_string(),
        rank: rank,
        file: file
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn notation_less_than_two_characters_long_should_throw() {
        let notation = "4";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Missing file"));
    }

    // rank tests

    #[test]
    fn non_number_rank_should_throw() {
        let notation = "q";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid rank q"));
    }

    #[test]
    fn rank_greater_than_8_should_throw() {
        let notation = "9";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid rank 9"));
    }

    #[test]
    fn rank_less_than_1_should_throw() {
        let notation = "0";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid rank 0"));
    }

    // file tests

    #[test]
    fn non_alpha_file_should_throw() {
        let notation = "!4";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid file !"));
    }

    #[test]
    fn file_less_than_a_should_throw() {
        let notation = "94";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid file 9"));
    }

    #[test]
    fn file_more_than_h_should_throw() {
        let notation = "i4";
        let actual = std::panic::catch_unwind(|| decode(notation.to_string()));
        let err = *(actual.unwrap_err().downcast::<String>().unwrap());
        assert!(err.contains("Invalid file i"));
    }

    #[test]
    fn translate_rank_and_file() {
        let notation = "e4";
        let expected = Notation {
            text: notation.to_string(),
            rank: 3,
            file: 4
        };
        let actual = decode(notation.to_string());
        assert_eq!(expected, actual);
    }
}