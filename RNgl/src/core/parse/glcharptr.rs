
use glx::*;
use core::Type::Type;
use core::parse::glfstar::parse4fstar;
use num::Float;
use num::Num;
use std::ffi::CString;

pub unsafe fn parseCharptr(args: &mut [&str]) -> Type {

        let s = args[2];
        if !s.contains("#version") && s.contains("{"){
            let mut str = String::from(vglsl);
            str += s;
            let cstr=   CString::new(str.as_str()).unwrap();
           return Type::GLchar_ptr(cstr);
        }

    let cstr = CString::new(args[2]).unwrap();

  //  println!("parseCharptr ={:?} ", cstr);
    Type::GLchar_ptr(cstr)

}
