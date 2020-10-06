mod stuff;
mod shader_utils;

use image;

use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation, WebGlBuffer, WebGlTexture, WebGlVertexArrayObject};

pub struct View 
{
    background_shader: WebGlProgram,
    background_vao: WebGlVertexArrayObject,
    background_triangle_count: i32,
    background_tile_texture: WebGlTexture,
}

impl View
{
    pub fn new(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<View, String>
    {
        let background = View::init_background(context, tile_map)?;
        
        Ok(View{
            background_shader: background.0,
            background_vao: background.1,
            background_triangle_count: background.2,
            background_tile_texture: background.3
        })
    }

    fn init_background(context: &WebGl2RenderingContext, tile_map: image::RgbaImage) -> Result<(WebGlProgram, WebGlVertexArrayObject, i32, WebGlTexture), String>
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

        let program = shader_utils::link_program(&context, &vert_shader, &frag_shader, vec![(0, "position"), (1, "uv")])?;


        context.use_program(Some(&program));

        //Screen filling quad
        let vertices = vec![ 1.0, -1.0,  0.0, 
                             1.0,  1.0,  0.0, 
                            -1.0,  1.0,  0.0,
                            -1.0, -1.0,  0.0 ];

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
        let uv_buffer = context.create_buffer().ok_or("failed to create uv buffer")?;
        let uv_coords = vec! [ 1.0, 0.0,  1.0, 1.0,  0.0, 1.0,  0.0, 0.0 ];
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
        

        let vao = context.create_vertex_array().ok_or("failed to create vao")?;
        context.bind_vertex_array(Some(&vao));
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
        context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&uv_buffer));
        context.vertex_attrib_pointer_with_i32(1, 2, WebGl2RenderingContext::FLOAT, true, 0, 0);
        context.enable_vertex_attrib_array(0);
        context.enable_vertex_attrib_array(1);




        let tex = context.create_texture().ok_or("failed to create texture")?;


        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
        context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);        



        unsafe 
        {
            
            // let pixels = vec![ 0, 0, 0, 255,
            //                    0, 0, 255, 255,
            //                    0, 255, 0, 255,
            //                    255, 0, 0, 255 ];

                               
            // let pixels = vec![ 0, 0, 0, 255, 0, 0, 0, 255,
            // 0, 0, 255, 255, 0, 0, 255, 255,
            // 0, 0, 0, 255, 0, 0, 0, 255,
            // 0, 0, 255, 255, 0, 0, 255, 255,
            // 0, 255, 0, 255, 0, 255, 0, 255,
            // 255, 0, 0, 255, 255, 0, 0, 255,
            // 0, 255, 0, 255, 0, 255, 0, 255,
            // 255, 0, 0, 255, 255, 0, 0, 255 ];
            
            // let vert_array = js_sys::Uint8Array::view(&pixels);

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
        
        {
            context.use_program(Some(&program));
            let loc = context.get_uniform_location(&program, "tileMapWidth").ok_or("Failed to get location of tileMapWidth")?;
            context.uniform1f(Some(&loc), 2.0);
        }
        
        {
            context.use_program(Some(&program));
            let loc = context.get_uniform_location(&program, "tileMapHeight").ok_or("Failed to get location of tileMapHeight")?;
            context.uniform1f(Some(&loc), 2.0);
        }
        
        
        
        context.get_error();
        
        {
            context.use_program(Some(&program));
            // let map = vec![ 1.0, 1.0, 1.0, 1.0, 1.0,  1.0, 0.0, 1.0, 0.0, 1.0,  1.0, 1.0, 2.0, 1.0, 1.0,  1.0, 0.0, 1.0, 0.0, 1.0,  1.0, 1.0, 3.0, 1.0, 1.0 ];
            let map = vec![ 2, 2, 2, 2, 2,  1, 0, 1, 0, 1,  1, 1, 2, 1, 1,  1, 0, 1, 0, 1,  1, 1, 3, 1, 1 ];
            let loc = context.get_uniform_location(&program, "map").ok_or("Failed to get location of map")?;
            context.uniform1iv_with_i32_array(Some(&loc), &map);
        }

        {
            context.use_program(Some(&program));
            let loc = context.get_uniform_location(&program, "width").ok_or("Failed to get location of width")?;
            context.uniform1f(Some(&loc), 5.0);
        }
        
        {
            context.use_program(Some(&program));
            let loc = context.get_uniform_location(&program, "height").ok_or("Failed to get location of height")?;
            context.uniform1f(Some(&loc), 5.0);
        }




        Ok((program, vao, (vertices.len() / 3) as i32, tex))
    }



    
    fn render_background(&self, context: &WebGl2RenderingContext)
    {
        context.use_program(Some(&self.background_shader));
        context.bind_vertex_array(Some(&self.background_vao));
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.background_tile_texture));

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_FAN,
            0,
            self.background_triangle_count,
        );
    }

    fn clear_screen(&self, context: &WebGl2RenderingContext)
    {
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn draw(&self, context: &WebGl2RenderingContext)
    {
        self.clear_screen(context);
        self.render_background(context);
    }
}