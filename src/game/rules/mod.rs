use std::vec::Vec;

use crate::game::rules::rule::Rule;

mod rule;
mod rules_1_2_who_s_go_is_it;
mod rules_1_3_has_a_move_been_made;
mod rules_1_4_cant_move_after_checkmate;
mod rules_1_5_stalemate;
mod rules_2_1_the_board;

pub fn build_rules() -> Vec<Box<Rule>> {
    vec![
        Box::new(rules_1_2_who_s_go_is_it::Implementation::new()),
        Box::new(rules_1_3_has_a_move_been_made::Implementation::new()),
        Box::new(rules_1_4_cant_move_after_checkmate::Implementation::new()),
        Box::new(rules_1_5_stalemate::Implementation::new()),
        Box::new(rules_2_1_the_board::Implementation::new())
    ]
}