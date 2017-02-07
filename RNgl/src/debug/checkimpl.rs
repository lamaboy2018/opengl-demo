use std::fs::File;
use std::path::Path;
use std::error::Error;

use std::io::BufWriter;
use std::io::prelude::*;
use std::io::BufReader;
use std::char;
use std::collections::HashMap;
use glx::DFile;

pub fn Instant() -> HashMap<String, String> {


    let mut file = File::open(DFile).unwrap();
    // {
    // Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    // Ok(file) => file,
    // };

    let mut fin = BufReader::new(file);
    let mut vecapi: HashMap<String, String> = HashMap::new();
    let mut glhear = String::from("gl");
    for (num, line) in fin.lines().enumerate() {
        let mut l: String = line.unwrap();
        if l.contains("pub unsafe fn") {

            let index = l.find("fn ").unwrap();
            let (_, name) = l.split_at(index + 3);

            let index = name.find(')').unwrap();
            let (name, _) = name.split_at(index + 1);

            //  glhear +=name;
            // vecapi.push("gl".to_string()+name);

            vecapi.insert("gl".to_string() + name, name.to_string());
            // glhear.clear();
            //
            // {
            // let  s = "gl".to_string()+name;
            // println!("fn ={:?} ,len ={}",vecapi.get(&s).unwrap(),vecapi.len());
            // }
            //


        }


    }
    return vecapi;

}
