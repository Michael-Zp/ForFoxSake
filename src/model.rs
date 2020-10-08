use crate::for_fox_sake::read_only_input::ReadOnlyInput;
use crate::view_model::ViewModel;
use cgmath;
pub mod level;

pub struct Model 
{
    player_pos: cgmath::Vector2<f32>,
}

impl Model
{
    pub fn new() -> Result<Model, String>
    {
        Ok(Model{ 
            player_pos: cgmath::Vector2 { x: 0.0, y: 0.0 } 
        })
    }

    pub fn to_view_model(&self) -> ViewModel
    {
        let mut sprite_sizes: [cgmath::Vector2<f32>;10] = [cgmath::Vector2{ x: 0.0, y: 0.0 };10];
        sprite_sizes[0] = cgmath::Vector2{ x: 0.2, y: 0.2 };
        sprite_sizes[1] = cgmath::Vector2{ x: 0.2, y: 0.2 };
        sprite_sizes[2] = cgmath::Vector2{ x: 0.2, y: 0.2 };
        
        let mut sprite_positions: [cgmath::Vector2<f32>;10] = [cgmath::Vector2{ x: 0.0, y: 0.0 };10];
        sprite_positions[0] = cgmath::Vector2{ x: -0.5, y: 0.0 };
        sprite_positions[1] = cgmath::Vector2{ x: -0.5, y: 0.0 };
        sprite_positions[2] = self.player_pos;
        
        let mut sprite_tile_map_indices: [i32;10] = [0;10];
        sprite_tile_map_indices[0] = 1;
        sprite_tile_map_indices[1] = 2;
        sprite_tile_map_indices[2] = 0;

        ViewModel {
            sprite_sizes: sprite_sizes,
            sprite_positions: sprite_positions,
            sprite_tile_map_indices: sprite_tile_map_indices,
            sprite_count: 3,
        }
    }

    pub fn get_level(&self, level_code: u8) -> Result<level::Level, String>
    {
        match level_code
        {
            0 => Ok(level::LEVEL0),
            _ => Err(format!("Level not found")),
        }
    }

    pub fn update(&mut self, input: ReadOnlyInput, delta_time: f32)
    {
        const SPEED: f32 = 0.2;

        if input.is_input_down(&format!("MoveLeft")) || input.is_input_pressed(&format!("MoveLeft"))
        {
            self.player_pos.x -= SPEED * delta_time;
        }
        
        if input.is_input_down(&format!("MoveRight")) || input.is_input_pressed(&format!("MoveRight"))
        {
            self.player_pos.x += SPEED * delta_time;
        }

        if input.is_input_down(&format!("MoveDown")) || input.is_input_pressed(&format!("MoveDown"))
        {
            self.player_pos.y -= SPEED * delta_time;
        }

        if input.is_input_down(&format!("MoveUp")) || input.is_input_pressed(&format!("MoveUp"))
        {
            self.player_pos.y += SPEED * delta_time;
        }
    }
}