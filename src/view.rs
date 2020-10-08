mod shader_utils;
mod background_helper;
mod sprites_helper;

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

    sprite_shader: WebGlProgram,
    sprite_texture: WebGlTexture,
    sprite_count: i32,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage, sprite_tile_map: image::RgbaImage) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        let sprites = View::init_sprite_renderer(context, sprite_tile_map)?;
        
        Ok(View{
            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3,

            sprite_shader: sprites.0,
            sprite_texture: sprites.1,
            sprite_count: 0,
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

    fn init_sprite_renderer(context: &WebGl2RenderingContext, texture_image: image::RgbaImage) -> Result<(WebGlProgram, WebGlTexture), String>
    {
        let program = sprites_helper::initialize_sprites_shader(&context)?;
        let tex = shader_utils::initialize_texture(context, texture_image, &program, true)?;
        sprites_helper::set_tile_map_uniforms(context, &program, 2.0, 2.0)?;

        Ok((program, tex))
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

    fn render_sprites(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.sprite_shader));
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.sprite_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.sprite_count * 6,
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

    pub fn update_sprites(&mut self, context: &WebGl2RenderingContext, new_sizes: [cgmath::Vector2<f32>;10], new_positions: [cgmath::Vector2<f32>;10], new_tile_map_indices: [i32;10], sprite_count: i32) -> Result<(), String>
    {
        sprites_helper::update_sprite_sizes(context, &self.sprite_shader, new_sizes)?;
        sprites_helper::update_sprite_positions(context, &self.sprite_shader, new_positions)?;
        sprites_helper::update_sprite_tile_map_indices(context, &self.sprite_shader, new_tile_map_indices)?;
        self.sprite_count = sprite_count;
        Ok(())
    }

    pub fn update(&mut self, context: &WebGl2RenderingContext, view_model: ViewModel) -> Result<(), String>
    {
        self.update_sprites(context, view_model.sprite_sizes, view_model.sprite_positions, view_model.sprite_tile_map_indices, view_model.sprite_count)?;
        Ok(())
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
        self.render_sprites(context);
    }
}