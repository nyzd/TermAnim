use crate::action::Action;

pub trait Model {
    fn actions(&self) -> Vec<Action>;
}
