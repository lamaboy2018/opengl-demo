use core::Type::Type;

use core::api::Api;
use glx::*;
use std::collections::HashMap;
use core::datamap::getTexMap;
// let  ref  hm : HashMap<GLuint,GLuint>  =  getMap();
pub unsafe fn glGenTextures(api: Api) {

    let args = api.args;
    let mut textures: Vec<GLuint> = vec![0;args[0].getGLsizei() as usize ];


    gl::GenTextures(args[0].getGLsizei(), textures.as_mut_ptr());

    if let Ok(mut map) = getTexMap().write() {
        println!("map glGenTextures  insert<{},{}> ",
                 args[1].getGLuint(),
                 textures[0]);
        map.insert(args[1].getGLuint(), textures[0]);

    }
}

pub unsafe fn glBindTexture(api: Api) {
    let args = api.args;
    let remotTexture = args[1].getGLuint();
    if let Ok(map) = getTexMap().read() {

        match map.get(&remotTexture) {
            Some(texture) => {
                println!("map glBindTexture  get <{},{:?}> ", remotTexture, *texture);
                gl::BindTexture(args[0].getGLenum(), *texture);
            }
            None => {
                if remotTexture == 0 {
                    gl::BindTexture(args[0].getGLenum(), remotTexture);
                }
                println!("glBindTexture  get <{},{:?}>  error ",
                         remotTexture,
                         map.get(&remotTexture));
            }
        }

    }

}

pub unsafe fn glTexParameteri(api: Api) {
    // TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint)
    let args = api.args;
    gl::TexParameteri(args[0].getGLenum(), args[1].getGLenum(), args[2].getGLint());
}
