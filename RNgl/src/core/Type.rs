use glx::*;
use std::ffi::CString;
use std::os::raw::c_char;
// pub type GLenum = super::__gl_imports::raw::c_uint;
// pub type GLboolean = super::__gl_imports::raw::c_uchar;
// pub type GLbitfield = super::__gl_imports::raw::c_uint;
// pub type GLvoid = super::__gl_imports::raw::c_void;
// pub type GLbyte = super::__gl_imports::raw::c_char;
// pub type GLshort = super::__gl_imports::raw::c_short;
// pub type GLint = super::__gl_imports::raw::c_int;
// pub type GLclampx = super::__gl_imports::raw::c_int;
// pub type GLubyte = super::__gl_imports::raw::c_uchar;
// pub type GLushort = super::__gl_imports::raw::c_ushort;
// pub type GLuint = super::__gl_imports::raw::c_uint;
// pub type GLsizei = super::__gl_imports::raw::c_int;
// pub type GLfloat = super::__gl_imports::raw::c_float;
// pub type GLclampf = super::__gl_imports::raw::c_float;
// pub type GLdouble = super::__gl_imports::raw::c_double;
// pub type GLclampd = super::__gl_imports::raw::c_double;
// pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
// pub type GLchar = super::__gl_imports::raw::c_char;
// pub type GLcharARB = super::__gl_imports::raw::c_char;
//
// #[cfg(target_os = "macos")] pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
// #[cfg(not(target_os = "macos"))] pub type GLhandleARB = super::__gl_imports::raw::c_uint;
//
// pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
// pub type GLhalf = super::__gl_imports::raw::c_ushort;
//
// Must be 32 bits
// pub type GLfixed = GLint;
//
// pub type GLintptr = isize;
// pub type GLsizeiptr = isize;
// pub type GLint64 = i64;
// pub type GLuint64 = u64;
// pub type GLintptrARB = isize;
// pub type GLsizeiptrARB = isize;
// pub type GLint64EXT = i64;
// pub type GLuint64EXT = u64;


// macro_rules! gltype {
// ($($(#[$meta:meta])*  $name:ident($ty:ty),)*) => {
// #[derive(Debug, Clone)]
// pub enum Type<T> {
// $($(#[$meta])* $name(T),)*
// }
// impl<T> Type<T> {
// pub fn unwrap(&self) ->&T{
// match *self {
// $(Type::$name(ref x) =>{x},)*
// }
// }
// }
// }
// }
//
// #[macro_export]
// gltype!{
// pub enum Type {
// GLnull(CString),
// GLenum(GLenum),
// GLboolean(GLboolean),
// GLint(GLint),
// GLbyte(GLbyte),
// GLshort(GLshort),
// GLclampx(GLclampx),
// GLubyte(GLubyte),
// GLushort(GLushort),
// GLuint(GLuint),
// GLsizei(GLsizei),
// GLclampf(GLclampf),
// GLdouble(GLdouble),
// GLclampd(GLclampd),
// GLfloat_4fv((GLfloat, GLfloat, GLfloat, GLfloat)),
// GLfloat(GLfloat),
// GLintptr(GLintptr),
// GLsizeiptr(GLsizeiptr),
// GLbitfield(GLbitfield),
// GLchar_ptr(CString),
//
// }
// #[derive(Clone,Debug)]
// pub enum Type<T> {
// GLnull(T),
// GLenum(T),
// GLboolean(T),
// GLint(T),
// GLbyte(T),
// GLshort(T),
// GLclampx(T),
// GLubyte(T),
// GLushort(T),
// GLuint(T),
// GLsizei(T),
// GLclampf(T),
// GLdouble(T),
// GLclampd(T),
// GLfloat_4fv(T),
// GLfloat(T),
// GLintptr(T),
// GLsizeiptr(T),
// GLbitfield(T),
// GLchar_ptr(T),
// }
//
// impl<T> Type<T>{
// pub fn  unwarp(&self) ->&T{
// match *self{
// Type::GLnull( ref x) =>{x}
// Type::GLenum(ref x) =>{x}
// Type::GLboolean(ref x) =>{x}
// Type::GLint(ref x) =>{x}
// Type::GLbyte(ref x) =>{x}
// Type::GLshort(ref x) =>{x}
// Type::GLclampx(ref x) =>{x}
// Type::GLubyte(ref x) =>{x}
// Type::GLushort(ref x) =>{x}
// Type::GLfloat(ref x) =>{x}
// Type::GLintptr(ref x) =>{x}
// Type::GLsizeiptr(ref x) =>{x}
// Type::GLbitfield(ref x) =>{x}
// Type::GLchar_ptr(ref x) =>{x}
// }
// }
// }
// static  NUll : &'static CString =  CString::new("null").unwrap().as_ref();

