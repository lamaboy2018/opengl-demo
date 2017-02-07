use glx::*;


use std::collections::HashMap;
use std::sync::RwLock;
//use std::rc::Rc;

lazy_static! { //glGetUniformLocation
 pub    static      ref         DATA_BUFFERS_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_TEXTURE_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_SHADER_MAP : RwLock<HashMap<GLuint,GLuint>>  = RwLock::new(HashMap::new());
 pub    static      ref         DATA_UNFORM_LOCATION_MAP : RwLock<HashMap<GLint,GLint>>  = RwLock::new(HashMap::new());
 pub    static      ref         SHADER_SOUREC_MAP : RwLock<HashMap<GLuint,String>>  = RwLock::new(HashMap::new());
}

pub fn getTexMap() -> &'static RwLock<HashMap<GLuint, GLuint>> {
    return &DATA_TEXTURE_MAP;
}
pub fn getBufMap() -> &'static RwLock<HashMap<GLuint, GLuint>> {
    return &DATA_BUFFERS_MAP;
}

pub fn getShaderMap() -> &'static RwLock<HashMap<GLuint, GLuint>> {
    return &DATA_SHADER_MAP;
}

pub fn getUniformMap() -> &'static RwLock<HashMap<GLint, GLint>> {
    return &DATA_UNFORM_LOCATION_MAP;
}

pub fn getShaderSMap()->&'static RwLock<HashMap<GLuint, String>>{
    return &SHADER_SOUREC_MAP;
}

pub fn GLintMapWrite(Map: &'static RwLock<HashMap<GLint, GLint>>,K:GLint,V:GLint){

    if let Ok(mut map) = Map.write() {
        map.insert(K, V);
    }
}


pub fn GLintMapRead(Map: &'static RwLock<HashMap<GLint, GLint>>,select: GLint)->GLint{

    if let Ok(map) = Map.read() {
        match map.get(&select) {
            Some(value) => {
                *value
            }
            None => { 0}
        }

    }
        else
        {
            0
        }

}

pub fn GLuintMapWrite(Map: &'static RwLock<HashMap<GLuint, GLuint>>,K: GLuint,V :GLuint){
    if let Ok(mut map) = Map.write() {
       // println!("map glCreateShader   insert<{},{}> ", remoteSharder, shader);
        map.insert(K, V);
    }
}

pub fn GLuintMapRead(Map: &'static RwLock<HashMap<GLuint, GLuint>>,select: GLuint)->GLuint{

    if let Ok(map) = Map.read() {
        match map.get(&select) {
            Some(value) => {
                *value
            }
            None => { 0}
        }

    }
        else
            {
                0
            }

}


/*pub fn mapRead<U , M>  (Map: M , select: U ) ->U  where U : GLuint + GLint ,M : &'static RwLock<HashMap<U, U>> {

    if let Ok(map) = Map.read() {
        match map.get(&select) {
            Some(value) => {

                *value
            }
            None => { String::new() }
        }

    }else
    {
       0 as U
    }
}*/


pub fn StringMapWrite(shader:GLuint,s:String){
    if let Ok(mut map) = getShaderSMap().write() {
        // println!("map glCreateShader   insert<{},{}> ", remoteSharder, shader);
        map.insert(shader, s);
    }
}

pub fn StringmapRead(select:GLuint )->String {

    if let Ok(map) = getShaderSMap().read() {
        match map.get(&select) {
            Some(source) => {
                // println!(" glCompileShader  shader ={}  \n source= {}",*shader,*source);
                source.to_owned()
            }
            None => { String::new() }
        }

    }else
    {
        String::new()
    }
}

//


