use crate::model::model_utils;

pub struct FoxHoleInGrid
{
    pub entry: model_utils::GridPosition,
    pub exit: model_utils::GridPosition,
}

pub struct FoxHoleWithPosition
{
    pub entry: cgmath::Vector2<f32>,
    pub exit: cgmath::Vector2<f32>,
}

impl FoxHoleWithPosition
{
    pub fn from(grid_hole: &FoxHoleInGrid, width: f32, height: f32) -> FoxHoleWithPosition
    {
        let entry_pos = model_utils::grid_to_position(&grid_hole.entry, width, height);
        let exit_pos = model_utils::grid_to_position(&grid_hole.exit, width, height);

        FoxHoleWithPosition{ entry: entry_pos, exit: exit_pos }
    }
}