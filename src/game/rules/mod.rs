use crate::game::game::*;
use crate::game::moving::*;

mod rules_1_2_who_s_go_is_it;

type Validate = fn(&Game, String) -> Result<(), MoveError>;

pub const VALIDATIONS: &[Validate] = &[
    rules_1_2_who_s_go_is_it::validate
];
