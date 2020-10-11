use crate::view_models::PostProcessEffects;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlTexture};

pub trait Effect
{
    fn get_effect_type(&self) -> PostProcessEffects;
    fn set_running_time(&mut self, running_time: f32);
    fn get_running_time(&self) -> f32;
    fn set_max_running_time(&mut self, max_running_time: f32);
    fn get_max_running_time(&self) -> f32;
    fn apply(&self, context: &WebGl2RenderingContext, render_texture: &WebGlTexture, program: &WebGlProgram);
}