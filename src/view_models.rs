use cgmath;

pub struct SpritesViewModel
{
    pub sprite_sizes: [cgmath::Vector2<f32>;10], 
    pub sprite_positions: [cgmath::Vector2<f32>;10], 
    pub sprite_tile_map_indices: [i32;10], 
    pub sprite_count: i32,
}

pub struct LevelViewModel
{
    pub data: std::vec::Vec<i32>,
    pub width: f32,
    pub height: f32,
}