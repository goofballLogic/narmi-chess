/*
1.2	The player with the light-coloured pieces (White) makes the first move,
then the players move alternately, with the player with the dark-coloured pieces (Black) making the next move.
*/

use super::rule::Rule;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation {
        Implementation {}
    }
}

impl Rule for Implementation {}
