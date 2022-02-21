use crate::action::Action;
use crate::model::Model;
use crate::Frame;

pub struct Wellcome;

impl Model for Wellcome {
    fn actions(&self) -> Vec<Action> {
	vec![
	    Action {
		name: String::from("Intro"),
		
		frames: vec![
		    Frame {
			content: String::from("Hello world")
		    },
		    
		    Frame {
			content: String::from("This is a animation")
		    }
		]
	    }
	]
    }
}
