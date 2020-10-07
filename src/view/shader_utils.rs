use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlTexture, WebGlVertexArrayObject};
use cgmath;

pub fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> 
{
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))
    ?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } 
    else 
    {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
    }
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader, attribs: std::vec::Vec<(u32, &str)>) -> Result<WebGlProgram, String> 
{
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))
    ?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);

    for attrib in &attribs 
    {
        context.bind_attrib_location(&program, attrib.0, attrib.1);
    }

    context.link_program(&program);
    context.use_program(Some(&program));

    // let loc = context.get_uniform_location(&program, "myMap").ok_or(format!("get_uniform_location for myMap returned error code: {}", context.get_error()))?;

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } 
    else 
    {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}

pub fn set_uniform1f(context: &WebGl2RenderingContext, program: &WebGlProgram, data: f32, name: &str) -> Result<(), String>
{
    context.use_program(Some(program));
    let loc = context.get_uniform_location(program, name).ok_or(format!("Failed to get location of {}", name))?;
    context.uniform1f(Some(&loc), data);
    Ok(())
}

pub fn set_uniform2f(context: &WebGl2RenderingContext, program: &WebGlProgram, data: cgmath::Vector2<f32>, name: &str) -> Result<(), String>
{
    context.use_program(Some(program));
    let loc = context.get_uniform_location(program, name).ok_or(format!("Failed to get location of {}", name))?;
    context.uniform2f(Some(&loc), data.x, data.y);
    Ok(())
}

pub fn initialize_texture(context: &WebGl2RenderingContext, texture: image::RgbaImage, program: &WebGlProgram, linear: bool) -> Result<WebGlTexture, String>
{
    context.use_program(Some(program));
    let tex = context.create_texture().ok_or("failed to create texture")?;

    context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    if linear
    {
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
    }
    else
    {
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
    }
    context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);        

    unsafe 
    {
        let texture_width : i32 = texture.width() as i32;
        let texture_height : i32 = texture.height() as i32;

        let pixel_array = js_sys::Uint8Array::view(&texture.into_raw());

        match context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                texture_width,
                texture_height,
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

    Ok(tex)
}


pub fn initialize_quad_with_uvs(context: &WebGl2RenderingContext, program: &WebGlProgram, center: cgmath::Vector2<f32>, size: cgmath::Vector2<f32>) -> Result<(usize, WebGlVertexArrayObject), String> 
{
    context.use_program(Some(program));
    //Screen filling quad
    let right = center.x + size.x / 2.0;
    let left = center.x - size.x / 2.0;
    let up = center.y + size.y / 2.0;
    let down = center.y - size.y / 2.0;

    let vertices = vec![ right, down, 0.0,
                         right, up, 0.0,
                         left, up, 0.0,
                         right, down, 0.0,
                         left, up, 0.0,
                         left, down, 0.0 ];

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
    let uv_coords = vec![1.0, 0.0, 
                         1.0, 1.0, 
                         0.0, 1.0,
                         1.0, 0.0,
                         0.0, 1.0, 
                         0.0, 0.0];
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