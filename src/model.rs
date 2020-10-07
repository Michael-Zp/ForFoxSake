use crate::for_fox_sake::read_only_input::ReadOnlyInput;
pub mod level;

pub struct Model 
{
    
}

impl Model
{
    pub fn new() -> Result<Model, String>
    {
        Ok(Model{ })
    }

    pub fn get_level(&self, level_code: u8) -> Result<level::Level, String>
    {
        match level_code
        {
            0 => Ok(level::LEVEL0),
            _ => Err(format!("Level not found")),
        }
    }

    pub fn update(&self, input: ReadOnlyInput)
    {
        // web_sys::console::log_1(&format!("Update").into());
        if input.is_input_down(&format!("MoveLeft")) || input.is_input_pressed(&format!("MoveLeft"))
        {
            web_sys::console::log_1(&format!("Char should move left").into());
        }
    }
}