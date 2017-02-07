use std::sync::Arc;
#[derive(Debug,Clone)]
pub struct Queue<T> {
    qdata: Vec<T>,
}


impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { qdata: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.qdata.push(item);

        //  println!("recv push ={:?}",item);
    }
    // pub fn pop(&mut self) ->T{
    // self.qdata.remove(0);
    //
    // }
}
