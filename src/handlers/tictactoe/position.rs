#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellX {
    One,
    Two,
    Tree,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellY {
    One,
    Two,
    Tree,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellPos {
    x: CellX,
    y: CellY,
}

impl CellPos {
    pub fn new(x: CellX, y: CellY) -> Self {
        Self { x: x.into(), y: y.into() }
    }
    pub fn x(&self) -> &CellX {
        &self.x
    }

    pub fn y(&self) -> &CellY {
        &self.y
    }
}

impl From<&CellX> for usize {
    fn from(value: &CellX) -> Self {
        match value {
            CellX::One => 0,
            CellX::Two => 1,
            CellX::Tree => 2,
        }
    }
}

impl TryFrom<usize> for CellX {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Tree),
            _ => Err(()),
        }
    }
}

impl From<&CellY> for usize {
    fn from(value: &CellY) -> usize {
        match value {
            CellY::One => 0,
            CellY::Two => 1,
            CellY::Tree => 2,
        }
    }
}

impl TryFrom<usize> for CellY {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Tree),
            _ => Err(()),
        }
    }
}
