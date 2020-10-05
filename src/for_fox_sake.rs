use crate::model::{Model};
use crate::view::{View};

pub struct ForFoxSake 
{
    model: Model,
    view: View,
}

impl ForFoxSake 
{
    pub fn new() -> Result<ForFoxSake, String>
    {
        Ok(ForFoxSake {
            model: Model::new()?,
            view: View::new()?
        })
    }

    pub fn test_interface(&self) 
    {
        self.model.test_interface();
        self.view.test_interface();
    }
}