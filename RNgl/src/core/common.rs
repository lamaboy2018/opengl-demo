use core::Type::Type;

use core::api::Api;
use glx::*;

pub unsafe fn glDisable(api: Api) {


    gl::Disable(api.args[0].getGLenum());

}
pub unsafe  fn  glEnable(api:Api){
    gl::Enable(api.args[0].getGLenum());
}

pub unsafe fn glClear(api: Api) {

    // ,mask: GLbitfield
    gl::Clear(api.args[0].getGLbitfield());
}

pub unsafe fn glBlendFunc(api: Api) {

    gl::BlendFunc(api.args[0].getGLenum(), api.args[1].getGLenum());
}

pub unsafe fn glGetIntegerv(api: Api) {
    //  gl::GetIntegerv();
}

pub unsafe fn glGetString(api: Api) {
    // glGetString
}


pub unsafe fn glClearColor(api: Api) {
    // ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat)
    let args = api.args;
    gl::ClearColor(args[0].getGLfloat(),
                   args[1].getGLfloat(),
                   args[2].getGLfloat(),
                   args[3].getGLfloat());
}

pub unsafe fn glViewport(api: Api) {
 //  gl::Viewport(0, 0, 500, 300);
    let args = api.args;
    gl::Viewport(args[0].getGLint(),
                 args[1].getGLint(),
                 args[2].getGLsizei(),
                 args[3].getGLsizei());
}
