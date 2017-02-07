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

pub fn draw(api:Api){
unsafe {
match api.name.as_ref(){
// common  - - - - - - - - >>>
"glDisable" =>  glDisable(api),
"glEnable" => glEnable(api),
"glBlendFunc" => glBlendFunc(api),

 //  "glViewport" => glViewport(api),

/// vertureArrays
"glVertexAttribPointer" => glVertexAttribPointer(api),
"glDisableVertexAttribArray" => glDisableVertexAttribArray(api),
"glEnableVertexAttribArray" => glEnableVertexAttribArray(api),
/// vertureArrays


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


"glDrawArrays" =>glDrawArrays(api),

_ => {}
}
}
}