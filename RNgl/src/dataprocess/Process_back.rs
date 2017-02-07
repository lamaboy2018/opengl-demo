use dataprocess::Queue::Queue;
//use std::collections::VecDeque as Queue;
//type Queue = VecDeque;
const NUM_QUEUE: usize = 32;
pub struct Process {

  //  Quequ :[[f32; 16]; 32],
    index : usize,
    capacity : usize,
 //   queueArry : [&'a mut  Queue<String>;NUM_QUEUE],
    queueArry :  Vec<Queue<String>>,
}

//let mut buf :[i32 ; len as i32] = [0 ; len];
//    let mut vbuf : Vec<u8> = vec![0 as u8 ;len ];

impl  Process {
   pub fn new(size : usize) ->Process{
       Process{
           index : 0,
           capacity : size ,
          // queueArry :  [& Queue::new(); NUM_QUEUE],
           queueArry: vec![Queue::new() as Queue<String>; size ],
       }


    }
    pub fn recv(&mut self, line : String)
    {

        if self.index > NUM_QUEUE {
                self.index =0;
            }
        if line.contains("eglSwapBuffers")
            {
                self.index += 1
            }
        println!("revc index {} ={}",self.index,line);
        self.queueArry[self.index].push(line);


    }

/*    pub fn getQueueArry()-> &Vec<Queue<String>>
    {
       & self.queueArry;
    }*/

}
