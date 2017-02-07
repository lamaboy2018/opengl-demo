
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
use utils::Lazy::*;

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


//#[derive(Clone,Default)]
pub struct  translate{
    mVerticesData:[GLfloat; 54],
    mvs : &'static str,
    mfs : &'static str,
    mvsharder: GLuint,
    mfsharder: GLuint,
    remvsharder: GLuint,
    remfsharder: GLuint,
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
unsafe impl ::std::marker::Sync for translate {}

impl translate {
    pub fn new()->Self{
        let mut  r = render::new();
        translate{
            mVerticesData : VerticesData,
            mvs : vshader,
            mfs : fshader,
            mvsharder: 0,
            mfsharder: 0,
            remvsharder: 0,
            remfsharder: 0,
            program : 0/*unsafe{ gl::CreateProgram()}*/,
            mMVPMatrixHandle: 0,
            mColorHandle : 0,
            Render:r,
            //color
            //colorblue : blue(),
            //  colorcyan :cyan(),
        }
    }

    pub fn mutinstance() -> &'static mut  Self {
        static mut instance: MLazy<translate> = MLazy {
            lock:ONCE_INIT,
            ptr: 0 as *mut  translate,
        };
        unsafe {
            instance.get(translate::new)
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
            self.mColorHandle =     gl::GetUniformLocation(mProgramObject, CString::new("vColor").unwrap().as_ptr());

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
                    //   gl::VertexAttribPointer(0,args[1].getGLint(),args[2].getGLenum(),args[3].getGLboolean(),args[4].getGLsizei(),args[5].getGLfloatxfv() as * const c_void);
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
                "glCreateShader" =>{
                    let args = api.args;
                    let stype = args[0].getGLenum();
                    let remoteSharder = args[1].getGLuint();

                    match stype{
                        gl::VERTEX_SHADER =>{
                            self.mvsharder = gl::CreateShader(stype);
                            self.remvsharder = remoteSharder;
                        },
                        gl::FRAGMENT_SHADER =>{
                            self.mfsharder = gl::CreateShader(stype);
                            self.remfsharder = remoteSharder;
                        },
                        _ =>{ },
                    }




                  } ,
                "glShaderSource" => {
                    let args = api.args;
                   // let cstr = args[2].getGLcharptr();
                    let remoteshader = args[0].getGLuint();
                   // let shader = SharderMapRead(remoteshader).setString(args[2].getString()).getGLuint();

                    // let shader =  GLuintMapRead(getShaderMap(),remoteshader);
                    if remoteshader == self.remfsharder {

                        let c_str = CString::new(self.mfs.as_bytes()).unwrap();
                        gl::ShaderSource(self.mfsharder, 1, &c_str.as_ptr(), 0 as *const GLint);//

                    }else if remoteshader == self.remvsharder {

                        let c_str = CString::new(self.mvs.as_bytes()).unwrap();
                        gl::ShaderSource(self.mvsharder, 1, &c_str.as_ptr(), 0 as *const GLint);//
                    }


                },
                "glCompileShader" => {
                    let args = api.args;
                    let remoteshader = args[0].getGLuint();
                 //   let shader = SharderMapRead(remoteshader).getGLuint();
                    let mut shader = 0;
                   // println!("map glCompileShader  get <{},{}> ", remoteshader, shader);
                    if  remoteshader == self.remvsharder{
                        shader = self.mvsharder;
                        gl::CompileShader(shader);

                    }else if remoteshader == self.remfsharder
                        {
                            shader = self.mfsharder;
                            gl::CompileShader(shader);
                        }


                    let mut compilation_success: GLint = mem::uninitialized();
                    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compilation_success);

                    if compilation_success == 0 {
                        let mut log_length: GLint = mem::uninitialized();
                        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

                        let mut error_log: Vec<u8> = Vec::with_capacity(log_length as usize);

                        gl::GetShaderInfoLog(shader,
                                             log_length,
                                             &mut log_length,
                                             error_log.as_mut_ptr() as *mut GLchar);

                        error_log.set_len(log_length as usize);
                        let log = String::from_utf8(error_log).unwrap();



                       // println!(" glCompileShader  shader ={:?}  \n source= {}",shader,SharderMapRead(remoteshader).getString());

                        panic!("GetShaderInfoLog ={}", log);

                    }else {
                        println!("GetShaderInfoLog  compilation_success ={}", compilation_success);
                    }
                },
                "glCreateProgram" => {
                    let args = api.args;
                    let remoteprogram = args[0].getGLuint();
                    //let mut program: GLuint = 0;
                    self.program = gl::CreateProgram();
                  //  ProgramMapWrite(remoteprogram,self.program);
                },
                "glAttachShader" => {
                    let args = api.args;

                    let remoteshader = args[1].getGLuint();
                    let mut shader : GLuint =0;

                    if  remoteshader == self.remvsharder{
                        shader = self.mvsharder;
                        gl::AttachShader(self.program, shader);//

                    } else
                    if remoteshader == self.remfsharder
                        {
                            shader = self.mfsharder;
                            gl::AttachShader(self.program, shader);//
                        }

                },
               // "glBindAttribLocation" => glBindAttribLocation(api),

                "glGetUniformLocation" => {
                    let args = api.args;

                    let str = args[1].getString();
                    let cstr = args[1].getGLcharptr();
                    if str.contains("uMVPMatrix") {

                        self.mMVPMatrixHandle = gl::GetUniformLocation(self.program, cstr);
                    }
                    else if str.contains("vColor"){
                        self.mColorHandle = gl::GetUniformLocation(self.program, cstr);
                    }




                },
                "glLinkProgram" => {
                    let program = self.program;
                    gl::LinkProgram(program);

                    let mut status = gl::FALSE as GLint;
                    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
                    if status != (gl::TRUE as GLint) {
                        let mut len: GLint = 0;
                        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                        let mut buf = Vec::with_capacity(len as usize);
                        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                        println!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
                    }
                },
                "glUseProgram" =>{gl::UseProgram(self.program)},

                //         "glUniformMatrix4fv" => glUniformMatrix4fv(api),
                "glUniform4fv" =>{
                    let args = api.args;
                    let fv = args[2].getGLfloatxfv();
                  //  let fv = args[2].getGLfloatxfv();
                 //   static  mut  once :i32 =0;
/*                    if once < 2{
                        gl::Uniform4fv(self.mColorHandle, args[1].getGLsizei(), red().as_ptr());
                        once +=1;
                        return;
                    }
                    if once == 2{
                        once =0;
                        gl::Uniform4fv(self.mColorHandle, args[1].getGLsizei(),fv);
                    }*/
                    gl::Uniform4fv(self.mColorHandle, args[1].getGLsizei(),fv);

                } ,

                /// glDrawArrays
                "glDrawArrays" =>{
                    let args = api.args;
                    // DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei)
                    gl::DrawArrays(args[0].getGLenum(),args[1].getGLint(),args[2].getGLsizei());
                },

                _ => {}
            }
        }
    }
}
