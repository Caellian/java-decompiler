use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

use jaded::gen::java::JavaBackend;
use jaded::gen::{GenerateCode, GeneratorBuilder};
use jvm_class_format::Class;

// root structure of a java file is a class
pub fn compile(source: impl AsRef<str>) -> Result<Vec<u8>, std::io::Error> {
    let mut javac = Command::new("javac")
        .args(["-nowarn", source.as_ref()])
        .spawn()?;

    javac.wait().unwrap();

    let result = std::fs::read("tests/units/Unit.class");
    let _ = std::fs::remove_file("tests/units/Unit.class"); // it will be overriden
    result
}

#[test]
fn hello_world() {
    let src = compile("tests/units/01_hello_world.java").unwrap();
    let hello_world = Class::read(src).unwrap();

    let lang = GeneratorBuilder::java().no_header().build();

    let result = JavaBackend
        .generate(&lang, &(), &hello_world)
        .expect("unable to generate class code")
        .0;

    let out = File::create("./sample_output.java").expect("unable to create output file");
    let mut w = BufWriter::new(out);
    w.write(result.as_bytes()).unwrap();
    w.flush().unwrap();
}
