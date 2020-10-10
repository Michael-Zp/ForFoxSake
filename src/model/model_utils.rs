pub struct GridPosition
{
    pub column: i32,
    pub row: i32,
}

pub fn grid_to_position(grid_pos: &GridPosition, width: f32, height: f32) -> cgmath::Vector2<f32>
{

    let tile_width = 2.0 / width;
    let tile_height = 2.0 / height;
    cgmath::Vector2 { x: -1.0 + tile_width  / 2.0 + grid_pos.column as f32 * tile_width, 
                      y:  1.0 - tile_height / 2.0 - grid_pos.row as f32    * tile_height }
}


pub struct SpriteAnimationMetaData
{
    pub from_index: i32,
    pub to_index: i32,
    pub timeout: f32,
}

impl std::fmt::Display for SpriteAnimationMetaData
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({}, {}, {})", self.from_index, self.to_index, self.timeout)
    }
}