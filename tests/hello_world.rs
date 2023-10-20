use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;

use jaded::gen::java::JavaBackend;
use jaded::gen::{GenerateCode, GeneratorBuilder};
use jvm_class_format::error::ClassReadError;
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

const TEST_NAME: &str = "01_hello_world";
#[test]
fn hello_world() -> Result<(), ClassReadError> {
    #[cfg(feature = "tracing-subscriber")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    #[cfg(not(feature = "tracing-subscriber"))]
    println!("Running without logging");
    #[cfg(feature = "tracing-subscriber")]
    tracing::info!("- {}", TEST_NAME);

    let src = compile(format!("tests/units/{}.java", TEST_NAME)).unwrap();
    let hello_world = Class::read(src)?;

    let lang = GeneratorBuilder::java().no_header().build();

    let result = JavaBackend
        .generate(&lang, &(), &hello_world)
        .expect("unable to generate class code")
        .0;

    let out = File::create("./sample_output.java").expect("unable to create output file");
    let mut w = BufWriter::new(out);
    w.write(result.as_bytes()).unwrap();
    w.flush().unwrap();

    Ok(())
}
