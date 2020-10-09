use crate::model::model_utils::{GridPosition, grid_to_position};
use cgmath;

pub struct FoxHole<T>
{
    pub entry: T,
    pub exit: T,
    pub used: bool,
}

impl<T> FoxHole<T>
{
    pub fn new(entry: T, exit: T, used: Option<bool>) -> FoxHole<T>
    {
        FoxHole { entry: entry, exit: exit, used: used.unwrap_or(false)}
    }
}


impl FoxHole<GridPosition>
{
    pub fn from(grid_hole: &FoxHole<GridPosition>, width: f32, height: f32) -> FoxHole<cgmath::Vector2<f32>>
    {
        let entry_pos = grid_to_position(&grid_hole.entry, width, height);
        let exit_pos = grid_to_position(&grid_hole.exit, width, height);

        FoxHole{ entry: entry_pos, exit: exit_pos, used: false }
    }
}