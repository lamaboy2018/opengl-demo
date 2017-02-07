#![allow(dead_code)]


#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

#![allow(unused_must_use)]
#![allow(unused_mut)]

#[macro_use]
extern crate lazy_static;
#[warn(non_snake_case)]
extern crate glfw;

extern crate num;
extern crate libc;
// mod start
mod dataprocess;
mod glx;
mod debug;
#[macro_use]
mod core;
mod utils;
use glx::gl;
// end

use std::collections::LinkedList;
use std::cell::UnsafeCell;
// use std::net::{TcpListener, TcpStream};
use std::thread;
use glfw::{Action, Context, Key};
use std::sync::{Arc, RwLock};

use std::sync::mpsc::{channel, Sender, Receiver};
use dataprocess::Process::Producer;
use libc::memcpy;

use core::api::*;




use core::render::readering;
use core::renderTest;
use core::renderobj::*;
use debug::pyramid;
use debug::translate;
static  mut mViewMatrix : [f32;16] =[
    -3.42659,    0.02738219,    -0.023744134,    -0.022585884,
    0.04622666,    1.9997377,    0.009685329,    0.009212874,
    -0.07784159,    -0.0178102,    1.0509692,    0.99970245,
    0.0,    0.0,    1.1025641,    3.0
];


fn main() {
    println!("Hello, world!");

    //  let mut List: Arc<RwLock<LinkedList<String>>>  = Arc::new(RwLock::new(LinkedList::new()));
    let mut List: Arc<UnsafeCell<LinkedList<Api>>> = Arc::new(UnsafeCell::new(LinkedList::new()));
    //  unsafe {&mut * obj.index.get()};
    let mut wlist = unsafe { &mut *List.clone().get() };
    let mut rlist = unsafe { &mut *List.clone().get() };
    let mut startRead: Arc<RwLock<usize>> = Arc::new(RwLock::new(0));
    let wstartRead = startRead.clone();
    let wr = startRead.clone();
    let rstartRead = startRead.clone();

  //  static  mut rw : i32= 0;
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw::WindowHint::ContextVersion(3, 2);
    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::ContextVersion(4, 1);
    println!("{:?}", glfw::get_version());
    println!("GLFW version: {:?}", glfw::get_version_string());

    let (mut window, events) =
        glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    window.hide();
    window.set_key_polling(true);
    window.make_current();
    //  refwindow = Arc::new(window);
    let (tx, rx): (Sender<Api>, Receiver<Api>) = channel();
    let p = Producer::get();
    p.start_net();

    thread::Builder::new().name("send api".to_string()).spawn(move || {
        p.pressdata(tx);
    });

    thread::Builder::new().name("recv api ".to_string()).spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("write_guard Received {:?}", n);

           if n.name.contains("eglSwapBuffers")
            /*if wlist.len() > 128*/ {

                if let Ok(mut write_guard) = wstartRead.write() {
                    //   println!("write_guard Received {}", n);
                    *write_guard = 1;

                }
            }
            wlist.push_front(n);

        }
    });


    unsafe {gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Viewport(0, 0, 800, 600);
    }
    window.show();
  //  let mut  Render = render::new();

    let mut  rtid =0;
  // let mut mpyramid  = pyramid::pyramid::new();
   // let mut translate = translate::translate::mutinstance();
    println!("test start --------------- - - - - ->");
 //   let vs = mpyramid.complile_shader(gl::VERTEX_SHADER);
 //   let fs = mpyramid.complile_shader(gl::FRAGMENT_SHADER);
  //  println!(" vs ={},fs ={}",vs,fs);
  //  println!("test end --------------- - - - - ->");

  //  let program =  mpyramid.link_program(vs,fs);

     while !window.should_close() {


/*       unsafe {
                           mpyramid.draw(&mut mViewMatrix);
           window.swap_buffers();
                        }*/
        let mut f = 0;
        if let Ok(r_guard) = rstartRead.read() {
            // thread::sleep_ms(2000);
            // println!("rguarf = {}",*r_guard);
            f = *r_guard;
        }

        if f > 0 && !rlist.is_empty()  {
         //   println!(" len ={}", rlist.len());
            let a = rlist.pop_back();


         //   println!(" a = {:?} len  = {} ", a, rlist.len());
            if a.is_some()
                {
                    let api: Api = a.unwrap();

                    if rtid == 0 {
                        rtid = api.tid;
                        println!("rtid = {}", rtid);
                    }


                    match api.name.as_ref() {
                        "eglSwapBuffers()" => {
                            window.swap_buffers();
                            unsafe {
                                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                                gl::Enable(gl::DEPTH_TEST);
                            }
                        }
                        _ => {}
                    }
                    if api.tid != rtid {
                      //    unsafe {mpyramid.draw(api,&mut mViewMatrix);}
                          unsafe {core::engine::startDraw(api);}
                      //  unsafe { translate::translate::mutinstance().draw(api, &mut mViewMatrix); }
                    }
                    /*            if api.tid != rtid {
                unsafe  {
                    let mut floag :i32  =0;
                    gl::GetIntegerv(gl::CONTEXT_FLAGS,&mut floag);
                    let val = floag as gl::types::GLenum;
                    if (val & gl::CONTEXT_FLAG_ROBUST_ACCESS_BIT) != 0 {
                        println!("CONTEXT_FLAGS  = flase");
                    }
                   // readering(api);
                    if !window.is_current(){
                        println!("is_current = flase");
                    }
                }


            }*/
                    //thread::sleep_ms(1);
                }/*else{
                //thread::sleep_ms(1);
            }*/

/*            if let Ok(mut w) = wr.try_write() {
                //   println!("write_guard Received {}", n);
                *w = 0;

            }*/
        }else {
            thread::sleep_ms(1);
        }


        //  window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
