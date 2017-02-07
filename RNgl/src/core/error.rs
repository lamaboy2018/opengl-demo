use glx::*;

#[allow(dead_code)]
pub unsafe fn glcheckGlError(s:&str){
    let mut err = gl::GetError();

    while  err  != gl::NO_ERROR{


        let mut str = "";
        match err
        {
            gl::INVALID_ENUM         =>{ str = "INVALID_ENUM"}
            gl::INVALID_VALUE =>{str ="INVALID_VALUE "}
            gl::INVALID_OPERATION =>{str = "INVALID_OPERATION "}
            0x0503 =>{str =" STACK_OVERFLOW"}
            0x0504 =>{str = "GL_STACK_UNDERFLOW"}
            0x0507 =>{ str ="GL_CONTEXT_LOST"}
            gl::OUT_OF_MEMORY =>{str =" STACK_OVERFLOW"}
            gl::INVALID_FRAMEBUFFER_OPERATION =>{str ="INVALID_FRAMEBUFFER_OPERATION"}
            _ => {  println!("gl::GetError {} err ={} unknow " ,s, err );}
        }
        println!("gl::GetError {} err ={}  {} " ,s, err ,str );
        err = gl::GetError();

    }
    println!("gl::GetError {}  no err ={}   " ,s, err  );
}
/*

#[allow(dead_code)]
#[inline]
fn get_gl_error(ctxt: &mut context::CommandContext) -> Option<&'static str> {
    match unsafe { ctxt.gl.GetError() } {
        gl::NO_ERROR => None,
        gl::INVALID_ENUM => Some("GL_INVALID_ENUM"),
        gl::INVALID_VALUE => Some("GL_INVALID_VALUE"),
        gl::INVALID_OPERATION => Some("GL_INVALID_OPERATION"),
        gl::INVALID_FRAMEBUFFER_OPERATION => Some("GL_INVALID_FRAMEBUFFER_OPERATION"),
        gl::OUT_OF_MEMORY => Some("GL_OUT_OF_MEMORY"),
        gl::STACK_UNDERFLOW => Some("GL_STACK_UNDERFLOW"),
        gl::STACK_OVERFLOW => Some("GL_STACK_OVERFLOW"),
        gl::CONTEXT_LOST => Some("GL_CONTEXT_LOST"),
        _ => Some("Unknown glGetError return value")
    }
}*/
