mod shader_utils;
mod background_helper;
mod player_helper;

use crate::view_model::ViewModel;

use image;
use cgmath;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlTexture, WebGlVertexArrayObject};

pub struct View 
{
    background_shader: WebGlProgram,
    background_vao: WebGlVertexArrayObject,
    background_triangle_count: i32,
    background_tile_texture: WebGlTexture,

    player_shader: WebGlProgram,
    player_vao: WebGlVertexArrayObject,
    player_triangle_count: i32,
    player_texture: WebGlTexture,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage, player_texture: image::RgbaImage) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        let player = View::init_player(context, player_texture)?;
        
        Ok(View{
            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3,

            player_shader: player.0,
            player_vao: player.1,
            player_triangle_count: player.2,
            player_texture: player.3,
        })
    }

    fn init_background(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<(WebGlProgram, WebGlVertexArrayObject, i32, WebGlTexture), String>
    {
        let program = background_helper::initialize_background_shader(&context)?;
        let screen_filling_quad = shader_utils::initialize_quad_with_uvs(&context, &program, cgmath::Vector2 { x: 0.0, y: 0.0 }, cgmath::Vector2 { x: 2.0, y: 2.0 })?;
        let tex = shader_utils::initialize_texture(context, tile_map, &program, true)?;  
        background_helper::set_tile_map_uniforms(context, &program, 2.0, 2.0)?;   
        
        Ok((program, screen_filling_quad.1, screen_filling_quad.0 as i32, tex))
    }

    fn init_player(context: &WebGl2RenderingContext, texture_image: image::RgbaImage) -> Result<(WebGlProgram, WebGlVertexArrayObject, i32, WebGlTexture), String>
    {
        let program = player_helper::initialize_player_shader(&context)?;
        let quad = shader_utils::initialize_quad_with_uvs(&context, &program, cgmath::Vector2 { x: 0.0, y: 0.0 }, cgmath::Vector2 { x: 0.2, y: 0.2 })?;
        let tex = shader_utils::initialize_texture(context, texture_image, &program, true)?;

        Ok((program, quad.1, quad.0 as i32, tex))
    }
    
    fn render_background(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.background_shader));
        context.bind_vertex_array(Some(&self.background_vao));
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.background_tile_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.background_triangle_count,
        );
    }

    fn render_player(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.player_shader));
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        context.bind_vertex_array(Some(&self.player_vao));
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.player_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.player_triangle_count,
        );
    }

    fn clear_screen(&self, context: &WebGl2RenderingContext)
    {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update_map(&self, context: &WebGl2RenderingContext, map: std::vec::Vec<i32>, width: f32, height: f32) -> Result<(), String>
    {
        context.use_program(Some(&self.background_shader));
        let loc = context.get_uniform_location(&self.background_shader, "map").ok_or("Failed to get location of map")?;
        context.uniform1iv_with_i32_array(Some(&loc), &map);

        shader_utils::set_uniform1f(context, &self.background_shader, width, "width")?;
        shader_utils::set_uniform1f(context, &self.background_shader, height, "height")?;

        Ok(())
    }

    pub fn update_player(&self, context: &WebGl2RenderingContext, new_position: cgmath::Vector2<f32>)
    {
        player_helper::update_player_pos(context, &self.player_shader, new_position);
    }

    pub fn update(&self, context: &WebGl2RenderingContext, view_model: ViewModel)
    {
        self.update_player(context, view_model.player_pos);
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
        self.render_player(context);
    }
}