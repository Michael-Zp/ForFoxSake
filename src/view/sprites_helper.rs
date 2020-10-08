use crate::view::shader_utils;

use cgmath;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn initialize_sprites_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> 
{
    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        uniform vec2[10] sizes;
        uniform vec2[10] positions;
        uniform int[10] tileMapIndices;

        out vec2 uv;
        flat out int tileMapIndex;

        void main()
        {
            int idx = gl_VertexID / 6;
            vec2 centerPos = positions[idx];
            tileMapIndex = tileMapIndices[idx];
            
            int subIdx = gl_VertexID % 6;
            
            if(subIdx == 0)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(1, -1), 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 1)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(1, 1), 0, 1);
                uv = vec2(1.0, 1.0);
            }
            else if(subIdx == 2)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(-1, 1), 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else if(subIdx == 3)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(1, -1), 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 4)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(-1, 1), 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else// if(subIdx == 5)
            {
                gl_Position = vec4(centerPos + (sizes[idx] / 2.0) * vec2(-1, -1), 0, 1);
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

        uniform float tileMapWidth;
        uniform float tileMapHeight;
        uniform sampler2D tileMap;
        
        in vec2 uv;
        flat in int tileMapIndex;

        out vec4 outColor;

        void main()
        {
            //Flip, because bmp is not flipped in the file
            vec2 uv = vec2(uv.x, 1.0 - uv.y);

            float tileToUseCol = mod(float(tileMapIndex), tileMapWidth);
            float tileToUseRow = floor(float(tileMapIndex) / tileMapHeight);

            float startTileX = tileToUseCol / tileMapWidth;
            float tileSizeX = 1.0 / tileMapWidth;
            float tiledUvX = mix(startTileX, startTileX + tileSizeX, uv.x);

            float startTileY = tileToUseRow / tileMapHeight;
            float tileSizeY = 1.0 / tileMapHeight;
            float tiledUvY = mix(startTileY, startTileY + tileSizeY, uv.y);

            outColor = texture(tileMap, vec2(tiledUvX, tiledUvY));

            // outColor = vec4(1.0, 0.0, 0.0, 1.0);
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

fn update_sprite_uniform(context: &WebGl2RenderingContext, sprite_shader: &WebGlProgram, new_positions: [cgmath::Vector2<f32>;10], uniform_name: &str) -> Result<(), String>
{
    context.use_program(Some(sprite_shader));
    let loc = context.get_uniform_location(sprite_shader, uniform_name).ok_or(format!("Failed to get location of {}", uniform_name))?;
    let mut data : std::vec::Vec<f32> = std::vec::Vec::new();
    for x in new_positions.iter()
    {
        data.push(x.x);
        data.push(x.y);
    }

    context.uniform2fv_with_f32_array(Some(&loc), &data);
    Ok(())
}

pub fn update_sprite_sizes(context: &WebGl2RenderingContext, sprite_shader: &WebGlProgram, new_sizes: [cgmath::Vector2<f32>;10]) -> Result<(), String>
{
    
    update_sprite_uniform(context, sprite_shader, new_sizes, "sizes")?;
    Ok(())
}

pub fn update_sprite_positions(context: &WebGl2RenderingContext, sprite_shader: &WebGlProgram, new_positions: [cgmath::Vector2<f32>;10]) -> Result<(), String>
{
    update_sprite_uniform(context, sprite_shader, new_positions, "positions")?;
    Ok(())
}

pub fn update_sprite_tile_map_indices(context: &WebGl2RenderingContext, sprite_shader: &WebGlProgram, new_indices: [i32;10]) -> Result<(), String>
{       
    context.use_program(Some(sprite_shader));
    let loc = context.get_uniform_location(sprite_shader, "tileMapIndices").ok_or("Failed to get location of tileMapIndices")?;
    let mut data : std::vec::Vec<i32> = std::vec::Vec::new();
    for x in new_indices.iter()
    {
        data.push(x.clone());
    }

    context.uniform1iv_with_i32_array(Some(&loc), &data);
    Ok(())
}

pub fn set_tile_map_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram, width: f32, height: f32) -> Result<(), String>
{       
    context.use_program(Some(&program));
    shader_utils::set_uniform1f(context, program, width, "tileMapWidth")?;
    shader_utils::set_uniform1f(context, program, height, "tileMapHeight")?;
    Ok(())
}