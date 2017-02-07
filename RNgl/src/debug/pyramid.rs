
use  glx::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use core::api::*;
use debug::color::*;
use core::renderobj::*;

use std::os::raw::c_void;


use core::common::*;
use core::buffer::*;
use core::buffer::vbo::*;
use core::img::*;
use core::shader::*;
use core::texture::*;
use core::draw::*;
use  core::vertexArrays::*;


static   vshader :&'static str   ="#version 450
            uniform mat4 uMVPMatrix;
            in vec4 vPosition;
            void main()
            {
            gl_Position = uMVPMatrix * vPosition;
            };";

static  fshader :&'static str =
"#version 450\n\
precision mediump float;\n\
uniform vec4 vColor;\n\
out vec4 fragColor;\n\
void main()\n\
{\n\
  fragColor = vColor;\n\
}";



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



pub struct  pyramid{
    mVerticesData:[GLfloat; 54],
    mvs : &'static str,
    mfs : &'static str,
    program: GLuint,
    mMVPMatrixHandle : GLint,
    mColorHandle: GLint,
    Render :render,

/*

    colorcyan: [GLfloat;4],
    colorblue: [GLfloat;4],
    colorred: [GLfloat;4],
    colorgreen: [GLfloat;4],
    coloryellow: [GLfloat;4],
*/


}

impl pyramid {
    pub fn new()->pyramid{
        let mut  r = render::new();
        pyramid{
            mVerticesData : VerticesData,
            mvs : vshader,
            mfs : fshader,
            program : 0/*unsafe{ gl::CreateProgram()}*/,
            mMVPMatrixHandle: 0,
            mColorHandle : 0,
            Render:r,
            //color
            //colorblue : blue(),
          //  colorcyan :cyan(),
        }
    }
  pub  fn complile_shader(&mut self , ty: GLenum) -> GLuint {
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);


