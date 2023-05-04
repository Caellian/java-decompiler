use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use java_decompiler::file::jar::Jar;
use java_decompiler::gen::java::class::ClassContext;
use java_decompiler::gen::java::JavaBackend;
use java_decompiler::gen::Generate;
use java_decompiler::gen::GeneratorBuilder;

pub use jvm_class_format as class;
use tracing::Level;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.pop().expect("couldn't get program name from args");

    let jar = Jar::new("./ref/java.jar").unwrap();

    let class = jar.classes().skip(4).next().unwrap();
    println!("{}", class.class_name);

    let lang = GeneratorBuilder::java()
        .header("decompiler development test")
        .build();
    let out = File::create("./sample_output.java").expect("unable to create output file");

    let mut w = BufWriter::new(out);

    JavaBackend::write_value(&lang, &mut ClassContext, &class, &mut w)
        .expect("unable to generate class code");

    w.flush().expect("unable to flush");
}