#[derive(Clone,Debug)]
pub enum Type {
    GLnull(CString),
    GLenum(GLenum),
    GLboolean(GLboolean),
    GLint(GLint),
    GLbyte(GLbyte),
    GLshort(GLshort),
    GLclampx(GLclampx),
    GLubyte(GLubyte),
    GLushort(GLushort),
    GLuint(GLuint),
    GLsizei(GLsizei),
    GLclampf(GLclampf),
    GLdouble(GLdouble),
    GLclampd(GLclampd),
    GLfloatxfv(Vec<GLfloat>),
    GLfloat(GLfloat),
    GLintptr(GLintptr),
    GLsizeiptr(GLsizeiptr),
    GLbitfield(GLbitfield),
    GLchar_ptr(CString),
}


impl Type {
    //    pub fn  unwarp(&self) ->fn(Type){
    // match *self{
    // Type::GLnull(_) =>{self.getGLnull}
    // Type::GLenum(_) =>{self.getGLenum}
    //
    // Type::GLboolean(ref x) =>{self.getGLboolean}
    // Type::GLint(ref x) =>{self.getGLboolean}
    // Type::GLbyte(ref x) =>{self.getGLboolean}
    // Type::GLshort(ref x) =>{self.getGLboolean}
    // Type::GLclampx(ref x) =>{self.getGLboolean}
    // Type::GLubyte(ref x) =>{self.getGLboolean}
    // Type::GLushort(ref x) =>{self.getGLboolean}
    // Type::GLfloat(ref x) =>{self.getGLboolean}
    // Type::GLintptr(ref x) =>{self.getGLboolean}
    // Type::GLsizeiptr(ref x) =>{self.getGLboolean}
    // Type::GLbitfield(ref x) =>{self.getGLboolean}
    // Type::GLchar_ptr(ref x) =>{self.getGLboolean}
    // }
    // }
/*    pub fn get<T>(&self)->T{
        match *self {
            Type::GLint(x) => x as T,
            _ => 0  as T,
        }
    }*/

    pub fn getGLint(&self) -> GLint {
        match *self {
            Type::GLint(x) => x,
            _ => 0,
        }

    }
    pub fn getGLnull(&self) -> *const c_char {
        match *self {
            Type::GLnull(ref x) => x.as_ptr(),
            _ => CString::new("null").unwrap().as_ptr(),
        }

    }

    pub fn getGLenum(&self) -> GLenum {
        match *self {
            Type::GLenum(x) => x,
            _ => 0,
        }

    }

    pub fn getGLboolean(&self) -> GLboolean {
        match *self {
            Type::GLboolean(x) => x,
            _ => 0,
        }

    }

    pub fn getGLbyte(&self) -> GLbyte {
        match *self {
            Type::GLbyte(x) => x,
            _ => 0,
        }

    }

    pub fn getGLushort(&self) -> GLushort {
        match *self {
            Type::GLushort(x) => x,
            _ => 0,
        }

    }

    pub fn getGLuint(&self) -> GLuint {
        match *self {
            Type::GLuint(x) => x,
            _ => 0,
        }

    }

    pub fn getGLsizei(&self) -> GLsizei {
        match *self {
            Type::GLsizei(x) => x,
            _ => 0,
        }

    }

    pub fn getGLclampf(&self) -> GLclampf {
        match *self {
            Type::GLclampf(x) => x,
            _ => 0.0,
        }

    }

