#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

#![allow(unused_must_use)]
#![allow(overflowing_literals)]

extern crate glfw;

#[macro_use]
extern crate cgmath;
#[macro_use]
extern crate glm;
mod  glx;
mod  camera;
use glfw::*;


use  glx::*;
//use camera::camera;


use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use cgmath::prelude::*;
pub use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::conv::*;
use glm::*;
// Vertex data
static VERTEX_DATA: [GLfloat; 27] = [
    0.0,  0.5, -0.5,
    0.5, -0.5, 0.5,
    -0.5, -0.5, 0.5,

    0.0,  0.5, -0.5,
    0.5, -0.5, 0.5,
    0.5, 0.25, -0.25,

    0.0,  0.5,  -0.5,
    -0.5, -0.5, 0.5,
    0.5, 0.25,-0.25
];
//
// Shader sources
static VS_SRC: &'static str =
"#version 450  \n\
 uniform mat4 uMVPMatrix;\n\
 uniform mat4 uMLookAt;\n\
 in vec3 position;\n\
 void main() {\n\
    gl_Position = vec4(position, 1.0) * uMLookAt  ;\n\
 }";

static FS_SRC: &'static str =
"#version 450 \n\
 uniform vec4 vColor;\n\
 out vec4 out_color;\n\
 void main() {\n\
    out_color = vColor;\n\
 }";

const   RED  : GLfloat = 0xFFFF0000 as GLfloat;
const  BLUE : GLfloat  = 0xFF0000FF as GLfloat;
const  YELLOW  : GLfloat     = 0xFFFFFF00 as GLfloat;
const  CYAN     : GLfloat     = 0xFF00FFFF as GLfloat ;
const GRAY      : GLfloat    = 0xFF888888 as GLfloat ;


pub fn red() ->[GLfloat;4]{
    [
        /*        RED / 255.0,
                RED / 255.0,
                RED / 255.0,*/
        1.0,
        0.0,
        0.0,

        1.0
    ]
}


pub fn blue() ->[GLfloat;4]{
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

pub fn cyan() ->[GLfloat;4]{
    [
        128.0 / 255.0,
        128.0  / 255.0,
        128.0  / 255.0,
        1.0
    ]
}

pub fn gray() ->[GLfloat;4]{
    [
        /*        GRAY / 255.0,
                GRAY / 255.0,
                GRAY / 255.0,*/
        0.0,
        1.0,
        0.0,

        1.0
    ]
}

pub fn yellow() ->[GLfloat;4]{
    [
        1.0 ,
        1.0,
        1.0,
        1.0
    ]
}


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint { unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = Vec::with_capacity(len as usize);
        buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
        panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
    }
    program
} }



static mut  posy:f32 = 1.0;
static mut  posx:f32 = 1.0;
static mut  posz:f32 = -1.0;

fn main() {
    let mut px =0.5;
    let mut py =0.25;
    let  mut xyz : [GLfloat; 54] = [
        0.0,  0.5, 0.0,
        -0.5, -0.5, 0.5,
        0.5, -0.5, 0.5,

        0.0,  0.5, 0.0,
        0.5, -0.5, 0.5,
        0.5, -0.5, -0.5,

        0.0,  0.5,  0.0,
        0.5, -0.5, -0.5,
        -0.5, -0.5,-0.5,

        0.0,  0.5,  0.0,
        -0.5, -0.5, -0.5,
        -0.5, -0.5,0.5,



        -0.5, -0.5, -0.5,
        -0.5, -0.5,0.5,
        0.5, -0.5,0.5,

        0.5, -0.5, 0.5,
        0.5, -0.5,-0.5,
        -0.5, -0.5,-0.5,

    ];

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window ,events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
   // let (mut window2 ,events2) = glfw.create_window(80, 60, "Hello this is window", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");


    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.make_current();
    glfw::WindowHint::ContextVersion(3, 1);
    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::ContextVersion(4, 1);
    glfw::WindowHint::ContextVersion(4, 5);

    // Create GLSL shaders
    let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut vao = 0;
    let mut vbo = 0;
    let mut color_attr : GLint =0;
    let mut mMVPMatrixHandle :GLint =0;
    let mut mMLookAtHandle : GLint =0;
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (xyz.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&xyz[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
        color_attr = gl::GetUniformLocation(program,CString::new("vColor").unwrap().as_ptr());
        mMVPMatrixHandle = gl::GetUniformLocation(program,CString::new("uMVPMatrix").unwrap().as_ptr());


        mMLookAtHandle = gl::GetUniformLocation(program,CString::new("uMLookAt").unwrap().as_ptr());
// Matrix4::look_at(Point3::new(0.0,0.0,0.0),Point3::new( -3.0, 0.0, 0.0),Vector3::new(0.0,1.0,0.0));


  //     println!(" color_attr = {:?}",arr);
        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program,    CString::new("position").unwrap().as_ptr());

       // println!(" pos_attr = {}",pos_attr);
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 3, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());

    }

    let  mut mViewMatrix : [GLfloat;16] =[
        -3.42659,               0.02738219,         -0.023744134,               -0.022585884,
        0.04622666,             1.9997377,          0.009685329,                0.009212874,
        -0.07784159,            -0.0178102,         1.0509692,                  0.99970245,
        0.0,                    0.0,                1.1025641,                  3.0
    ];

    let  mut mone : [GLfloat;16] =[
        1.0,               1.0,         1.0,               1.0,
        1.0,             1.0,          1.0,                1.0,
        1.0,            1.0,         1.0,                  1.0,
        1.0,                    1.0,                1.0,                  1.0
    ];
    let  mut defmat : Matrix4<f32>;
