pub mod gl31;
pub mod gles32;
pub mod gles31;
pub mod gles30;
pub mod gl45;


pub static  DFile : &'static str = "src/glx/gl45/mod.rs";
pub static  vglsl : &'static str = "#version 450  \n";

pub use  glx::gl45::types::*;
pub use  glx::gl45 as gl;

/*pub static  DFile : &'static str = "src/glx/gles32/mod.rs";
pub static  vglsl : &'static str = "#version 320 es \n";

pub use  glx::gles32::types::*;
pub use  glx::gles32 as gl;*/
/***************************
http://blog.csdn.net/blues1021/article/details/51322134

OpenGL ES 1.0 <=> OpenGL 1.3    Android 1.0
OpenGL ES 1.1 <=> OpenGL 1.5    Android 2.2 (API level 8) high
OpenGL ES 2.0 <=> OpenGL 2.0    Shading language 1.10
OpenGL ES 3.0 <=> OpenGL  3.2  3.3   4.0 - 4.3   OpenGL 4.3 provides full compatibility with OpenGL ES 3.0.
OpenGL ES 3.1 <=>  OpenGL 4.4 Shader Mode 5.0 Android 5.0 (API level 21) and higher
OpenGL ES 3.2  <=> OpenGL  4.5 Shader Mode 5.0功能 Android (since version 6.0)
*/

/*
OpenGL 3.2: #version 150 core
OpenGL 3.3: #version 330 core
OpenGL 4.0: #version 400 core
OpenGL 4.1: #version 410 core
OpenGL 4.2: #version 420 core
OpenGL 4.3: #version 430 core
OpenGL 4.4: #version 440 core*/