    pub fn getGLdouble(&self) -> GLdouble {
        match *self {
            Type::GLdouble(x) => x,
            _ => 0.0,
        }
    }

    pub fn getGLclampd(&self) -> GLclampd {
        match *self {
            Type::GLclampd(x) => x,
            _ => 0.0,
        }
    }
    pub fn getGLfloatxfv(&self) -> *const GLfloat {
        match *self {
            Type::GLfloatxfv(ref x) => x.as_ptr(),
            _ => [0.0, 0.0, 0.0, 0.0].as_ptr(),
        }
    }
    pub fn getGLfloat(&self) -> GLfloat {
        match *self {
            Type::GLfloat(x) => x,
            _ => 0.0,
        }
    }

    pub fn getGLintptr(&self) -> GLintptr {
        match *self {
            Type::GLintptr(x) => x,
            _ => 0,
        }
    }


    pub fn getGLbitfield(&self) -> GLbitfield {
        match *self {
            Type::GLbitfield(x) => x,
            _ => 0,
        }
    }


    pub fn getGLcharptr(&self) -> *const c_char {
        match *self {
            Type::GLchar_ptr(ref x) => {
                //    println!("getGLcharptr {:?}",x);
                x.as_ptr()
            }
            _ => CString::new("null").unwrap().as_ptr(),
        }
    }

    pub fn getString(&self)->String{
        match *self {
            Type::GLchar_ptr(ref x) => {

                x.to_owned().into_string().unwrap()
            }
            _ =>String::new(),
        }
    }
}

/*
pub unsafe fn string_from_c_str(c_str: *const c_char) -> String {
    String::from_utf8_lossy(CStr::from_ptr(c_str).to_bytes()).into_owned()
}
*/

// #[macro_export]
// macro_rules! getGLsizei{
// ($e:expr) => {
// match $e{
// Type::GLsizei(x) =>{ x}
// _ => {0}
// }
// }
// }
//
//
// macro_rules! getGLenum{
// ($e:expr) => {
// match $e{
// Type::GLenum(x) =>{ x}
// _ => {0}}
// }
// }
//
// macro_rules! getGLboolean{
// ($e:expr) => {
// match $e{
// Type::GLboolean(x) =>{ x}
// _ => {0}
// }
// }
// }
//
//
// macro_rules! getGLbitfield{
// ($e:expr) => {
// match $e{
// Type::GLbitfield(x) =>{x}
// _ => {0}}
// }
// }
//
//
// macro_rules! getGLint{
// ($e:expr) => {
// match $e{
// Type::GLint(x) =>{ x}
// _ => {0}}
// }
// }
//
// macro_rules! getGLuint{
// ($e:expr) => {
// match $e{
// Type::GLuint(x) =>{x}
// _ => {0}}
// }
// }
//
//
//
// macro_rules! getGLcharptr {
// ($e:expr) => {
// match $e{
// Type::GLchar_ptr(ref mut x) =>{x }
// _ => {
// CString::new("").unwrap()
//  let cptr : * const GLchar = c_str.as_ptr();
// &cptr
//
// }
//
// }
// }
// }
//
//
//
// macro_rules! getGLnull{
// ($e:expr) => {
// match $e{
// Type::GLnull =>{0 as *const GLint; }
// _ => {0 as *const GLint ;}
// }
//
// }
// }
//
//
//
//
//
// macro_rules! getGLfloat_4fv{
// ($e:expr) => {
// match $e{
// Type::GLfloat_4fv(v) =>{v}
// _ => {(0.0,0.0,0.0,0.0)}
// }
//
// }
// }
//
//
//


// let get = |c :Type| {match  c{
// Type::GLsizei(x) =>{ x}
// _ => {}
// }}
//


// pub fn get<V>(value :& Type){
//
// match value{
// Type::GLsizei(x) =>{x}
// _ => {V}
// }
// }

// impl Type{
//
// |x: i32| x + 1;
// }


// impl fmt::Display for Type {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// write!(f, "Api tid = {},name = {}", self.tid, self.name)
// }
// }
