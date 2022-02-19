use crate::model::Model;

pub struct Wellcome;

impl Model for Wellcome {
    fn draw(&self) -> &str {
	return "Wellcome";
    }
}
