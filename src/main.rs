use tokio::time::{sleep, Duration};

pub mod model;
pub mod models;

use model::Model;
use models::Wellcome;

const CLEAR: &'static str = "\x1B[2J\x1B[H";

/// Clear the screen
fn clear() {
    print!("{}", CLEAR);
}

#[derive(Debug)]
struct Frame {
    /// Content of Frame
    content: String,
}

impl Frame {
    pub fn new() -> Self {
	Self {
	    content: String::new(),
	}
    }

    /// Push model to Frame content
    fn push_model(&mut self, model: &dyn Model) -> () {
	self.content.push_str(model.draw());
    }
}

type Frames = Vec<Frame>;

struct Animation {
    /// All animation frames, thats will play
    frames: Frames,

    /// How much wait to show the next frame
    frame_delay: u64,
}

impl Animation {
    /// Creates a new Animation object
    pub fn new(frame_delay: u64) -> Self {
	Self {
	    frames: vec![],
	    frame_delay
	}
    }

    fn push_frame(&mut self, frame: Frame) -> () {
	self.frames.push(frame);
    }

    /// Start animation
    async fn start(&self) {
	for frame in &self.frames {
	    println!("{}", frame.content);
	    sleep(Duration::from_millis(self.frame_delay)).await;
	    clear();
	}
    }
}

#[tokio::main]
async fn main() {
    clear();

    let mut frame0 = Frame::new();
    frame0.push_model(&Wellcome);

    let mut anim = Animation::new(3000);

    anim.push_frame(frame0);

    anim.start().await;
}
