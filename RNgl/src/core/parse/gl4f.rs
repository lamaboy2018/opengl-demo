use glx::*;
use core::Type::Type;
use core::parse::glone::parseOne;
// use core::parse::glfstar::parsefstar;

// match length = 12 parse ["[Tid=3562] ", " glUniform4f", "GLint", "0x5", "GLfloat", "0.000000", "GLfloat", "0.000000", "GLfloat", "0.000000", "GLfloat", "1.000000"]

pub fn parse4f(args: &mut [&str]) {

    println!("parse4fv = {:?}", args.len());
    // let v1 = parseOne(args[0], args[1]);
    //   let v2 = parseOne(args[2], args[3]);

    // let v3 = parse4fstar(&mut args[4..]);
    // println!("v3 ={:?}",v3);


}
