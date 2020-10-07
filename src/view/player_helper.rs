use crate::view::shader_utils;

use cgmath;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

pub fn initialize_player_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> 
{
    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        uniform vec2 translation;

        in vec4 position;
        in vec2 uvIn;

        out vec2 uvOut;

        void main()
        {
            gl_Position = position + vec4(translation, 0, 0);
            uvOut = uvIn;
        }
    "#,
    )?;

    let frag_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        precision highp float;

        uniform sampler2D tex;
        
        in vec2 uvOut;

        out vec4 outColor;

        void main()
        {
            //Flip, because bmp is not flipped in the file
            vec2 uv = vec2(uvOut.x, 1.0 - uvOut.y);

            // outColor = vec4(0.0, 0.0, 0.25, 1.0);
            // outColor = texture(tex, uv);
            outColor = texture(tex, uv);
        }
    "#,
    )?;

    return shader_utils::link_program(
        &context,
        &vert_shader,
        &frag_shader,
        vec![(0, "position"), (1, "uv")],
    );
}

pub fn update_player_pos(context: &WebGl2RenderingContext, player_shader: &WebGlProgram, new_position: cgmath::Vector2<f32>) -> Result<(), String>
{       
    context.use_program(Some(player_shader));
    shader_utils::set_uniform2f(context, player_shader, new_position, "translation")?;
    Ok(())
}