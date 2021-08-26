use crate::class::Class;
use crate::file::manifest::Manifest;
use std::fs::File;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Debug)]
pub struct Jar {
    path: PathBuf,

    manifest: Option<Manifest>,
    main_class: Option<String>,

    file_count: usize,
}

impl Jar {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Jar, std::io::Error> {
        let file = File::open(path.as_ref())?;
        let mut archive = ZipArchive::new(file)?;

        let (manifest, main_class) = {
            let mut mf_file = archive.by_name("META-INF/MANIFEST.MF")?;
            let manifest = match Manifest::read_from(&mut mf_file) {
                Ok(m) => Some(m),
                Err(err) => {
                    tracing::warn!("manifest parsing error: {}", err);
                    None
                }
            };
            let main_class = match &manifest {
                Some(m) => m.get("Main-Class").map(|s| s.to_string()),
                None => None,
            };
            (manifest, main_class)
        };

        Ok(Jar {
            path: path.as_ref().to_path_buf(),

            manifest,
            main_class,

            file_count: archive.len(),
        })
    }

    pub fn classes(&self) -> Classes {
        Classes {
            last: self.file_count - 1,
            current: 0,

            over: self,
        }
    }

    /*
    pub fn decompile(&mut self) {
        for file_number in 0..self.archive.len() {
            let mut f = &self.archive.by_index(file_number)?;

            if f.name().ends_with('/') { // Directory
                // Do nothing
            } else if f.name().ends_with(".class") {
                let mut content = Vec::with_capacity(f.size() as usize);
                f.read_to_end(&mut content)?;

                let mut reader = Cursor::new(content);

                let class = Class::read_from(&mut reader).unwrap();

                println!("{}:", f.name());
                println!("CP: {:?}", class.constant_pool);
                println!("Attributes: {:?}", class.attributes);
                println!("Fields: {:?}", class.fields);
                println!("Methods: {:?}", class.methods);

                let mut output = File::create(format!("ref/decomp/{}.java", class.class_name.name))?;

                let jg = JavaGenerator {
                    target_version: JavaVersion::Java16,
                    header_message: None
                };

                jg.generate(&class, &mut output);
                output.flush()?;
            } else {
                println!("{}", f.name());
            }
        }
    }
     */
}

pub struct Classes<'a> {
    pub over: &'a Jar,
    pub last: usize,

    pub current: usize,
}

impl<'a> Iterator for Classes<'a> {
    type Item = Class;

    fn next(&mut self) -> Option<Class> {
        if self.current > self.last {
            return None;
        }

        let file = match File::open(self.over.path.clone()) {
            Ok(f) => f,
            Err(err) => {
                tracing::error!(
                    "Unable to open zip file while iterating over classes: {}",
                    err
                );
                return None;
            }
        };
        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(err) => {
                tracing::error!(
                    "Unable to open zip archive while iterating over classes: {}",
                    err
                );
                return None;
            }
        };

        loop {
            let mut zip_file = match archive.by_index(self.current) {
                Ok(a) => a,
                Err(err) => {
                    tracing::error!(
                        "Unable to access file by index while iterating over classes: {}",
                        err
                    );
                    return None;
                }
            };

            self.current += 1;

            let name = zip_file.name();
            if name.ends_with(".class") {
                let class = match Class::read_from(&mut zip_file) {
                    Ok(c) => c,
                    Err(err) => {
                        tracing::error!(
                            "Unable to access file by index while iterating over classes: {}",
                            err
                        );
                        return None;
                    }
                };

                return Some(class);
            }
        }
    }
}
