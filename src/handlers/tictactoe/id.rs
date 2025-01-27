use super::position::{CellPos, CellX, CellY};

pub struct GridButtonId {
    game_id: super::game::GameId,
    cell_position: super::position::CellPos,
}

impl GridButtonId {
    pub fn new(game_id: super::game::GameId, cell_position: super::position::CellPos) -> Self {
        Self {
            game_id,
            cell_position,
        }
    }

    pub fn game_id(&self) -> &super::game::GameId {
        &self.game_id
    }
    
    pub fn cell_position(&self) -> &super::position::CellPos{
        &self.cell_position    
    }
}

impl From<GridButtonId> for String {
    fn from(value: GridButtonId) -> Self {
        format!(
            "{},{},{}",
            value.game_id.to_string(),
            usize::from(value.cell_position.x()).to_string(),
            usize::from(value.cell_position.y()).to_string()
        )
    }
}

impl TryFrom<&String> for GridButtonId {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut split = value.split(",").map(ToOwned::to_owned);

        let Some(id) = split.next().and_then(|s| {
            s.parse()
                .map(|id| unsafe { super::game::GameId::new_unchecked(id) })
                .ok()
        }) else {
            return Err(());
        };

        let Some(x) = split.next().and_then(|s| {
            s.parse::<usize>()
                .map_err(|_| ())
                .and_then(|n| CellX::try_from(n))
                .ok()
        }) else {
            return Err(());
        };
        let Some(y) = split.next().and_then(|s| {
            s.parse::<usize>()
                .map_err(|_| ())
                .and_then(|n| CellY::try_from(n))
                .ok()
        }) else {
            return Err(());
        };

        Ok(Self::new(id, CellPos::new(x, y)))
    }
}
