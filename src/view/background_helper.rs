use crate::view::shader_utils;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlTexture, WebGlVertexArrayObject};

pub fn initialize_background_shader(context: &WebGl2RenderingContext) -> Result<WebGlProgram, String> 
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

pub fn initialize_screen_filling_quad_with_uvs(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(usize, WebGlVertexArrayObject), String> 
{
    context.use_program(Some(program));
    //Screen filling quad
    let vertices = vec![ 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0 ];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe 
    {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    //UV coords
    let uv_buffer = context
        .create_buffer()
        .ok_or("failed to create uv buffer")?;
    let uv_coords = vec![1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0];
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));

    unsafe 
    {
        let uv_array = js_sys::Float32Array::view(&uv_coords);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &uv_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("failed to create vao")?;

    context.bind_vertex_array(Some(&vao));
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));
    context.vertex_attrib_pointer_with_i32(1, 2, WebGl2RenderingContext::FLOAT, true, 0, 0);
    context.enable_vertex_attrib_array(0);
    context.enable_vertex_attrib_array(1);

    Ok((vertices.len() / 3, vao))
}


pub fn initialize_tile_map_texture(context: &WebGl2RenderingContext, tile_map: image::RgbaImage, program: &WebGlProgram) -> Result<WebGlTexture, String>
{
    context.use_program(Some(program));
    let tex = context.create_texture().ok_or("failed to create texture")?;

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
    context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);        

    unsafe 
    {
        let tile_map_width : i32 = tile_map.width() as i32;
        let tile_map_height : i32 = tile_map.height() as i32;

        let pixel_array = js_sys::Uint8Array::view(&tile_map.into_raw());

        match context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                tile_map_width,
                tile_map_height,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(&pixel_array)
            )
        {
            Ok(x) => Ok(x),
            Err(_) => Err("failed to copy image data to background texture"),
        }?;
    }

    set_tile_map_uniforms(context, program, 2.0, 2.0)?;

    Ok(tex)
}

fn set_tile_map_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram, width: f32, height: f32) -> Result<(), String>
{       
    context.use_program(Some(&program));
    shader_utils::set_uniform1f(context, program, width, "tileMapWidth")?;
    shader_utils::set_uniform1f(context, program, height, "tileMapHeight")?;
    Ok(())
}