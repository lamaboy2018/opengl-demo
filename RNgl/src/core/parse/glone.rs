use glx::*;
use core::Type::Type;

use num::Float;
use num::Num;
use std::ffi::CString;
use core::parse::glfstar::parse4fstar;
use core::parse::glcharptr::parseCharptr;
// pub fn parseOne(typestr: &str, value: &str) -> Type {

pub fn parseOne(args: &mut [&str]) -> Type {
    let typestr = args[0];
    let mut value = args[1];
    match typestr {
        "GLenum" => {
            // let (_,value ) = value.split_at(2);
            // let v : u32  = value.trim().parse().expect("Please type a number!");
            //  let value = parseid(value);
            // println!(" parse1enum {} {:?}", typestr, value);
            let value = u32::from_str_radix(&value[2..], 16).unwrap();
            // println!(" parse1enum {} {:?}", typestr, value);
            Type::GLenum(value)
        }
        "GLint" | "GLint return" => {
            let value = GLint::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLint(value)
        }
        "GLuint" | "GLuin" => {
            let value = GLuint::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLuint(value)
        }

        "GLsizei" => {
            let value = GLsizei::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLsizei(value)
        }
        "GLintptr" => {
            let value = GLintptr::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLintptr(value)
        }

        "GLsizeiptr" => {
            let value = GLsizeiptr::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLsizeiptr(value)
        }

        "GLbitfield" => {
            let value = GLbitfield::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLbitfield(value)
        }
        "p=" | "GLuint*" | "GLuint *" => {
            let value = GLuint::from_str_radix(&value[2..], 16).unwrap();
            println!(" GLuint* {} {:?}", typestr, value);
            Type::GLuint(value)
        }

        "GLboolean" => {
            let value = GLboolean::from_str_radix(&value[2..], 16).unwrap();
            //   println!(" parse1enum {} {:?}", typestr, value);
            Type::GLboolean(value)
        }
        "GLfloat *" | "GLfloat*" | "GLvoid *" => {
            parse4fstar(args)
            //  Type::GLfloat_4fv(v)
        }
        "GLfloat" => {
            let v1 = GLfloat::from_str_radix(&value[..], 10).unwrap();
            Type::GLfloat(v1)
        }
        "GLchar *" | "GLchar*" => unsafe { parseCharptr(args) },
        "GLuint return" => {
            let v1 = GLuint::from_str_radix(&value[2..], 16).unwrap();

            Type::GLuint(v1)
        }
        "GLclampf" => {
            let v = GLclampf::from_str_radix(&value[..], 10).unwrap();
            Type::GLclampf(v)
        }

        _ => Type::GLnull(CString::new("null").unwrap()),
    }
}
