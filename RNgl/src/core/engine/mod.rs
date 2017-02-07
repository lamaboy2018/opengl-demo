mod draw;
pub mod rengine;
use glx::*;

pub use core::engine::rengine::*;
use core::api::Api;
use utils::delay::mdelay;
//static  e : &'static Rengine  = get();


pub fn get()->&'static mut Rengine{
    Rengine::instance()
}

pub fn programWriteMap(K:GLuint,V:GLuint)
{
    get().ProgramMapWrite(K,V);
}

pub fn programReadMap(K: GLuint)->GLuint{
    get().ProgramMapRead(K)
}

pub fn SharderReadMap(K: GLuint)->  sharder{
    get().SharderMapRead(K)
}


pub fn SharderWriteMap(K:GLuint,V:sharder){
    get().SharderMapWrite(K,V)
}

pub fn localWriteMap(K:GLint,V:uniformlocation){
    get().uLocationMapWrite(K,V)
}



pub fn localReadMap(K:GLint)->uniformlocation{
get().uLocationMapRead(K)
}

pub fn undateMatrix(mat: *const GLfloat)->*const GLfloat{
    get().undateMatrix(mat)
}

pub fn startDraw(api:Api){
get().draw(api);
    mdelay(1);
}











