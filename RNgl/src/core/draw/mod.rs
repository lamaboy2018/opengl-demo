use core::Type::Type;
use glx::*;
use core::api::Api;


pub unsafe fn glDrawArrays(api: Api) {
    let args = api.args;
    // DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei)
    gl::DrawArrays(args[0].getGLenum(),args[1].getGLint(),args[2].getGLsizei());
    //  gl::DrawArrays
}

pub unsafe fn glDrawElements(api: Api) {
    // gl::DrawElements
}