unsafe {
     defmat =     camera::camera::roatoz(posx,posy,posz);
}

    while !window.should_close() {

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            //   gl::UniformMatrix4fv(mMVPMatrixHandle, 1, false, mvpMatrix, 0);

/*            matrix::Matrix4::new(
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0,
                0.0,0.0,0.0,0.0
            );*/
         //   let mut mRotationMatrix     :Matrix4<f32> =    Matrix4::from_scale(0.0);
          //  let mut ViewMatrix          :Matrix4<f32> = Matrix4::look_at(Point3::new(0.0,0.0,0.0),Point3::new( -3.0, 0.0, 0.0),Vector3::new(0.0,1.0,0.0));



          //  println!("Matrix4<f32> ={:?}",ViewMatrix );
           // gl::PushMatrix();
           // gl::Translatef(px,0.0,0.0);
            //gl::Rotatef(px ,1.0,1.0,0.0);
           // println!("Matrix4<f32> ={:?}",px );
            gl::UniformMatrix4fv(mMVPMatrixHandle, 1, gl::FALSE, mViewMatrix.as_ptr());


            let la=   camera::camera::lookAt(Point3::new(posx,posy,posz),Point3::new( 0.0, 0.0, 0.0),Vector3::new(0.0,1.0,0.0));
            //let v =array4x4(la);
           // println!("array4x4(la) ={:?} la={:?}",v,la);
            let arr = camera::camera::roatoz(posx,posy,posz);

           // let arr = arr * la;

            gl::Viewport(0,0,800,600);


            gl::UniformMatrix4fv(mMLookAtHandle, 1, gl::FALSE, arr.as_ptr());

          //  gl::Uniform4fv(color_attr, 1, yellow().as_ptr());
         //   gl::DrawArrays(gl::TRIANGLES, 12, 3);

            gl::Uniform4fv(color_attr, 1, cyan().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 9, 3);

            gl::Uniform4fv(color_attr, 1, gray().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 6, 3);
            // Draw a triangle from the 3 vertices
            gl::Uniform4fv(color_attr, 1, red().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 3, 3);
            gl::Uniform4fv(color_attr, 1, blue().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);


            gl::Viewport(0,0,200,200);
                //let mat = camera::camera::roatoz(90.0,90.0,90.0);
           let mat=  camera::camera::lookAt(Point3::new(posx,posx /posx.tan(),-0.025),Point3::new( 0.0, 0.0, 0.0),Vector3::new(0.0,3.0,0.0))  ;

            gl::UniformMatrix4fv(mMLookAtHandle, 1, gl::FALSE, mat.as_ptr());

           // gl::Uniform4fv(color_attr, 1, yellow().as_ptr());
           // gl::DrawArrays(gl::TRIANGLES, 12, 3);

            gl::Uniform4fv(color_attr, 1, cyan().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 9, 3);

            gl::Uniform4fv(color_attr, 1, gray().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 6, 3);
            // Draw a triangle from the 3 vertices
            gl::Uniform4fv(color_attr, 1, red().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 3, 3);
            gl::Uniform4fv(color_attr, 1, blue().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            //gl::PopMatrix();


        }
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event,&mut px,&mut py);
        }

        window.swap_buffers();
      //  window2.swap_buffers();
    }



    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent ,px :&mut f32, py:&mut f32) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)

        }
        glfw::WindowEvent::MouseButton(button,action,Modifiers)=>{
            println!("MouseButton  button = {:?}  action = {:?} Modifiers = {:?}",button,action,Modifiers);
        }
        glfw::WindowEvent::Scroll(x,y)=>{
        println!("scroll x ={},y= {}",x,y);
            unsafe {
               // posy = y /2.0;
                posz  += y as f32 ;
            }
        }
/*        glfw::WindowEvent::MouseButton()(x,y)=>{
            println!("scroll x ={},y= {}",x,y);
        }*/
        glfw::WindowEvent::CursorPos(xpos, ypos) => {

                let x :f32  =   xpos as f32 / 80.0;
                let y :f32  =   ypos as f32 / 60.0;
                unsafe {
                    posy = y /2.0;
                    posx =  x /2.0;
                }
                *px = x;
                *py = y ;

          //  println!("Cursor position: ({:?}, {:?})", *px, *py);
            /*            let mut  xyz: [GLfloat; 9] = [

                            0.0,  0.5,  -0.5,
                            -0.5, -0.5, 0.5,
                            ( xpos as f32 ) / 800.0 , (ypos as f32) / 600.0 ,-0.25
                        ];*/

/*            gl::Uniform4fv(color_attr, 1, blue().as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);*/
        }

        _ => {}
    }
}
