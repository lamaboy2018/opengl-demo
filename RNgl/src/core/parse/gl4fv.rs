use glx::*;
use core::Type::Type;
use core::parse::glone::parseOne;
use core::parse::glfstar::parse4fstar;
// recv data[12] =["[Tid=7606]", "glUniform4fv", "GLuint", "0x2", "GLsizei", "0x1", "GLfloat *", "[16]", "0.000000", "0.000000", "0.000000", "0.000000"]

pub fn parse4fv(args: &mut [&str]) {
    // println!("parse4fv = {:?}" ,args);
    //  let v1 =  parseOne(args[0],args[1]);
    //  let v2 = parseOne(args[2],args[3]);
    // let v3 = parse4fstar(& mut args[4..]);
    // println!("v3 ={:?}",v3);

    // vec![v1,v2,v3]

}