            match ty{

                gl::VERTEX_SHADER =>{
                    let c_str = CString::new(self.mvs.as_bytes()).unwrap();
                    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
                }
                gl::FRAGMENT_SHADER =>{
                    let c_str = CString::new(self.mfs.as_bytes()).unwrap();
                    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
                }
                _=>{}
            }


            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);

                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                println!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
            } else {
                println!("build - - - - - - - - - -  - - - -> ok src len ");
            }
        }
        return shader;
    }

  pub  fn link_program(&self,vs: GLuint, fs: GLuint) -> GLuint {
        unsafe {
            let program = self.program;
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                println!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
            } else {
                println!("build - - - - - - - - - -  - - - -> ok");
            }
            return program;
        }
    }
    pub fn getmMVPMatrixHandle(&self)->GLint{

        return  self.mMVPMatrixHandle;
    }

   pub  fn draw(&mut self ,api:Api, mvpMatrix: &mut [GLfloat;16]) {
       unsafe {
/*           gl::Clear(gl::COLOR_BUFFER_BIT |gl::DEPTH_BUFFER_BIT);
           gl::Enable(gl::DEPTH_TEST);*/

           let mProgramObject = self.program;
         //  gl::UseProgram(mProgramObject);

            self.mMVPMatrixHandle =  gl::GetUniformLocation(mProgramObject, CString::new("uMVPMatrix").unwrap().as_ptr());
          //  self.mColorHandle =     gl::GetUniformLocation(mProgramObject, CString::new("vColor").unwrap().as_ptr());

          // println!("mColorHandle ={}",self.mColorHandle);
           //gl::UniformMatrix4fv(self.mMVPMatrixHandle, 1, gl::FALSE, readMatrix());
           let VERTEX_POS_INDX =0;
           //gl::VertexAttribPointer(VERTEX_POS_INDX, 3, gl::FLOAT, gl::FALSE, 0, self.mVerticesData.as_ptr() as * const GLvoid );
           //gl::EnableVertexAttribArray(VERTEX_POS_INDX);

           let mut  startPos = 0;
           let  verticesPerface = 3;

           // self.Render.draw(self.mMVPMatrixHandle,api);

           self.draw2(api,mvpMatrix);


/*
           gl::Uniform4fv(self.mColorHandle, 1, blue().as_ptr());
           gl::DrawArrays(gl::TRIANGLES, startPos, verticesPerface);
           startPos += verticesPerface;

           gl::Uniform4fv(self.mColorHandle, 1, cyan().as_ptr());
           gl::DrawArrays(gl::TRIANGLES, startPos, verticesPerface);
           startPos += verticesPerface;

           gl::Uniform4fv(self.mColorHandle, 1, red().as_ptr());
           gl::DrawArrays(gl::TRIANGLES, startPos, verticesPerface);
           startPos += verticesPerface;

           gl::Uniform4fv(self.mColorHandle, 1, gray().as_ptr());
           gl::DrawArrays(gl::TRIANGLES, startPos, verticesPerface);
           startPos += verticesPerface;

           gl::Uniform4fv(self.mColorHandle, 1, yellow().as_ptr());
           gl::DrawArrays(gl::TRIANGLES, startPos, verticesPerface);
           startPos += verticesPerface;
*/

        //   gl::Uniform4fv(self.mColorHandle, 1, yellow().as_ptr());
           //gl::DrawArrays(gl::TRIANGLES, 15, verticesPerface);
           //startPos += verticesPerface;


       }

    }
 pub   fn draw2(&mut self ,api:Api, mvpMatrix: &mut [GLfloat;16])
    {
        unsafe {
            match api.name.as_ref(){
                "glUniformMatrix4fv" =>{
                    let args = api.args;
                    let fv = args[3].getGLfloatxfv();
                    // let remotlocation = args[0].getGLint();
                    //  let location = uLocationMapRead(remotlocation).getGLuint();
                    //let location = GLintMapRead(getUniformMap(),remotlocation);
                        WriteMatrix(fv);
                    gl::UniformMatrix4fv(self.mMVPMatrixHandle, args[1].getGLsizei(), args[2].getGLboolean(), readMatrix());

                }
                "glVertexAttribPointer" => {
                    //VertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void
                    let args= api.args;
                  //  gl::VertexAttribPointer(0,args[1].getGLint(),args[2].getGLenum(),args[3].getGLboolean(),args[4].getGLsizei(),args[5].getGLfloatxfv() as * const c_void);
                    gl::VertexAttribPointer(0,args[1].getGLint(),args[2].getGLenum(),args[3].getGLboolean(),args[4].getGLsizei(),self.mVerticesData.as_ptr() as * const GLvoid);
                   // glcheckGlError("glVertexAttribPointer");
                },
              //  "glDisableVertexAttribArray" => glDisableVertexAttribArray(api),
                "glEnableVertexAttribArray" => {
                    gl::EnableVertexAttribArray(0);
                },
             //  "glDisable" =>  glDisable(api),
               // "glEnable" => glEnable(api),
            //    "glBlendFunc" => glBlendFunc(api),

            // Clearing the Buﬀers
              //  "glClear" => glClear(api),
              //  "glClearColor" => glClearColor(api),
            // Clearing the Buﬀers

                "glViewport" => glViewport(api),

            /// vertureArrays
/*                "glVertexAttribPointer" => glVertexAttribPointer(api),
                "glDisableVertexAttribArray" => glDisableVertexAttribArray(api),
                "glEnableVertexAttribArray" => glEnableVertexAttribArray(api),*/
            /// vertureArrays


            ///  common  - - - - - - - - >>>
            /// shadet  - - - - - - - - >>>
            "glCreateShader" => glCreateShader(api),
            "glShaderSource" => glShaderSource(api),
            "glCompileShader" => glCompileShader(api),
            "glCreateProgram" => {
                let args = api.args;
                let remoteprogram = args[0].getGLuint();
                //let mut program: GLuint = 0;
                self.program = gl::CreateProgram();
                ProgramMapWrite(remoteprogram,self.program);
            }
            "glAttachShader" => glAttachShader(api),
            "glBindAttribLocation" => glBindAttribLocation(api),
            "glGetUniformLocation" => glGetUniformLocation(api),
            "glLinkProgram" => glLinkProgram(api),
            "glUseProgram" => glUseProgram(api),
   //         "glUniformMatrix4fv" => glUniformMatrix4fv(api),
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
            //     "eglSwapBuffers()" =>{window.swap_buffers();           }
            ///  Textures - - - - >>
            ///
            /// glDrawArrays
            "glDrawArrays" =>glDrawArrays(api),

                _ => {}
            }
        }
    }
}
