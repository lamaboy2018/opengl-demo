extern crate crossbeam;
use std::thread;

use std::sync::{Arc, Mutex, Condvar, RwLock};


use std::string::String;
use std::vec::*;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use std::io::BufRead;
use std::cell::{RefCell,UnsafeCell};

// use dataprocess::Queue::Queue;
use dataprocess::IRQueue::IRQueue;
use std::collections::VecDeque as Queue;

pub const NUM_QUEUE: usize = 32;

pub struct Qdata {
    pub data:UnsafeCell<Vec<Queue<String>>>,
    pub size: usize,
    pub index: UnsafeCell<IRQueue>,
    pub ch: (Sender<String>, Receiver<String>), // pub rw_lock: RwLock<usize>,
   // pub cver: Arc<Condvar>,
 //   pub rw_lock: Arc<RwLock<usize>>,
}

unsafe impl Sync for Qdata {}

impl Qdata {
    pub fn new(size: usize) -> (Producer, Consumer) {
        let v = vec![Queue::new(); size];
        let  i =  IRQueue::new(size);
        let mut data = Arc::new(Qdata {
            data: UnsafeCell::new(v),
            size: 0,
            index: UnsafeCell::new(i),
            ch: channel(), //    rw_lock: RwLock::new(size),
           // cver: Arc::new(Condvar::new()),
            //rw_lock: Arc::new(RwLock::new(count)),
        });
        let cver = Arc::new(Condvar::new());
        (Producer::new(data.clone(),cver.clone()), Consumer::new(data.clone(),cver.clone()))
    }
}

pub struct Producer {
    producer: Arc<Qdata>,

}

impl Producer {
    pub fn new(qdata: Arc<Qdata> ) -> Producer {
        Producer { producer: qdata}
    }

    pub fn start_producering(&mut self) {

        println!(" wait for connect ing ...");
        let listener = TcpListener::bind("192.168.4.164:60000").unwrap();

     //   let  a =  Arc::new(i);
        for client in listener.incoming() {
            println!(" wait for connect ing ...");
         //   let  mut QRindex = a.clone();
            match client {
                Ok(client) => {
                    let  condvar = &self.cver;
                    let  mut  obj = &mut self.producer;
                    let     QRindex = unsafe {&mut * obj.index.get()};
                    let     VQData =  unsafe {&mut * obj.data.get()};
                    crossbeam::scope(| scope| {
                        scope.spawn(move || {
                            //   println!("connected from: {} ", peer_addr);
                            let mut reader = BufReader::new(client);
                            //    let mut line = String::new();
                            let mut index: usize = 0;
                           // reader.read_line();
                            for (num, line) in reader.lines().enumerate()
                                {
                                let mut l: String = line.unwrap();
                                //    println!("[{}] {}", num, l);
                                //    let &(ref lock, ref cvar) = &*pair;
                                // thread::sleep_ms(2);
                                //    let mut status = lock.lock().unwrap();

                                if l.contains("eglSwapBuffers") {
                                    // status.index.enqueue(index);
                                    QRindex.enqueue(index);
                                    index += 1;
                                    if index >= NUM_QUEUE {
                                        index = 0;
                                    }
                                    condvar.notify_all();
                                }

                                    VQData[index].push_back(l);
                                // status.data[index].push(l);

                                // status.data[index].push_back(l);
                                // let _findex = index+1;


                            }


                        });
                    });



                }
                Err(e) => {
                    println!("Fucked by the network problem: {} ", e);
                }
            }
        }


    }
    // end
}
pub struct Consumer {
    consumer: Arc<Qdata>,
    cver: Arc<Condvar>,
}

impl Consumer {
    pub fn new(qdata: Arc<Qdata>,ver: Arc<Condvar>) -> Consumer {
        Consumer { consumer: qdata,cver :ver }
    }
    pub fn parssQdata(&mut self)
    {
        let  condvar = &self.cver;
        let  mut  obj = &mut self.consumer;
        let     QRindex = unsafe {&mut * obj.index.get()};
        let     VQData =  unsafe {&mut * obj.data.get()};

    loop{
        if !QRindex.run(){
            println!("start to wait >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

            //condvar.wait(_).unwrap();
        }

        let index  = QRindex.dequeue();


    }

    }
}
