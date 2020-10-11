pub mod effect;
pub mod vignette;

use crate::view_models::PostProcessEffects;
use web_sys::{WebGlProgram, WebGl2RenderingContext};


pub fn get_shader_by_type(context: &WebGl2RenderingContext, effect_type: &PostProcessEffects) -> Result<WebGlProgram, String>
{
    match effect_type
    {
        PostProcessEffects::VIGNETTE =>
        {
            vignette::get_shader(context)
        }
    }
}

pub fn get_effect_by_type(effect_type: PostProcessEffects, running_time: f32, max_running_time: f32) -> Box<dyn effect::Effect>
{
    match effect_type
    {
        PostProcessEffects::VIGNETTE => 
        {
            Box::new(vignette::Vignette::new(running_time, max_running_time))
        }
    }
}