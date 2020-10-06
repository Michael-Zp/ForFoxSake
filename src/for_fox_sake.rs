use crate::model::{Model};
use crate::view::{View};

use image;

use web_sys::{WebGl2RenderingContext};

pub struct ForFoxSake 
{
    model: Model,
    view: View,
    context: WebGl2RenderingContext,
}

impl ForFoxSake 
{
    pub fn new(context: WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<ForFoxSake, String>
    {
        Ok(ForFoxSake {
            model: Model::new()?,
            view: View::new(&context, tile_map)?,
            context
        })
    }

    pub fn draw(&self)
    {
        self.view.draw(&self.context)
    }

    pub fn test_interface(&self) 
    {
        self.model.test_interface();
    }
}