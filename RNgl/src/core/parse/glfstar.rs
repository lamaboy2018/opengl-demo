use glx::*;
use core::Type::Type;
use std::f32;
use num::Float;
use num::Num;
use std::mem::size_of;

pub fn parse4fstar(args: &mut [&str]) -> Type {
    //  println!("parsefstar  ={:?}", args);
    // "GLfloat *", "[64]", "-2.519481", "0.572619", "-0.413208", "-0.393052", "0.611490", "1.903503", "0.233177", "0.221802", "-1.261723", "-0.220910", "0.938127", "0.892364", "0.000000", "0.000000", "1.102564", "3.000000"]
    // "GLfloat *", "[16]", "0.000000", "0.000000", "0.000000", "0.000000"]
    let mut vfloat: Vec<GLfloat> = Vec::new();
    match args[0] {
        "GLfloat *" | "GLfloat*" | "GLvoid *" => {
            let numlen = args[1].len() -1;
            let numfloat =  usize::from_str_radix(&args[1][1..numlen], 10).unwrap() / 4 /*size_of::<GLfloat>()*/ ;
            for i in 0..numfloat {
                let vf  = GLfloat::from_str_radix(&args[2 + i ], 10).unwrap();
                vfloat.push(vf)
            }

        //    println!(" numfloat ={} parse4fstar vfloat ={:?}",numfloat, vfloat);
            Type::GLfloatxfv(vfloat)

/*            {
                let v1 = GLfloat::from_str_radix(&args[2], 10).unwrap();
                let v2 = GLfloat::from_str_radix(&args[3], 10).unwrap();
                let v3 = GLfloat::from_str_radix(&args[4], 10).unwrap();
                let v4 = GLfloat::from_str_radix(&args[5], 10).unwrap();

                //  println!("GLfloat ={} {},{},{}  ", v1,v2,v3,v4);
                Type::GLfloatxfv((v1, v2, v3, v4))
            }*/
        }
        _ => Type::GLfloatxfv(vfloat),

    }


}
