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
        let model = Model::new()?;
        let view = View::new(&context, tile_map)?;

        let level = model.get_level(0)?;
        view.update_map(&context, level.get_data().to_vec(), level.get_width() as f32, level.get_height() as f32)?;

        Ok(ForFoxSake {
            model: model,
            view: view,
            context
        })
    }

    pub fn draw(&self)
    {
        self.view.draw(&self.context)
    }
}