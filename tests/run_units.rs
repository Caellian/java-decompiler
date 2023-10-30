use std::cell::OnceCell;
use std::fs::{DirEntry, File};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;

use jaded::gen::java::JavaBackend;
use jaded::gen::{GenerateCode, GeneratorBuilder};
use jvm_class_format::error::ClassReadError;
use jvm_class_format::Class;

fn javac(source: impl AsRef<Path>) -> Command {
    static mut JAVA_HOME: OnceCell<PathBuf> = OnceCell::new();
    let javac = unsafe {
        JAVA_HOME.get_or_init(|| match std::env::var("JAVA_HOME") {
            Ok(it) => PathBuf::from_str((it + "/bin/javac").as_str()).unwrap(),
            Err(_) => PathBuf::from_str("javac").unwrap(),
        })
    };

    let mut c = Command::new(javac);
    c.args([
        "-nowarn",
        source.as_ref().to_str().expect("invalid source path"),
    ]);
    c
}

// root structure of a java file is a class
pub fn compile(source: impl AsRef<Path>) -> Result<Vec<u8>, std::io::Error> {
    let mut javac_command = javac(source)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let compilation_result = javac_command.wait()?;

    if !compilation_result.success() {
        // If compilation fails, capture the error output and include it in the error message.
        let mut error_output = String::new();
        if let Some(mut stderr) = javac_command.stderr {
            stderr.read_to_string(&mut error_output)?;
        }

        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("compile error:\n{}", error_output),
        ));
    }

    let result = std::fs::read("tests/units/Unit.class")?;
    let _ = std::fs::remove_file("tests/units/Unit.class"); // it will be overriden
    Ok(result)
}

fn entry_num(entry: &DirEntry) -> usize {
    entry
        .file_name()
        .to_string_lossy()
        .chars()
        .take_while(|it| it.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[test]
fn run_units() -> Result<(), ClassReadError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let lang = GeneratorBuilder::java().no_header().build();

    let units = std::fs::read_dir("tests/units").expect("can't iterate test units");
    let mut units = units
        .into_iter()
        .filter_map(|it| it.ok())
        .collect::<Vec<_>>();
    units.sort_by_key(entry_num);

    let mut any_failed = false;
    for unit in units.into_iter() {
        let filename = unit.file_name().to_string_lossy().to_string();
        tracing::info!("Testing unit: {}", filename);
        let binary = match compile(unit.path()) {
            Ok(it) => it,
            Err(err) => {
                tracing::error!("{}", err);
                continue;
            }
        };

        let hello_world = Class::read(binary)?;

        let result = JavaBackend
            .generate(&lang, &(), &hello_world)
            .expect("unable to generate class code")
            .0;

        let source = std::fs::read_to_string(unit.path()).unwrap();

        if source != result {
            tracing::error!(
                "Unit {} failed\n{}",
                filename,
                pretty_assertions::StrComparison::new(&source, &result).to_string()
            );
            any_failed = true;
        } else {
            tracing::info!("Unit {} passed", filename);
        }
    }
    assert!(!any_failed, "some tests failed; check the logs");

    Ok(())
}
