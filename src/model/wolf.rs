use crate::model::model_utils::{GridPosition, grid_to_position};

pub struct Wolf<T>
{
    pub pos: T,
}


impl Wolf<GridPosition>
{
    pub fn from(grid_hole: &Wolf<GridPosition>, width: f32, height: f32) -> Wolf<cgmath::Vector2<f32>>
    {
        let pos = grid_to_position(&grid_hole.pos, width, height);

        Wolf{ pos: pos, }
    }
}