use glx::*;
use std::mem;
pub use std::os::raw;
pub type fptr =*const raw::c_void;
pub enum  glcall{
    glBindAttribLocation(fptr),
   // eglGetUniformLocation(FnOnce()->T),
}

impl glcall{
    pub fn new()-> Self{
        unsafe {
            glcall::glBindAttribLocation(gl::ActiveTexture as fptr)
        }

    }
}