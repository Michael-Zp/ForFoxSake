use crate::view::shader_utils;

use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn initialize_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> 
{
    let vert_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r#"#version 300 es

        in vec4 position;
        in vec2 uvIn;

        out vec2 uvOut;

        void main()
        {
            gl_Position = position;
            uvOut = uvIn;
        }
    "#,
    )?;

    let frag_shader = shader_utils::compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r#"#version 300 es
        precision highp float;

        uniform float width;
        uniform float height;
        uniform int[20*20] map;

        uniform float tileMapWidth;
        uniform float tileMapHeight;
        uniform sampler2D tileMap;
        
        in vec2 uvOut;

        out vec4 outColor;

        void main()
        {
            //Flip, because bmp is not flipped in the file
            vec2 uv = vec2(uvOut.x, 1.0 - uvOut.y);

            float col = floor(width * uv.x);
            float row = floor(height * uv.y);
            float mapTile = row * width + col;
            float tileToUse = float(map[int(mapTile)]);

            float isOddCol = step(0.5, mod(col, 2.0));
            uv.x = isOddCol * (1.0 - uv.x) + (1.0 - isOddCol) * uv.x;
            
            float idOddRow = step(0.5, mod(row, 2.0));
            uv.y = idOddRow * (1.0 - uv.y) + (1.0 - idOddRow) * uv.y;

            float tileToUseCol = mod(tileToUse, tileMapWidth);
            float tileToUseRow = floor(tileToUse / tileMapHeight);

            float startTileX = tileToUseCol / tileMapWidth;
            float tileSizeX = 1.0 / tileMapWidth;
            float mapTileSizeX = 1.0 / width;
            float tiledUvX = mix(startTileX, startTileX + tileSizeX, mod(uv.x, mapTileSizeX) / mapTileSizeX);

            float startTileY = tileToUseRow / tileMapHeight;
            float tileSizeY = 1.0 / tileMapHeight;
            float mapTileSizeY = 1.0 / height;
            float tiledUvY = mix(startTileY, startTileY + tileSizeY, mod(uv.y, mapTileSizeY) / mapTileSizeY);

            // outColor = vec4(0.0, 0.0, 0.25, 1.0);
            // outColor = texture(tileMap, uv);
            outColor = texture(tileMap, vec2(tiledUvX, tiledUvY));
            // outColor = texture(tileMap, vec2(0.55, 0.55));
            // outColor = vec4(uv.x, uv.x, uv.x, 1.0);
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

pub fn set_tile_map_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram, width: f32, height: f32) -> Result<(), String>
{       
    context.use_program(Some(&program));
    shader_utils::set_uniform1f(context, program, width, "tileMapWidth")?;
    shader_utils::set_uniform1f(context, program, height, "tileMapHeight")?;
    Ok(())
}