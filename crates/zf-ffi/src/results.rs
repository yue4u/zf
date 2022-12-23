pub use bincode::*;

#[derive(Decode, Encode, Debug, PartialEq)]
pub enum CommandResults {
    Levels(Vec<String>),
    Empty,
}
