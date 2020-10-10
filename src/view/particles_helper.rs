use crate::view::shader_utils;

use cgmath;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn initialize_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> 
{
    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        uniform vec2[10] positions;
        uniform float[10] numberOfParticles;
        uniform float[10] seed;
        uniform float[10] maxSpeeds;
        uniform float[10] runningTimes;
        uniform float[10] maxRunningTimes;
        uniform float[10] tileMapIndices;

        out vec2 uv;
        flat out int idx;
        flat out int tileMapIndex;
        flat out vec2 centerPos;

        const float sizeAdjust = 1.4142; // 1 / cos(45°) -> Convert half diagonal of quad to cover a circular area of r = half diagonal

        void main()
        {
            idx = gl_VertexID / 6;
            centerPos = positions[idx];
            float maxSize = maxSpeeds[idx] * runningTimes[idx];
            tileMapIndex = int(tileMapIndices[idx]);
            
            int subIdx = gl_VertexID % 6;
            
            if(subIdx == 0)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(1, -1) * sizeAdjust, 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 1)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(1, 1) * sizeAdjust, 0, 1);
                uv = vec2(1.0, 1.0);
            }
            else if(subIdx == 2)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(-1, 1) * sizeAdjust, 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else if(subIdx == 3)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(1, -1) * sizeAdjust, 0, 1);
                uv = vec2(1.0, 0.0);
            }
            else if(subIdx == 4)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(-1, 1) * sizeAdjust, 0, 1);
                uv = vec2(0.0, 1.0);
            }
            else// if(subIdx == 5)
            {
                gl_Position = vec4(centerPos + maxSize * vec2(-1, -1) * sizeAdjust, 0, 1);
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

        uniform float[10] maxRunningTimes;
        uniform float[10] runningTimes;

        uniform float tileMapWidth;
        uniform float tileMapHeight;
        uniform sampler2D tileMap;
        
        in vec2 uv;
        flat in int idx;
        flat in int tileMapIndex;
        flat in vec2 centerPos;

        out vec4 outColor;

        void main()
        {
            //Flip, because bmp is not flipped in the file
            // vec2 uv = vec2(uv.x, 1.0 - uv.y);

            // float tileToUseCol = mod(float(tileMapIndex), tileMapWidth);
            // float tileToUseRow = floor(float(tileMapIndex) / tileMapHeight);

            // float startTileX = tileToUseCol / tileMapWidth;
            // float tileSizeX = 1.0 / tileMapWidth;
            // float tiledUvX = mix(startTileX, startTileX + tileSizeX, uv.x);

            // float startTileY = tileToUseRow / tileMapHeight;
            // float tileSizeY = 1.0 / tileMapHeight;
            // float tiledUvY = mix(startTileY, startTileY + tileSizeY, uv.y);

            // outColor = texture(tileMap, vec2(tiledUvX, tiledUvY));

            outColor = vec4(1.0, 0.0, 0.0, 1.0 - (runningTimes[idx] / maxRunningTimes[idx]));
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

pub fn update_positions(context: &WebGl2RenderingContext, shader: &WebGlProgram, new_positions: [cgmath::Vector2<f32>;10]) -> Result<(), String>
{
    shader_utils::set_uniform2f_arr10(context, shader, new_positions, "positions")?;
    Ok(())
}

pub fn update_max_speeds(context: &WebGl2RenderingContext, shader: &WebGlProgram, new_max_speeds: [f32;10]) -> Result<(), String>
{
    shader_utils::set_uniform1f_arr10(context, shader, new_max_speeds, "maxSpeeds")?;
    Ok(())
}

pub fn update_running_times(context: &WebGl2RenderingContext, shader: &WebGlProgram, new_running_times: [f32;10]) -> Result<(), String>
{
    shader_utils::set_uniform1f_arr10(context, shader, new_running_times, "runningTimes")?;
    Ok(())
}

pub fn update_max_running_times(context: &WebGl2RenderingContext, shader: &WebGlProgram, new_max_running_times: [f32;10]) -> Result<(), String>
{
    shader_utils::set_uniform1f_arr10(context, shader, new_max_running_times, "maxRunningTimes")?;
    Ok(())
}

// pub fn update_tile_map_indices(context: &WebGl2RenderingContext, shader: &WebGlProgram, new_indices: [i32;10]) -> Result<(), String>
// {       
//     context.use_program(Some(shader));
//     let loc = context.get_uniform_location(shader, "tileMapIndices").ok_or("Failed to get location of tileMapIndices")?;
//     let mut data : std::vec::Vec<i32> = std::vec::Vec::new();
//     for x in new_indices.iter()
//     {
//         data.push(x.clone());
//     }

//     context.uniform1iv_with_i32_array(Some(&loc), &data);
//     Ok(())
// }

// pub fn set_tile_map_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram, width: f32, height: f32) -> Result<(), String>
// {       
//     context.use_program(Some(&program));
//     shader_utils::set_uniform1f(context, program, width, "tileMapWidth")?;
//     shader_utils::set_uniform1f(context, program, height, "tileMapHeight")?;
//     Ok(())
// }