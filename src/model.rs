/// For draw on the terminal
pub trait Model {
    /// Draw the frame of Model
    fn draw(&self) -> &str;
}
