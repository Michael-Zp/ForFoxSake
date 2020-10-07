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
        ViewModel {
            player_pos: self.player_pos,
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
        const speed: f32 = 0.2;

        if input.is_input_down(&format!("MoveLeft")) || input.is_input_pressed(&format!("MoveLeft"))
        {
            self.player_pos.x -= speed * delta_time;
        }
        
        if input.is_input_down(&format!("MoveRight")) || input.is_input_pressed(&format!("MoveRight"))
        {
            self.player_pos.x += speed * delta_time;
        }

        if input.is_input_down(&format!("MoveDown")) || input.is_input_pressed(&format!("MoveDown"))
        {
            self.player_pos.y -= speed * delta_time;
        }

        if input.is_input_down(&format!("MoveUp")) || input.is_input_pressed(&format!("MoveUp"))
        {
            self.player_pos.y += speed * delta_time;
        }
    }
}