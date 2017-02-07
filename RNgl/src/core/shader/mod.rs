use std::mem;
use core::Type::Type;

use glx::*;


use core::api::Api;

use std::ffi::CString;

//use core::datamap::*;
use core::error::glcheckGlError;
//use core::renderobj::*;
use core::engine::*;

pub unsafe fn glCreateShader(api: Api) {
    let args = api.args;
    let stype = args[0].getGLenum();
    let remoteSharder = args[1].getGLuint();

    let mut shader = gl::CreateShader(stype);
    if shader == 0 {
        panic!("Invalid number: {}", shader);
    }
    println!("map glCreateShader   insert<{},{}> ", remoteSharder, shader);
    let  obj = sharder::new(stype).setGLuint(shader);
    SharderWriteMap(remoteSharder,obj);
  //  GLuintMapWrite(getShaderMap(),remoteSharder, shader);
    glcheckGlError("glCreateShader");




}

pub unsafe fn glGetShaderiv(api: Api) {
    // gl::GetShaderiv()
}


static  vShaderStr : &'static str=
"#version 300 es\n
uniform mat4 uMVPMatrix;\n
in vec4 vPosition;\n
void main()\n
{\n
   gl_Position = uMVPMatrix * vPosition;\n
}\n ";

static  fShaderStr : &'static str= "#version 300 es precision mediump float; uniform vec4 vColor; out vec4 fragColor; void main(){ fragColor = vColor; }";

/*
static  fShaderStr : &'static str= "#version 300 es
                    precision mediump float;\n
                    uniform vec4 vColor;\n
                    out vec4 fragColor;\n
                    void main()\n
                   {\n
                    fragColor = vColor;\n
                    }\n ";
*/

pub unsafe fn glShaderSource(api: Api) {
    // ShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint)
    //  let Cstr = getGLcharptr!(api.args[2]);
    let args = api.args;
    let cstr = args[2].getGLcharptr();
    let remoteshader = args[0].getGLuint();
    let shader = SharderReadMap(remoteshader).setString(args[2].getString()).getGLuint();

  // let shader =  GLuintMapRead(getShaderMap(),remoteshader);

    gl::ShaderSource(shader, args[1].getGLsizei(), &cstr, 0 as *const GLint);//
    glcheckGlError("glShaderSource");

   // StringMapWrite(shader,args[2].getCString());
    println!("map glShaderSource   insert<{},{}>  \n {:?}", remoteshader, shader,args[2].getString());


}


pub unsafe fn glCompileShader(api: Api) {
    // gl::CompileShader(api.args[0].getGLuint());
    let args = api.args;
    let remoteshader = args[0].getGLuint();
    let shader = SharderReadMap(remoteshader).getGLuint();

    println!("map glCompileShader  get <{},{}> ", remoteshader, shader);
    gl::CompileShader(shader);

    let mut compilation_success: GLint = mem::uninitialized();
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compilation_success);

    if compilation_success == 0 {
        let mut log_length: GLint = mem::uninitialized();
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

        let mut error_log: Vec<u8> = Vec::with_capacity(log_length as usize);
        /*                    println!("COMPILE_STATUS ={} error_log ={} ",
                                     compilation_success,
                                     log_length);*/
        gl::GetShaderInfoLog(shader,
                             log_length,
                             &mut log_length,
                             error_log.as_mut_ptr() as *mut GLchar);

        error_log.set_len(log_length as usize);
        let log = String::from_utf8(error_log).unwrap();



        println!(" glCompileShader  shader ={:?}  \n source= {}",shader,SharderReadMap(remoteshader).getString());

        panic!("GetShaderInfoLog ={}", log);

    }else {
        println!("GetShaderInfoLog  compilation_success ={}", compilation_success);
    }

}

pub unsafe fn glCreateProgram(api: Api) {
    // CreateProgram() -> types::GLuint
    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let mut program: GLuint = 0;
    program = gl::CreateProgram();
    programWriteMap(remoteprogram,program);

    println!("map glCreateProgram   insert<{},{}> ",  remoteprogram,        program);
    glcheckGlError("glCreateProgram");

}

pub unsafe fn glAttachShader(api: Api) {

    // AttachShader(program: types::GLuint, shader: types::GLuint)
    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let remoteshader = args[1].getGLuint();
    let program = programReadMap(remoteprogram);
    let shader  = SharderReadMap(remoteshader).getGLuint();

    println!("map glAttachShader  get< remoteprogram,program> <{},{:?}> ", remoteprogram, program);
    println!("map glAttachShader  get<remoteshader,shader> <{},{:?}> ",remoteshader,shader);
    gl::AttachShader(program, shader);//

    if  gl::IsProgram(program) == 0{
        println!(" glAttachShader is not a program");
    }
    glcheckGlError("glAttachShader");


}


pub unsafe fn glBindAttribLocation(api: Api) {
    // BindAttribLocation(program: types::GLuint, index: types::GLuint, name: *const types::GLchar)
    return;
    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let index = args[1].getGLuint();

    let program =  programReadMap(remoteprogram);
    let mut cptr = args[2].getGLcharptr();
    unsafe  {

        println!("map glBindAttribLocation  get<remoteprogram,program> = <{},{:?}>  {}",remoteprogram,program,args[2].getString());
    }

    gl::BindAttribLocation(program, 0,cptr );
    glcheckGlError("glBindAttribLocation");
}

