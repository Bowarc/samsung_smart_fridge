#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    #[default]
    Empty,
    P1,
    P2,
}

impl From<&State> for String {
    fn from(state: &State) -> String {
        match state {
            State::Empty => String::from(crate::INVISIBLE_CHARACTER),
            State::P1 => String::from("X"),
            State::P2 => String::from("O"),
        }
    }
}
