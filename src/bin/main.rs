use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use jaded::gen::java::JavaBackend;
use jaded::gen::GenerateCode;
use jaded::gen::GeneratorBuilder;
use jaded::settings::Settings;
use jvm_class_format::Class;

use clap::Parser;
use tracing::Level;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    pub input: PathBuf,
    pub output: PathBuf,

    #[command(flatten)]
    pub settings: Settings,
}

fn main() {
    #[cfg(debug_assertions)]
    let log_level = Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let log_level = Level::INFO;

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let args = Arguments::parse();

    let class = Class::open(args.input).expect("can't open class");

    let lang = GeneratorBuilder::java().build();
    let out = File::create(args.output).expect("unable to create output file");

    let mut w = BufWriter::new(out);

    JavaBackend
        .write_value(&lang, &(), &class, &mut w)
        .expect("unable to generate class code");

    w.flush().expect("unable to flush");
}
