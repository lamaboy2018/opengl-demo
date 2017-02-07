// use core::Type::Type;
use glx::*;
use core::api::Api;


pub unsafe fn glTexSubImage2D(api: Api) {
    // gl::TexSubImage2D(getGLenum!(api.args[0]),getGLint!(api.args[1]),getGLint!(api.args[2]),getGLint!(api.args[3]),getGLsizei!(api.args[4]),getGLenum!(api.args[5]),)
}

pub unsafe fn glPixelStorei(api: Api) {
    gl::PixelStorei(api.args[0].getGLenum(), api.args[1].getGLint());
}
