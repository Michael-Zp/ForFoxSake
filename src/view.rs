mod shader_utils;
mod background_helper;
mod sprites_helper;
mod particles_helper;
mod post_process_effect;

use crate::view_models::{SpritesViewModel, LevelViewModel, ParticlesViewModel, PostProcessViewModel, PostProcessEffects};

use image;
use cgmath;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlTexture, WebGlVertexArrayObject};

pub struct View 
{
    render_texture: WebGlTexture,

    background_shader: WebGlProgram,
    background_vao: WebGlVertexArrayObject,
    background_triangle_count: i32,
    background_tile_texture: WebGlTexture,

    sprite_shader: WebGlProgram,
    sprite_texture: WebGlTexture,
    sprite_count: i32,

    particles_shader: WebGlProgram,
    particle_systems_count: i32,

    post_process_effect_shaders: std::collections::HashMap<PostProcessEffects, WebGlProgram>,
    post_process_effects: std::vec::Vec<Box<dyn post_process_effect::effect::Effect>>,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage, sprite_tile_map: image::RgbaImage, width: i32, height: i32) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        let sprites = View::init_sprite_renderer(context, sprite_tile_map)?;
        let particles = View::init_particles_renderer(context)?;
        
        let render_texture = View::init_render_texture(context, width, height)?;
        
        let mut view = View {
            render_texture: render_texture,

            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3,

            sprite_shader: sprites.0,
            sprite_texture: sprites.1,
            sprite_count: 0,

            particles_shader: particles,
            particle_systems_count: 0,

            post_process_effect_shaders: std::collections::HashMap::new(),
            post_process_effects: std::vec::Vec::new(),
        };

        view.init_post_process_shaders(context)?;

        Ok(view)
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

    fn init_render_texture(context: &WebGl2RenderingContext, width: i32, height: i32) -> Result<WebGlTexture, String>
    {
        let tex = context.create_texture().ok_or("failed to create texture")?;
    
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
        match context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None
            )
        {
            Ok(x) => Ok(x),
            Err(_) => Err("failed to initialize render texture"),
        }?;

        context.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, Some(&tex), 0);

        unsafe
        {
            let draw_buffer = js_sys::Uint32Array::view(&[WebGl2RenderingContext::COLOR_ATTACHMENT0]);
            context.draw_buffers(draw_buffer.as_ref());
        }

        Ok(tex)
    }

    fn init_post_process_shaders(&mut self, context: &WebGl2RenderingContext) -> Result<(), String>
    {
        let all_effects = vec![ PostProcessEffects::VIGNETTE ];

        for effect in all_effects.iter()
        {
            self.post_process_effect_shaders.insert(*effect, post_process_effect::get_shader_by_type(context, effect)?);
        }

        Ok(())
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

    fn apply_post_process_effects(&self, context: &WebGl2RenderingContext)
    {
        for effect in self.post_process_effects.iter()
        {
            effect.apply(context, &self.render_texture, self.post_process_effect_shaders.get(&effect.get_effect_type()).unwrap());
        }
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

    pub fn update_sprites(&mut self, context: &WebGl2RenderingContext, updated_sprites: SpritesViewModel) -> Result<(), String>
    {
        sprites_helper::update_sizes(context, &self.sprite_shader, updated_sprites.sizes)?;
        sprites_helper::update_positions(context, &self.sprite_shader, updated_sprites.positions)?;
        sprites_helper::update_tile_map_indices(context, &self.sprite_shader, updated_sprites.tile_map_indices)?;
        self.sprite_count = updated_sprites.count;
        Ok(())
    }

    pub fn update_particle_systems(&mut self, context: &WebGl2RenderingContext, updated_particles: ParticlesViewModel) -> Result<(), String>
    {
        particles_helper::update_positions(context, &self.particles_shader, updated_particles.positions)?;
        particles_helper::update_max_speeds(context, &self.particles_shader, updated_particles.max_speeds)?;
        particles_helper::update_running_times(context, &self.particles_shader, updated_particles.running_times)?;
        particles_helper::update_max_running_times(context, &self.particles_shader, updated_particles.max_running_times)?;
        self.particle_systems_count = updated_particles.count;
        Ok(())
    }
    
    pub fn update_post_process_effects(&mut self, updated_post_process_effects: PostProcessViewModel) -> Result<(), String>
    {
        self.post_process_effects.clear();
        for effect in updated_post_process_effects.effects.iter()
        {
            self.post_process_effects.push(post_process_effect::get_effect_by_type(PostProcessEffects::VIGNETTE, effect.running_time, effect.max_running_time));
        }
        Ok(())
    }

    pub fn update(&mut self, context: &WebGl2RenderingContext, sprites: SpritesViewModel, particles: ParticlesViewModel, post_process_effects: PostProcessViewModel) -> Result<(), String>
    {
        self.update_sprites(context, sprites)?;
        self.update_particle_systems(context, particles)?;
        self.update_post_process_effects(post_process_effects)?;
        Ok(())
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
        self.render_sprites(context);
        self.render_particles(context);
        self.apply_post_process_effects(context);
    }
}