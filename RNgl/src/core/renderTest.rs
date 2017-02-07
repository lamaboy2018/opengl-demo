use core::Type::Type;

use glx::*;
use core::api::Api;

use core::common::*;
use core::buffer::*;
use core::buffer::vbo::*;
use core::img::*;
use core::shader::*;
use core::texture::*;
use core::draw::*;
use  core::vertexArrays::*;
// pub use gl;


pub fn readering(api: Api) {
    println!(" readering = {:?}  ",api );

    unsafe {
        match api.name.as_ref() {
            //   "eglSwapBuffers()" | " eglSwapBuffers()" => { println!(" p = {} ", api.name); }
            // common  - - - - - - - - >>>
/*            "glDisable" =>  glDisable(api),
            "glEnable" => glEnable(api),
            "glBlendFunc" => glBlendFunc(api),

            // Clearing the Buﬀers
            "glClear" => glClear(api),
            "glClearColor" => glClearColor(api),
            // Clearing the Buﬀers*/

            "glViewport" => glViewport(api),

            /// vertureArrays
           // "glVertexAttribPointer" => glVertexAttribPointer(api),
          //  "glDisableVertexAttribArray" => glDisableVertexAttribArray(api),
           // "glEnableVertexAttribArray" => glEnableVertexAttribArray(api),
            /// vertureArrays

/*
            ///  common  - - - - - - - - >>>
            /// shadet  - - - - - - - - >>>
            "glCreateShader" => glCreateShader(api),
            "glShaderSource" => glShaderSource(api),
            "glCompileShader" => glCompileShader(api),
            "glCreateProgram" => glCreateProgram(api),
            "glAttachShader" => glAttachShader(api),
            "glBindAttribLocation" => glBindAttribLocation(api),
            "glGetUniformLocation" => glGetUniformLocation(api),
            "glLinkProgram" => glLinkProgram(api),
            "glUseProgram" => glUseProgram(api),
            "glUniformMatrix4fv" => glUniformMatrix4fv(api),
            "glUniform4fv" => glUniform4fv(api),
            "glUniform4f" => glUniform4f(api),
            "glUniform1i" => glUniform1i(api),
            /// shadet  - - - - - - - - >>>

            /// buffet - - - - - - - - >>>
            "glUnmapBuffer" => {
                glUnmapBuffer(api);
            }
            "glBufferSubData" => glBufferSubData(api),
            "glGenBuffers" => glGenBuffers(api),
            "glBindBuffer" => glBindBuffer(api),
            "glBufferData" => glBufferData(api),
            /// buffet - - - - - - - - >>>
            ///  image  - - - - - - - - >>>

            "glTexSubImage2D" => glTexSubImage2D(api),
            //"glPixelStorei" =>{ glPixelStorei(api)}
            ///  image  - - - - - - - - >>>

            ///  Textures - - - - >>
            "glGenTextures" => glGenTextures(api),
            "glBindTexture" => glBindTexture(api),
            "glTexParameteri" => glTexParameteri(api),
            //     "eglSwapBuffers()" =>{window.swap_buffers();           }*/
            ///  Textures - - - - >>
            ///
            /// glDrawArrays

            "glDrawArrays" =>glDrawArrays(api),
            ///
            _ => {}
        }
    }

}
