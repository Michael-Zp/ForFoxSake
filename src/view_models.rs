use cgmath;

pub struct SpritesViewModel
{
    pub sizes: [cgmath::Vector2<f32>;10], 
    pub positions: [cgmath::Vector2<f32>;10], 
    pub tile_map_indices: [i32;10], 
    pub count: i32,
}

pub struct LevelViewModel
{
    pub data: std::vec::Vec<i32>,
    pub width: f32,
    pub height: f32,
}

pub struct ParticlesViewModel
{
    pub positions: [cgmath::Vector2<f32>;10],
    pub max_speeds: [f32;10],
    pub running_times: [f32;10],
    pub max_running_times: [f32;10],
    pub count: i32
}