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
use std::ffi::CString;
use std::collections::HashMap;
use std::sync::{RwLock,Arc};
use std::cell::{RefCell,UnsafeCell};
use libc::memcpy;
use libc::c_void;


use utils::Lazy::*;

#[derive(Clone,Debug)]
pub enum sharder{
    vertex(GLuint,String),
    fragment(GLuint,String),
    null,
}

impl sharder{

    pub fn new(stype:GLenum) ->sharder{
        match stype{
            gl::VERTEX_SHADER =>{sharder::vertex(0,String::new())},
            gl::FRAGMENT_SHADER =>{sharder::fragment(0,String::new())},
            _ =>{sharder::null },
        }
    }

    pub fn setGLuint(&mut self ,v:GLuint)->sharder{
        match *self{
            sharder::vertex(ref mut v0,_)   =>{*v0 = v;},
            sharder::fragment(ref mut v0,_) =>{*v0 = v;},
            sharder::null =>{}
        }
        return self.to_owned();
    }
    pub fn getGLuint(&self)->GLuint{
        match *self{
            sharder::vertex(v0,_)   =>{v0},
            sharder::fragment(v0,_) =>{v0},
            sharder::null =>{0}
        }
    }

    pub fn setString(&mut self ,s:String)->&mut sharder{
        match *self{
            sharder::vertex(_,ref mut v1)   =>{*v1 = s;},
            sharder::fragment(_,ref mut v1) =>{*v1 = s;},
            sharder::null =>{}
        }
        return self;
    }

    pub fn getString(&mut self)-> String{
        match *self{
            sharder::vertex(_,ref  v1)   =>{v1.to_owned()},
            sharder::fragment(_,ref v1) =>{v1.to_owned() },
            sharder::null =>{String::from("sharder Cdtring is null !!!")}
        }
    }

}


#[derive(Clone,Debug)]
pub enum uniformlocation{
    location(GLint,String),
    null
}


impl uniformlocation{
    pub fn new( location :GLint) ->uniformlocation{
        uniformlocation::location(location,String::new())
    }
    pub fn setGLuint(&mut self ,v:GLint)->&mut uniformlocation{
        match *self{
            uniformlocation::location(ref mut v0,_)   =>{*v0 = v;},

            uniformlocation::null =>{}
        }
        return self;
    }
    pub fn getGLuint(&self)->GLint{
        match *self{
            uniformlocation::location(v0,_)   =>{v0},
            uniformlocation::null =>{0}
        }
    }

    pub fn setString(&mut self ,s:String)-> uniformlocation{
        match *self{
            uniformlocation::location(_,ref mut v1)   =>{*v1= s;},
            uniformlocation::null =>{}
        }
        return self.to_owned();
    }

    pub fn getString(&mut self)-> String{
        match *self{
            uniformlocation::location(_,ref  v1)   =>{v1.to_owned()},
            uniformlocation::null =>{String::from("uniformlocation is null !!!!!!!!!!!!!")}
        }
    }

}

pub type matT = [GLfloat;16];

#[derive(Clone,Debug)]
pub enum matrix{
    mvpm(matT),
    projm(matT),
    viewm(matT),
    rotam(matT),
}

impl matrix{
    pub fn setMatrix(&mut self,m: *const GLfloat){
        match *self {

            matrix::mvpm(ref mut v)  =>{ unsafe {memcpy(v.as_ptr() as *mut  c_void ,m as *const  c_void , 4 * 16);} },
            matrix::projm(ref mut v) =>{ unsafe {memcpy(v.as_ptr() as *mut  c_void ,m as *const  c_void , 4 * 16);} },
            matrix::viewm(ref mut v) =>{ unsafe {memcpy(v.as_ptr() as *mut  c_void ,m as *const  c_void , 4 * 16);} },
            matrix::rotam(ref mut v) =>{ unsafe {memcpy(v.as_ptr() as *mut  c_void ,m as *const  c_void , 4 * 16);} },
        }
    }
    pub fn getMatrix(&self) ->& matT{
        match *self {

            matrix::mvpm(ref  v)  =>{ v },
            matrix::projm(ref  v) =>{ v },
            matrix::viewm(ref  v) =>{ v },
            matrix::rotam(ref  v) =>{ v },
        }
    }
}

pub type  Tprogram = &'static  RwLock<HashMap<GLuint,GLuint>>;
pub type  Tsharder = &'static  RwLock<HashMap<GLuint,sharder>>;
pub type  Tunlocation = &'static  RwLock<HashMap<GLint,uniformlocation>>;
lazy_static! {
 //pub    static      ref         DATA_BUFFERS_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 //pub    static      ref         DATA_TEXTURE_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_PROGRAM_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_SHADER_MAP : RwLock<HashMap<GLuint,sharder>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_UNFORM_LOCATION_MAP : RwLock<HashMap<GLint,uniformlocation>>  = RwLock::new(HashMap::new());
 pub    static      ref         STATIC_MATRIX : RwLock<matrix> = RwLock::new(matrix::mvpm([0.0;16]));
}

