use crate::model::Model;
use std::fmt::Display;
use tokio::time::{sleep, Duration};

const CLEAR: &'static str = "\x1B[2J\x1B[H";

/// Clear the screen
fn clear() {
    print!("{}", CLEAR);
}

pub struct Frame {
    /// Content of Frame
    pub content: Box<dyn Display>,
}

impl Frame {
    pub fn new<T>(content: &'static T) -> Self
    where
        T: Display,
    {
        Self {
            content: Box::new(content),
        }
    }
}

pub type Frames = Vec<Frame>;

pub struct Animation {
    /// All animation frames, thats will play
    frames: Frames,

    /// How much wait to show the next frame
    /// As miliseconds
    frame_delay: u64,
}

impl Animation {
    /// Creates a new Animation object
    pub fn new(frame_delay: u64) -> Self {
        Self {
            frames: vec![],
            frame_delay,
        }
    }

    pub fn push_frame(&mut self, frame: Frame) -> () {
        self.frames.push(frame);
    }

    /// Push all model frames
    pub fn push_model<T>(&mut self, model: T) -> ()
    where
        T: Model,
    {
        for action in model.actions() {
            for frame in action.frames {
                self.frames.push(frame);
            }
        }
    }

    /// Start animation
    pub async fn start(&self) {
        // Clear terminal to start animation
        clear();
        for frame in &self.frames {
            println!("{}", frame.content);
            sleep(Duration::from_millis(self.frame_delay)).await;
            clear();
        }
    }
}
