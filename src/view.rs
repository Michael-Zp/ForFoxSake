mod shader_utils;
mod background_helper;

use image;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlTexture, WebGlVertexArrayObject};

pub struct View 
{
    background_shader: WebGlProgram,
    background_vao: WebGlVertexArrayObject,
    background_triangle_count: i32,
    background_tile_texture: WebGlTexture,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        
        Ok(View{
            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3
        })
    }

    fn init_background(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<(WebGlProgram, WebGlVertexArrayObject, i32, WebGlTexture), String>
    {
        let program = background_helper::initialize_background_shader(&context)?;
        let screen_filling_quad = background_helper::initialize_screen_filling_quad_with_uvs(&context, &program)?;
        let tex = background_helper::initialize_tile_map_texture(context, tile_map, &program)?;     
        
        Ok((program, screen_filling_quad.1, screen_filling_quad.0 as i32, tex))
    }
    
    fn render_background(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.background_shader));
        context.bind_vertex_array(Some(&self.background_vao));
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.background_tile_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.background_triangle_count,
        );
    }

    fn clear_screen(&self, context: &WebGl2RenderingContext)
    {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update_map(&self, context: &WebGl2RenderingContext, map: std::vec::Vec<i32>, width: f32, height: f32) -> Result<(), String>
    {
        {
            context.use_program(Some(&self.background_shader));
            let loc = context.get_uniform_location(&self.background_shader, "map").ok_or("Failed to get location of map")?;
            context.uniform1iv_with_i32_array(Some(&loc), &map);
        }

        shader_utils::set_uniform1f(context, &self.background_shader, width, "width")?;
        shader_utils::set_uniform1f(context, &self.background_shader, height, "height")?;

        Ok(())
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
    }
}