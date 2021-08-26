use crate::error::ManifestParseError;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone)]
pub struct Attributes(HashMap<String, String>);

impl Attributes {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Manifest {
    main_section: Attributes,
    entries: HashMap<String, Attributes>,
}

impl Manifest {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Manifest, ManifestParseError> {
        let mut main_section: Option<HashMap<String, String>> = None;
        let mut section: HashMap<String, String> = HashMap::with_capacity(2);
        let mut entries: HashMap<String, Attributes> = HashMap::new();

        let mf_reader = BufReader::new(r);

        let mut last_key: Option<String> = None;

        for line in mf_reader.lines() {
            let l = line.unwrap();

            if l.is_empty() {
                if main_section.is_none() {
                    main_section = Some(section.clone());
                } else {
                    entries.insert(
                        section
                            .get("Name")
                            .ok_or(ManifestParseError::InvalidEntry)?
                            .clone(),
                        Attributes(section.clone()),
                    );
                }
            }

            match l.chars().next() {
                None => {}
                Some(first_char) => match first_char {
                    '#' => continue,
                    ' ' => {
                        let mut appended = String::new();
                        l.chars().skip(1).for_each(|c| appended.push(c));

                        match last_key {
                            None => return Err(ManifestParseError::MisplacedContinuation),
                            Some(ref key) => {
                                section.insert(
                                    key.clone(),
                                    section.get(key).expect("previous key not set").clone()
                                        + appended.as_str(),
                                );
                            }
                        }

                        continue;
                    }
                    _ => {
                        let mut kv: Vec<&str> = l.split(": ").collect();
                        let key = kv.remove(0).to_string();
                        let value = kv.join("");

                        last_key = Some(key.clone());
                        section.insert(key, value);
                    }
                },
            }
        }

        if main_section.is_none() {
            main_section = Some(section);
        } else {
            entries.insert(
                section
                    .get("Name")
                    .ok_or(ManifestParseError::InvalidEntry)?
                    .clone(),
                Attributes(section.clone()),
            );
        }

        Ok(Manifest {
            main_section: Attributes(main_section.unwrap()),
            entries,
        })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.main_section.get(key)
    }

    pub fn entry_get(&self, entry: &str, key: &str) -> Option<&str> {
        self.entries.get(entry).map(|a| a.get(key)).flatten()
    }
}
