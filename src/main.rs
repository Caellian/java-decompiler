use crate::file::jar::Jar;
use std::env;

pub mod class;
pub mod dependency;
pub mod error;
pub mod file;
pub mod gen;
pub mod model;
pub mod ext;

#[macro_use]
extern crate bitflags;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.pop().expect("couldn't get program name from args");

    let jar = Jar::new("./ref/java.jar").unwrap();

    let class = jar.classes().next().expect("no classes");
    println!("{}", class.class_name);

    for m in class.methods {
        println!("Method: {}", m.name);
        println!("{:?}", m);
    }

}
