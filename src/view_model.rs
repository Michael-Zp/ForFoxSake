use cgmath;

pub struct ViewModel
{
    pub sprite_sizes: [cgmath::Vector2<f32>;10], 
    pub sprite_positions: [cgmath::Vector2<f32>;10], 
    pub sprite_tile_map_indices: [i32;10], 
    pub sprite_count: i32,
}