pub fn getStaticMatrix()->&'static  RwLock<matrix>{
    return &STATIC_MATRIX;
}

    pub fn WriteMatrix(mat: *const GLfloat){

        if let Ok(mut m) = getStaticMatrix().write() {
        //    println!("WriteMatrix {:?}",mat);
                m.setMatrix(mat);
        }

    }

    pub fn readMatrix() ->*const GLfloat{
    if let Ok(m) =  getStaticMatrix().read() {
         //   println!("readMatrix {:?}",m.getMatrix());
            m.getMatrix().as_ptr()
        }
        else{
           [0.0;16].as_ptr()
    }

    }


pub  fn getProgramMap()->Tprogram{
return &DATA_PROGRAM_MAP;
}


pub fn ProgramMapWrite(K:GLuint,V:GLuint){

    if let Ok(mut map) = getProgramMap().write() {
        map.insert(K, V);
    }
}


pub fn  ProgramMapRead(K: GLuint)->GLuint{

    if let Ok(map) =  getProgramMap().read() {
        match map.get(&K) {
            Some(value) => {
                *value
            }
            None => { 0}
        }

    }
        else
        {
            println!("mod renderobj ProgramMapRead err!");
            0
        }

}


pub fn getSharderMap()->Tsharder{
    return &DATA_SHADER_MAP;
}

pub fn SharderMapWrite(K:GLuint,V:sharder){

    if let Ok(mut map) = getSharderMap().write() {
        map.insert(K, V);
    }
}

pub fn  SharderMapRead(K: GLuint)-> sharder{

    if let Ok(map) =  getSharderMap().read() {
        match map.get(&K) {
            Some(value) => {
                value.to_owned()
            }
            None => { sharder::null}
        }

    }
        else
        {
            println!("mod renderobj SharderMapRead err!");
            sharder::null
        }

}


pub fn getunlocation()->Tunlocation{
    return &DATA_UNFORM_LOCATION_MAP;
}

pub fn uLocationMapWrite(K:GLint,V:uniformlocation){

    if let Ok(mut map) = getunlocation().write() {
        map.insert(K, V);
    }
}

pub fn  uLocationMapRead(K: GLint)-> uniformlocation{

    if let Ok(map) =  getunlocation().read() {
        match map.get(&K) {
            Some(value) => {
                value.to_owned()
            }
            None => { uniformlocation::null}
        }

    }
        else
        {
            println!("mod renderobj uLocationMapRead err!");
            uniformlocation::null
        }

}
//lazy_static!{

// static   mut   RENDER : RefCell<render>  = RefCell::new(render::new());
//}



#[derive(Clone,Debug)]
pub struct  render{
    instand: i32,
    program   : &'static RwLock<HashMap<GLuint,GLuint>>,
    sharders  : &'static RwLock<HashMap<GLuint,sharder>>,
    locations : &'static RwLock<HashMap<GLint,uniformlocation>>,
    pub  matrixs   :  &'static  RwLock<matrix>,
}

#[derive(Clone,Debug)]
pub struct  Rengine{

    program   : &'static RwLock<HashMap<GLuint,GLuint>>,
    sharders  : &'static RwLock<HashMap<GLuint,sharder>>,
    locations : &'static RwLock<HashMap<GLint,uniformlocation>>,
    pub  matrixs   :  &'static  RwLock<matrix>,
}


/*pub fn getRender()->  Arc<render>{
    // unsafe {&mut * obj.index.get()};
    //return unsafe{&mut  *RENDER.clone()}
  unsafe { return  RENDER;}
}*/
/*
pub fn getRenderinner() ->&'static render{
  return   &RENDER.into_inner().unwrap();
}
*/


/*
pub fn RenderWriteMatrix( m: *const GLfloat){

   if let Ok(mut r) = getRender().write() {
        r.WriteMatrix(m);
   }

}

pub fn RendergetMatrix() ->*const GLfloat{

    if let Ok(r) =  getRender().read() {
                r.getMatrix()
    }else{
        println!("RendergetMatrix error !!!!!!!! ");
        [0.0;16].as_ptr()
    }
}
*/




impl render{
    pub fn new() ->render{
        let   r : render =
        render{
            instand : 1,
            program :getProgramMap(),
            sharders:getSharderMap(),
            locations:getunlocation(),
            matrixs:getStaticMatrix(),
        };
       r
    }

    pub fn instance() -> &'static mut  Self {
        static mut instance: MLazy<render> = MLazy {
            lock:ONCE_INIT,
            ptr: 0 as *mut  render,
        };
        unsafe {
            instance.get(render::new)
        }
    }

/*    pub fn WriteMatrix(&mut self , m: *const GLfloat){

        if let Ok(mut render) = getMatrix().write() {

            self.matrixs.setMatrix(m);}

    }

    pub fn getMatrix(&self) ->*const GLfloat{
        self.matrixs.getMatrix().as_ptr()
    }*/

    pub fn draw(&mut self,location:GLint,api:Api){
        unsafe {
            match api.name.as_ref(){
                "glDrawArrays" =>glDrawArrays(api),
                "glUniformMatrix4fv" =>glUniformMatrix4fv(api),
                 "glVertexAttribPointer" => glVertexAttribPointer(api),
                  "glDisableVertexAttribArray" => glDisableVertexAttribArray(api),
                 "glEnableVertexAttribArray" => glEnableVertexAttribArray(api),

                _ => {}
            }
        }
    }

}
