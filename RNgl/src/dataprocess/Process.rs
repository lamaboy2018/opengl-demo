extern crate crossbeam;
use std::thread;

use std::sync::{Arc, Mutex, Condvar, RwLock};


use std::string::String;
use std::vec::*;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::collections::HashMap;
use std::io::BufRead;


// use dataprocess::Queue::Queue;
use debug::filprintln::filprintln;
use debug::checkimpl;
use debug::Var2name::VarName;
use dataprocess::IRQueue::IRQueue;
use std::collections::{VecDeque as Queue, LinkedList};
use std::cell::{RefCell, UnsafeCell};

use core::api::*;
pub const NUM_QUEUE: usize = 32;

static count: usize = 0;

lazy_static! {

    static  ref  RCT  : Arc<RwLock<usize>> =  Arc::new(RwLock::new(count));
}

pub struct Qdata {
    pub data: Vec<Queue<String>>,
    pub size: usize,
    pub index: IRQueue,
    pub ch: (Sender<String>, Receiver<String>), // pub rw_lock: RwLock<usize>,
    pub rw_lock: Arc<RwLock<usize>>,
}

impl Qdata {
    pub fn new(size: usize) -> Qdata {
        let v = vec![Queue::new(); size];
        Qdata {
            data: v,
            size: 0,
            index: IRQueue::new(size),
            ch: channel(), //    rw_lock: RwLock::new(size),
            rw_lock: Arc::new(RwLock::new(count)),
        }

    }
}




pub struct Producer {
    cvar: Arc<(Mutex<Qdata>, Condvar)>,
}

lazy_static! {
 pub   static  ref  SP : Producer  = Producer::new(Arc::new((Mutex::new(Qdata::new(NUM_QUEUE)), Condvar::new())));
}

impl Producer {
    pub fn new(cvar: Arc<(Mutex<Qdata>, Condvar)>) -> Producer {
        Producer { cvar: cvar }
    }

    pub fn get_cvar(&self) -> &Arc<(Mutex<Qdata>, Condvar)> {
        &self.cvar
    }
    pub fn start_net(&'static self) {

        // let pair = self.cvar.clone();
        let id = thread::Builder::new().name("glserver".to_string()).spawn(move || {
            println!(" wait for connect ing ...");
            let listener = TcpListener::bind("192.168.1.60:60000").unwrap();

            for client in listener.incoming() {
                println!(" new  connect ing ...");
                match client {
                    Ok(client) => {
                        let id = thread::Builder::new()
                            .name(client.peer_addr().unwrap().to_string())
                            .spawn(move || {
                                let pair = self.cvar.clone();
                                let peer_addr = client.peer_addr().unwrap();

                                println!("connected from: {} ", peer_addr);
                                let mut reader = BufReader::new(client);
                                //    let mut line = String::new();
                                let mut index: usize = 0;

                                for (num, line) in reader.lines().enumerate() {
                                    let mut l: String = line.unwrap();
                                    //  println!("[{}] {}", num, l);
                                    let &(ref lock, ref cvar) = &*pair;
                                    // thread::sleep_ms(2);
                                    let mut status = lock.lock().unwrap();



                                    if l.contains("eglSwapBuffers") {
                                        status.index.enqueue(index);
                                        index += 1;
                                        if index >= NUM_QUEUE {
                                            index = 0;
                                        }


                                        status.size += 1;
                                        cvar.notify_all();
                                    }

                                    // status.data[index].push(l);
                                    if num > 2 {
                                        status.data[index].push_back(l);
                                    }

                                    // let _findex = index+1;


                                }


                            });
                        println!(" thread id  ={:?}", id.unwrap().thread())

                    }
                    Err(e) => {
                        println!("Fucked by the network problem: {} ", e);
                    }
                }
            }

        });


    }


    pub fn get() -> &'static Producer {
        &SP
    }

    pub fn pressdata(&self, tx: Sender<Api>) {
        println!("Received new thread");
        let mut DEBUG = VarName::new();
        DEBUG.Instant();
        // println!("hm len ={}", hm.len());

        let pair = self.cvar.clone();
        let &(ref lock, ref cvar) = &*pair;
        // Keep receiving in a loop, until tx is dropped!
        let mut qdata = lock.lock().unwrap();

        let l: Queue<String> = Queue::new();

        let mut Arrylist: Vec<Queue<String>> = vec![l; 120];
        //    let mut List: Arc<RefCell<LinkedList<String>>>  = Arc::new(RefCell::new(LinkedList::new()));
        //  let mut rw_lock: Arc<RwLock<Vec<Queue<String>>>> = Arc::new(RwLock::new(Arrylist));
        // let wlist = rw_lock.clone();
        // let rlist = rw_lock.clone();

        let capi: HashMap<String, String> = checkimpl::Instant();

        // crossbeam::scope(|scope| {
        //
        // scope.spawn( move || {
        //  let ref List=    unsafe {&mut *  List.clone().get()};
        //
        //   let len = *aa.push();
        //
        // loop
        // {
        // if let Ok(mut r_guard) = rlist.read() {
        // thread::sleep_ms(2000);
        // println!("rguarf = {:?}",r_guard[0]);
        // }
        //
        // }
        //
        // });
        //
        //
        //
        // });





        loop {
            if !qdata.index.run() {
                 // println!("start to wait >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
                qdata = cvar.wait(qdata).unwrap();
            }
            let _r = qdata.index.dequeue();
            let ref mut data = qdata.data[_r]; //data: VecDeque<String >
         //    let d :Vec<String>   = data.iter().collect();
            let mut len = data.len() as u32;

         //  let sublen = len;
            let mut i = 0;

            while len > i {

                let s: String = data.get(i as usize).unwrap().to_string();
                // println!("_r ={} ,len {} data[{}] = {:?}", _r, len, i, s);
                //println!("org recv [{}] len={}", s,s.len());
               // filprintln("glVertexAttribPointer",&s);
                if s.contains("glShaderSource") {


                    let mut source = data.get(i as usize).unwrap().to_string();
                    // println!("glShaderSource = { }", s);
                    // j = i + 1;i//
                    loop {
                        i += 1;
                        let s = data.get(i as usize).unwrap();
                  //      println!("org recv [{}] len={}", s,s.len());
                        if s.contains("GLint*") || s.contains("GLint *") {
                            source += "\n";
                            source += ":";
                            source += s;
                            i += 1;
                            break;
                        }
                        // j += 1;
                        source += "\n";
                        source += s;
                        source += " ";

                    }

                    println!("org recv {}", source);

                    let mut v: Vec<&str> = source.split(':').collect();
                    //DEBUG.println(&v);
                    let api = Api::new(&mut v);
                    tx.send(api).unwrap();
                    //  println!("glShaderSource  len  = {} i ={}{}", len, i, source);

                    continue;
                }

                i += 1;
                // len -= 1;
                let mut v: Vec<&str> = s.split(':').collect();
               // DEBUG.println(&v);
                let mut length = v.len();
                //  println!("recv  len  = {} data[{}] ={:?}", len, length, v);

                if length >= 2 {
                    let api = Api::new(&mut v);
                    // this is check some one isn't impl
/*                    match capi.get(&api.name) {
                        None => {
                            if api.name.contains("()") {
                                println!("checkapiimpl not impl {:?}", api);
                            }
                        }
                        Some(_) => {}
                    }*/

                    // this is check some one isn't impl
                    tx.send(api).unwrap();
                }

            }

            data.clear();
        }


    }
}