pub unsafe fn glGetUniformLocation(api: Api) {
    // GetUniformLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint

    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let remotelocation = args[2].getGLint();
    let cstr = args[1].getGLcharptr();
    let mut location: GLint = 0;
    let program =  programReadMap(remoteprogram);
    location = gl::GetUniformLocation(program, cstr);
    println!("map glGetUniformLocation  get<remoteprogram,program> = <{},{:?}> ",remoteprogram,program);
    let  unlocation = uniformlocation::new(location).setString(args[1].getString());

    localWriteMap(remotelocation,unlocation);

    println!("map GetUniformLocation   insert< remotelocation,location> <{},{}> {} {} ",
             remotelocation,
             localReadMap(remotelocation).getGLint(),args[1].getString(),location);
    glcheckGlError("glGetUniformLocation");



}

pub unsafe fn glGetProgramiv(api: Api) {
    //  pub unsafe fn GetProgramiv(program: types::GLuint, pname: types::GLenum, params: *mut types::GLint)
/*    let mut params: GLint = 0;
    gl::GetProgramiv(api.args[0].getGLuint(),
                     api.args[1].getGLuint(),
                     &mut params);*/
}
pub unsafe fn glLinkProgram(api: Api) {
    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let program = programReadMap(remoteprogram);

    gl::LinkProgram(program);
    println!("map glLinkProgram  get<remoteprogram,program> = <{},{:?}> ",
             remoteprogram,
             program);
   if  0 == gl::IsProgram(program){
       println!(" glLinkProgram is not a program");
   }


    let mut success: GLint = mem::uninitialized();
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

    if success == 0 {
        let mut log_length: GLint = mem::uninitialized();
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

        let mut error_log: Vec<u8> = Vec::with_capacity(log_length as usize);
        /*                    println!("COMPILE_STATUS ={} error_log ={} ",
                                     compilation_success,
                                     log_length);*/
        gl::GetProgramInfoLog(program,
                             log_length,
                             &mut log_length,
                             error_log.as_mut_ptr() as *mut GLchar);

        error_log.set_len(log_length as usize);
        let log = String::from_utf8(error_log).unwrap();

        gl::DeleteProgram(program);
        panic!("GetProgramInfoLog ={}", log);

    }else {
        println!("GetProgramInfoLog success ={} ", success);
    }



}

pub unsafe fn glUseProgram(api: Api) {
    // UseProgram(program: types::GLuint)
    // gl::UseProgram(api.args[0].getGLuint());

    let args = api.args;
    let remoteprogram = args[0].getGLuint();
    let program  = programReadMap(remoteprogram);

    gl::UseProgram(program);
    println!("map glUseProgram  get<remoteprogram,program> = <{},{:?}> ",
             remoteprogram,
             program);


}

pub unsafe fn glUniformMatrix4fv(api: Api) {
    // UniformMatrix4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ()
    let args = api.args;
    let fv = args[3].getGLfloatxfv();
    let remotlocation = args[0].getGLint();
    let location = localReadMap(remotlocation).getGLint();
    //let location = GLintMapRead(getUniformMap(),remotlocation);
   // WriteMatrix(fv);

    gl::UniformMatrix4fv(location, args[1].getGLsizei(), args[2].getGLboolean(), fv/*undateMatrix(fv)*/);
    println!("map  glUniformMatrix4fv  get<remotlocation,location> = <{},{}> ",
             remotlocation,
            location);


}
/*pub unsafe fn _glUniformMatrix4fv(location:GLint,api: Api){
    let args = api.args;
    let fv = args[3].getGLfloatxfv();
   // let remotlocation = args[0].getGLint();
  //  let location = uLocationMapRead(remotlocation).getGLuint();
    //let location = GLintMapRead(getUniformMap(),remotlocation);
  //  WriteMatrix(fv);
   // gl::UniformMatrix4fv(location, args[1].getGLsizei(), args[2].getGLboolean(), readMatrix());
   // println!("map  glUniformMatrix4fv  get<remotlocation,location> = <{},{}> ",
    //         remotlocation,
    //         location);
}*/

 fn blue() ->[GLfloat;4]{
    [
        /*        BLUE / 255.0,
                BLUE / 255.0,
                BLUE / 255.0,*/
        0.0,
        0.0,
        1.0,
        1.0
    ]
}

pub unsafe fn glUniform4fv(api: Api) {
    // Uniform4fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat)
    let args = api.args;
    let fv = args[2].getGLfloatxfv();

    let remotlocation = args[0].getGLint() ;
    let location = localReadMap(remotlocation ).getGLint();


    println!("map  glUniform4fv  get<remotlocation,location> = <{},{:?}> ",
             remotlocation,
             location);
    gl::Uniform4fv(location, args[1].getGLsizei(),fv /*blue().as_ptr()*/);//fv

  /*  if let Ok(map) = getUniformMap().read() {

        match map.get(&remotlocation) {
            Some(location) => {
                println!("map  glUniform4fv  get<remotlocation,location> = <{},{:?}> ",
                         remotlocation,
                         *location);
                gl::Uniform4fv(*location, args[1].getGLsizei(), fv);
            }
            None => {
                println!("map glUniform4fv  get<remotlocation,location> = <{},{:?}> error ",
                         remotlocation,
                         map.get(&remotlocation));
            }
        }

    }*/
    //  let args = api.args;
    // gl::Uniform4f(args[0].getGLint(),args[1].getGLfloat(),args[2].getGLfloat(),args[3].getGLfloat(),args[4].getGLfloat());
}


pub unsafe fn glUniform4f(api: Api) {
    // Uniform4f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat)
    let args = api.args;
    gl::Uniform4f(args[0].getGLint(),
                  args[1].getGLfloat(),
                  args[2].getGLfloat(),
                  args[3].getGLfloat(),
                  args[4].getGLfloat());
}



pub unsafe fn glUniform1i(api: Api) {
    // Uniform1i(location: types::GLint, v0: types::GLint)
    let args = api.args;
    gl::Uniform1i(args[0].getGLint(), args[1].getGLint());
}
