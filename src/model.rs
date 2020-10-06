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
}