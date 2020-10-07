use crate::model::{Model};
use crate::view::{View};
mod input;
use input::Input;
pub mod read_only_input;
use read_only_input::ReadOnlyInput;

use image;

use web_sys::{WebGl2RenderingContext};

pub struct ForFoxSake 
{
    model: Model,
    view: View,
    context: WebGl2RenderingContext,
    input: Input,
}

impl ForFoxSake 
{
    pub fn new(context: WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<ForFoxSake, String>
    {
        let model = Model::new()?;
        let view = View::new(&context, tile_map)?;

        let level = model.get_level(0)?;
        view.update_map(&context, level.get_data().to_vec(), level.get_width() as f32, level.get_height() as f32)?;

        let input = Input::new();

        Ok(ForFoxSake {
            model: model,
            view: view,
            context,
            input,
        })
    }

    pub fn update(&mut self, delta_time: i32)
    {
        let read_only_input = ReadOnlyInput::new(&self.input);
        self.model.update(read_only_input);
        self.input.finalize();
    }

    pub fn draw(&self)
    {
        self.view.draw(&self.context)
    }

    pub fn key_down(&mut self, key_code: i32)
    {
        self.input.key_down(key_code);
    }

    pub fn key_up(&mut self, key_code: i32)
    {
        self.input.key_up(key_code);
    }
}