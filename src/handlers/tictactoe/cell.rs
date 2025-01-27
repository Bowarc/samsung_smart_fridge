#[derive(Default, Debug, Copy, Clone)]
pub enum State {
    #[default]
    Empty,
    P1,
    P2,
}

impl State {
    fn emoji(&self) -> &str {
        match self {
            State::Empty => ":white_large_square:",
            State::P1 => ":regional_indicator_x:",
            State::P2 => ":o2:",
        }
    }
}

impl From<&State> for String {
    fn from(state: &State) -> String {
        match state {
            State::Empty => String::from("."),
            State::P1 => String::from("X"),
            State::P2 => String::from("O"),
        }
    }
}
