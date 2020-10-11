use crate::view::post_process_effect;
use crate::view_models::PostProcessEffects;
use crate::view::shader_utils;

use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlTexture};

pub struct Vignette
{
    effect_type: PostProcessEffects,
    running_time: f32,
    max_running_time: f32,
}

impl Vignette
{
    pub fn new(running_time: f32, max_running_time: f32) -> Vignette
    {
        Vignette { effect_type: PostProcessEffects::VIGNETTE, running_time: running_time, max_running_time: max_running_time }
    }
}

impl post_process_effect::effect::Effect for Vignette
{
    fn get_effect_type(&self) -> PostProcessEffects
    {
        self.effect_type
    }

    fn set_running_time(&mut self, running_time: f32)
    {
        self.running_time = running_time;
    }

    fn get_running_time(&self) -> f32
    {
        self.running_time
    }
    
    fn set_max_running_time(&mut self, max_running_time: f32)
    {
        self.max_running_time = max_running_time;
    }

    fn get_max_running_time(&self) -> f32
    {
        self.max_running_time
    }

    fn apply(&self, context: &WebGl2RenderingContext, render_texture: &WebGlTexture, program: &WebGlProgram)
    {
        context.use_program(Some(program));
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(render_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            6,
        );
    }
}

pub fn get_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String>
{
    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        out vec2 uv;

        void main()
        {
            int subIdx = gl_VertexID % 6;
            
            if(subIdx == 0)
            {
                gl_Position = vec4(1, -1, 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 1)
            {
                gl_Position = vec4(1, 1, 0, 1);
                uv = vec2(1.0, 1.0);
            }
            else if(subIdx == 2)
            {
                gl_Position = vec4(-1, 1, 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else if(subIdx == 3)
            {
                gl_Position = vec4(1, -1, 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 4)
            {
                gl_Position = vec4(-1, 1, 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else// if(subIdx == 5)
            {
                gl_Position = vec4(-1, -1, 0, 1);
                uv = vec2(0.0, 0.0);
            }
        }
    "#,
    )?;

    let frag_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        precision highp float;

        uniform sampler2D tex;
        
        in vec2 uv;

        out vec4 outColor;

        void main()
        {
            float alpha = smoothstep(0.5, 0.7, length(uv - vec2(0.5)));
            outColor = texture(tex, uv) + vec4(0.0, 0.0, 0.0, alpha);
        }
    "#,
    )?;

    return shader_utils::link_program(
        &context,
        &vert_shader,
        &frag_shader,
        vec![],
    );
}