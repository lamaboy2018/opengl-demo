use glx::*;
use std::fmt;

use std::u32;
use std::i64;
use std::ffi::CString;

use core::Type::Type;
use core::parse::glone::parseOne;
use core::parse::gl4fv::parse4fv;
use core::parse::gl4f::parse4f;
use debug::checkimpl::Instant;

#[derive(Debug)]
pub struct Api {
    pub tid: u32,
    pub name: String,
    pub args: Vec<Type>, /*  pub func: FnMut,
                          *   pub ret: Type, // pub callglimpl: fn(Vec<Type>) -> (), */
}

// impl fmt::Display for Api {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// write!(f, "Api tid = {},name = {}", self.tid, self.name)
// }
// }

impl Api {
    pub fn new(args: &mut Vec<&str>) -> Api {
        parsestep(args)
        // parse(args)
    }
}


//    Api {tid:}
//
// recv data[12] =["[Tid=7606]", "glUniform4fv", "GLuin", "0x2", "GLsizei", "0x1", "GLfloat *", "[16]", "0.000000", "0.000000", "0.000000", "0.000000"]
// recv data[8] =["[Tid=7606]", "glDrawArrays", "GLenum", "0x4", "GLsizei", "0x3", "", "0x0"]
// recv data[12] =["[Tid=7606]", "glUniform4fv", "GLuin", "0x2", "GLsizei", "0x1", "GLfloat *", "[16]", "0.000000", "0.000000", "0.000000", "0.000000"]
// recv data[8] =["[Tid=7606]", "glDrawArrays", "GLenum", "0x4", "GLsizei", "0x3", "", "0x0"]
// recv data[4] =["[Tid=7606]", "glClear", "GLint", "0x4100"]
// recv data[4] =["[Tid=7606]", "glEnable", "GLenum", "0xB71"]
// recv data[2] =["[Tid=7606]", "glUseProgram()"]
#[warn(lint)]
fn parsestep(args: &mut Vec<&str>) -> Api {
    let len = args.len();
    // println!(" match length = {} parse {:?}", len, args);

    match len {
        2 => {
            Api {
                tid: parseid(&mut args[0]),
                name: parsename(args[1]),
                args: vec![], //       ret: Type::GLnull(CString::new("null").unwrap()),

            }
        }
        4...10240000 => {
            let mut num = len - 1;
            let mut i = 2;
            //  println!("init   i = {}, num ={}  ", i, num);
            let mut argv: Vec<Type> = vec![];
            //    let mut retv = Type::GLnull(CString::new("null").unwrap());
            while i < num {
                if args[i].contains("GL") {
                   let v1 = parseOne(&mut args[i..]);
                    argv.push(v1);
                }
                i += 1;
                // num -= 2;
            }
            //   println!("parsestep   argv ={:?}", argv);
            Api {
                tid: parseid(&mut args[0]),
                name: parsename(args[1]),
                args: argv, //  ret: retv,
            }
        }
        _ => {
            Api {
                tid: 0,
                name: String::new(),
                args: vec![], //   ret: Type::GLnull(CString::new("null").unwrap()),
            }
        }
    }
}


fn parseid(s: &str) -> u32 {
   // let index = s.find("Tid=").unwrap();
    //    self.tid =
    let (_, id) = s.split_at(5);
    //    let mut l = id.len();
  //  let tid = id[index..id.len() -1];

 let tid=   u32::from_str_radix(&id[..id.len() -1], 10).unwrap();

//println!("parseid = {}",tid);
return tid;
}

fn parsename(s: &str) -> String {
    s.to_string()
}
