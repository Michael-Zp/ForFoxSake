mod stuff;

pub struct Model 
{
    map: stuff::Map,
}

impl Model
{
    pub fn new() -> Result<Model, String>
    {
        Ok(Model{
            map: stuff::Map{ }
        })
    }

    pub fn test_interface(&self)
    {
        stuff::test_impl();
    }
}