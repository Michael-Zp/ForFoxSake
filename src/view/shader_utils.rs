use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};


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