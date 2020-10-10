mod shader_utils;
mod background_helper;
mod sprites_helper;
mod particles_helper;

use crate::view_models::{SpritesViewModel, LevelViewModel, ParticlesViewModel};

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

    particles_shader: WebGlProgram,
    particle_systems_count: i32,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage, sprite_tile_map: image::RgbaImage) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        let sprites = View::init_sprite_renderer(context, sprite_tile_map)?;
        let particles = View::init_particles_renderer(context)?;
        
        Ok(View{
            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3,

            sprite_shader: sprites.0,
            sprite_texture: sprites.1,
            sprite_count: 0,

            particles_shader: particles,
            particle_systems_count: 0,
        })
    }

    fn init_background(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<(WebGlProgram, WebGlVertexArrayObject, i32, WebGlTexture), String>
    {
        let program = background_helper::initialize_shader(&context)?;
        let screen_filling_quad = shader_utils::initialize_quad_with_uvs(&context, &program, cgmath::Vector2 { x: 0.0, y: 0.0 }, cgmath::Vector2 { x: 2.0, y: 2.0 })?;
        let tex = shader_utils::initialize_texture(context, tile_map, &program, true)?;  
        background_helper::set_tile_map_uniforms(context, &program, 2.0, 2.0)?;   
        
        Ok((program, screen_filling_quad.1, screen_filling_quad.0 as i32, tex))
    }

    fn init_sprite_renderer(context: &WebGl2RenderingContext, texture_image: image::RgbaImage) -> Result<(WebGlProgram, WebGlTexture), String>
    {
        let program = sprites_helper::initialize_shader(&context)?;
        let tex = shader_utils::initialize_texture(context, texture_image, &program, true)?;
        sprites_helper::set_tile_map_uniforms(context, &program, 10.0, 10.0)?;

        Ok((program, tex))
    }

    fn init_particles_renderer(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String>
    {
        let program = particles_helper::initialize_shader(context)?;
        Ok(program)
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

    fn render_particles(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.particles_shader));
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.particle_systems_count * 6,
        );
    }

    fn clear_screen(&self, context: &WebGl2RenderingContext)
    {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn update_map(&self, context: &WebGl2RenderingContext, new_map: LevelViewModel) -> Result<(), String>
    {

        context.use_program(Some(&self.background_shader));
        let loc = context.get_uniform_location(&self.background_shader, "map").ok_or("Failed to get location of map")?;
        context.uniform1iv_with_i32_array(Some(&loc), &new_map.data);

        shader_utils::set_uniform1f(context, &self.background_shader, new_map.width, "width")?;
        shader_utils::set_uniform1f(context, &self.background_shader, new_map.height, "height")?;

        Ok(())
    }

    pub fn update_sprites(&mut self, context: &WebGl2RenderingContext, new_sprites: SpritesViewModel) -> Result<(), String>
    {
        sprites_helper::update_sizes(context, &self.sprite_shader, new_sprites.sizes)?;
        sprites_helper::update_positions(context, &self.sprite_shader, new_sprites.positions)?;
        sprites_helper::update_tile_map_indices(context, &self.sprite_shader, new_sprites.tile_map_indices)?;
        self.sprite_count = new_sprites.count;
        Ok(())
    }

    pub fn update_particle_systems(&mut self, context: &WebGl2RenderingContext, new_particles: ParticlesViewModel) -> Result<(), String>
    {
        particles_helper::update_positions(context, &self.particles_shader, new_particles.positions)?;
        particles_helper::update_max_speeds(context, &self.particles_shader, new_particles.max_speeds)?;
        particles_helper::update_running_times(context, &self.particles_shader, new_particles.running_times)?;
        particles_helper::update_max_running_times(context, &self.particles_shader, new_particles.max_running_times)?;
        self.particle_systems_count = new_particles.count;
        Ok(())
    }

    pub fn update(&mut self, context: &WebGl2RenderingContext, sprites: SpritesViewModel, particles: ParticlesViewModel) -> Result<(), String>
    {
        self.update_sprites(context, sprites)?;
        self.update_particle_systems(context, particles)?;
        Ok(())
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
        self.render_sprites(context);
        self.render_particles(context);
    }
}