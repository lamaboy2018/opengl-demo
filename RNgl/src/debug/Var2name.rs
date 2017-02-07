use std::fs::File;
use std::path::Path;
use std::error::Error;

use std::io::BufWriter;
use std::io::prelude::*;
use std::io::BufReader;
use std::char;
use std::collections::HashMap;

use glx::DFile;

// lazy_static! { //glGetUniformLocation
// pub  static  mut    DATA_BUFFERS_MAP :  HashMap<String,String>  =  HashMap::new();
//



pub struct VarName {
    pub map: HashMap<u32, String>, /*   var: u32,
                                    *  name: String, */
}


impl VarName {
    pub fn new() -> VarName {
        let mut mp: HashMap<u32, String> = HashMap::new();
        VarName {
            map: mp, /*    var: 0,
                      *    name: String::new(), */
        }
    }
    // src/gl/mod.rs

    pub fn Instant(&mut self) {
        // VarName::new();
      //  let fp = Path::new("src/glx/mod.rs");
     //   let display = fp.display();

        let mut file = File::open(DFile).unwrap();
        // {
        // Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        // Ok(file) => file,
        // };

        let mut fin = BufReader::new(file);
        //    let mut hm: HashMap<String, String> = HashMap::new();
        for (num, line) in fin.lines().enumerate() {
            let mut l: String = line.unwrap();
            if l.contains("pub const") {
                let index = l.find(char::is_uppercase).unwrap();
                let (_, varname) = l.split_at(index);

                let index = varname.find(':').unwrap();
                let (name, var) = varname.split_at(index);
                let index = var.find("= ").unwrap();
                let (_, var) = var.split_at(index + 2);
                let index = var.find(';').unwrap();

                let (var, _) = var.split_at(index);

                if var.contains("0x") && var.len() == 6 {
                    // println!("var  ={:?}  ",&var[2..]);
                    let value = u32::from_str_radix(&var[2..], 16).unwrap();
                    //    println!("var  ={:?} value ={} ",&var[2..],value);
                    self.map.insert(value, name.to_string());
                }



                // let mut str : Vec<&str> = l.split(':').collect();
                //  println!("[{}]  {} line ={}", name ,var,l);
            }


        }

    }

    // :glBufferData:GLenum:0x88EC:GLsizeiptr:0x120000:GLvoid *:[0]:GLenum:0x88E8
    pub fn println(&self, string: &Vec<&str>) {
        let len = string.len();
        let mut start = String::new();
        if len > 2 {
            for i in 0..len {
                match string[i] {
                    "GLenum" => {

                        start += string[i];
                        start += ":";
                        let value = u32::from_str_radix(&string[i + 1][2..], 16).unwrap();
                        start += self.map.get(&value).unwrap();
                    }
                    _ => {
                        start += ":";
                        start += string[i];
                        start += ":";

                    }
                }
            }
            println!("Toolprintln  org ={:?}", start);
        }

    }


    pub fn callfn() {

        //  glViewpoer(0,15,6,78,56,78);
    }
}
