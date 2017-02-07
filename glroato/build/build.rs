// Copyright 2015 Brendan Zabarauskas and the gl-rs developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate gl_generator;

use gl_generator::{Registry, Fallbacks, GlobalGenerator, Api, Profile};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {/*
    //let out_dir = env::var("OUT_DIR").unwrap();
    //let mut file = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();
    let mut file = File::create("src/glx/gl45/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl44/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 4), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
    let mut file = File::create("src/glx/gl43/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl42/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 2), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl41/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl40/mod.rs").unwrap();
    Registry::new(Api::Gl, (4, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl33/mod.rs").unwrap();
    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();



    let mut file = File::create("src/glx/gl32/mod.rs").unwrap();
    Registry::new(Api::Gl, (3, 2), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();


    let mut file = File::create("src/glx/gl31/mod.rs").unwrap();
    Registry::new(Api::Gl, (3, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl30/mod.rs").unwrap();
    Registry::new(Api::Gl, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();


    let mut file = File::create("src/glx/gl21/mod.rs").unwrap();
    Registry::new(Api::Gl, (2, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gl20/mod.rs").unwrap();
    Registry::new(Api::Gl, (2, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();


    let mut file = File::create("src/glx/gles32/mod.rs").unwrap();
    Registry::new(Api::Gles2, (3, 2), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();


    let mut file = File::create("src/glx/gles31/mod.rs").unwrap();
    Registry::new(Api::Gles2, (3, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let mut file = File::create("src/glx/gles30/mod.rs").unwrap();
    Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap(); */
}
