#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellX{
    One, Two, Tree
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellY{
    One, Two, Tree
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellPos{
    x: CellX,
    y: CellY
}
