use std::mem;
use std::sync;

pub struct CLazy<T> {
    pub lock: sync::Once,
    pub ptr: *const T,
}

impl<T> CLazy<T> {
    pub fn get<F>(&'static mut self, init: F) -> &'static T
        where F : FnOnce() -> T
    {
        // ~ decouple the lifetimes of 'self' and 'self.lock' such we
        // can initialize self.ptr in the call_once closure (note: we
        // do have to initialize self.ptr in the closure to guarantee
        // the ptr is valid for all calling threads at any point in
        // time)
        let lock: &sync::Once = unsafe { mem::transmute(&self.lock) };
        lock.call_once(|| {
            unsafe {
                self.ptr = mem::transmute(Box::new(init()));
            }
        });
        unsafe { &*self.ptr }
    }
}

pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;

