/*
1.3	A player is said to ‘have the move’ when his opponent’s move has been ‘made’.
*/

use super::rule::Rule;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation {
        Implementation {}
    }
}

impl Rule for Implementation {}
