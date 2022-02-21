use crate::Frames;

pub struct Action {
    /// Name of Action
    pub name: String,

    /// Frames of Action
    pub frames: Frames,
}

impl Action {
    /// Creates Action
    pub fn new(name: String, frames: Frames) -> Self {
	Self {
	    name,
	    frames,
	}
    }
}
