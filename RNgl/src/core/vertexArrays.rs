use  glx::*;
use core::Type::Type;
use core::api::Api;
use std::os::raw::c_void;
use core::error::glcheckGlError;


const  size: GLfloat = 0.4;

static    VerticesData: [GLfloat; 54] = [
    ////////////////////////////////////////////////////////////////////
    // top
    ////////////////////////////////////////////////////////////////////
    // FRONT
    0.0, size, 0.0, // top
    - size, - size, size, // front-left
    size, - size, size, // front-right
    // RIGHT
    0.0, size, 0.0, // top
    size, - size, size, // front-right
    size, - size, - size, // back-right
    // BACK
    0.0, size, 0.0, // top
    size, -size, - size, // back-right
    - size, - size, - size, // back-left
    // LEFT
    0.0, size, 0.0, // top
    - size, - size, - size, // back-left
    -size, - size, size, // front-left

    ////////////////////////////////////////////////////////////////////
    // BOTTOM
    ////////////////////////////////////////////////////////////////////
    // Triangle 1
    - size, - size, - size, // back-left
    - size, -size, size, // front-left
    size, - size, size, // front-right
    // Triangle 2
    size, - size, size, // front-right
    size, - size, - size, // back-right
    - size, -size, - size // back-left
];


pub unsafe fn glVertexAttribPointer(api: Api) {
    //VertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void
      let args= api.args;
    // gl::VertexAttribPointer(args[0].getGLuint(),args[1].getGLint(),args[2].getGLenum(),args[3].getGLboolean(),args[4].getGLsizei(),args[5].getGLfloatxfv() as * const c_void);
     gl::VertexAttribPointer(args[0].getGLuint(),args[1].getGLint(),args[2].getGLenum(),args[3].getGLboolean(),args[4].getGLsizei(),VerticesData.as_ptr() as * const c_void);
    glcheckGlError("glVertexAttribPointer");
}
pub unsafe fn glDisableVertexAttribArray(api: Api) {
    //DisableVertexAttribArray(index: types::GLuint)
      gl::DisableVertexAttribArray(api.args[0].getGLuint());
}

pub unsafe fn  glEnableVertexAttribArray(api :Api){
    gl::EnableVertexAttribArray(0/*api.args[0].getGLuint()*/);
    glcheckGlError("glEnableVertexAttribArray");
}