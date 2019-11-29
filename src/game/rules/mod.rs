use std::vec::Vec;

use crate::game::rules::rule::Rule;

mod rules_1_2_who_s_go_is_it;
mod rules_1_3_has_a_move_been_made;
mod rules_1_4_cant_move_after_check;
mod rule;

pub fn build_rules() -> Vec<Box<Rule>> {
    vec![
        Box::new(rules_1_2_who_s_go_is_it::Implementation::new()),
        Box::new(rules_1_3_has_a_move_been_made::Implementation::new()),
        Box::new(rules_1_4_cant_move_after_check::Implementation::new())
    ]
}

// pub const VALIDATIONS: &[Validate] = &[
//     rules_1_2_who_s_go_is_it::validate,
    //&rules_1_3_has_a_move_been_made::Implementation::validate,
    //&rules_1_4_cant_move_after_check::Implementation::validate
//];
