# TermAnim
Animation in terminal lib for Rust


# Example usage
``` rust
use termanim::{animation::{Animation, Frame}, model::Model, action::Action};

struct Wellcome;
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


#[tokio::main]
async fn main() {
    let mut animation = Animation::new(1000);

    animation.push_model(Wellcome);

    animation.start().await;
}

```
