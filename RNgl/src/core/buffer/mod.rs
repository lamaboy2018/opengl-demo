pub mod vbo;



use core::Type::Type;
use glx::*;
use core::api::Api;

use core::datamap::getBufMap;


pub unsafe fn glUnmapBuffer(api: Api) {
    gl::UnmapBuffer(api.args[0].getGLenum());
}

pub unsafe fn glBufferSubData(api: Api) {
    // gl::BufferSubData
}


pub unsafe fn glGenBuffers(api: Api) {
    let size = api.args[0].getGLsizei();
    let mut bufffers: Vec<GLuint> = vec![0;size as usize];
    let rembuffers = api.args[1].getGLuint();
    gl::GenBuffers(size, bufffers.as_mut_ptr());
    if let Ok(mut map) = getBufMap().write() {
        println!("map glGenBuffers   insert<{},{}> ", rembuffers, bufffers[0]);
        map.insert(rembuffers, bufffers[0]);
    }
    // println!("c ={} buffer[0] {} ",c,buffers[0]);
}

pub unsafe fn glBindBuffer(api: Api) {
    // BindBuffer(target: types::GLenum, buffer: types::GLuint)

    let args = api.args;
    let remotbuffers = args[1].getGLuint();
    if let Ok(map) = getBufMap().read() {

        match map.get(&remotbuffers) {
            Some(buffer) => {
                println!("map glBindBuffer  get <{},{:?}> ", remotbuffers, *buffer);
                gl::BindTexture(args[0].getGLenum(), *buffer);
            }
            None => {
                if remotbuffers == 0 {
                    gl::BindTexture(args[0].getGLenum(), remotbuffers);
                } else {
                    println!("glBindBuffer  get <{},{:?}>  error ",
                             remotbuffers,
                             map.get(&remotbuffers));
                }
            }
        }

    }

}


pub unsafe fn glBufferData(api: Api) {
    // gl::BufferData
}
