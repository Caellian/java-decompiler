use std::{io::Write, collections::HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum IndentKind {
    Space(usize),
    Tab,
}

impl ToString for IndentKind {
    fn to_string(&self) -> String {
        match self {
            IndentKind::Space(count) => " ".repeat(*count),
            IndentKind::Tab => "\t".to_string(),
        }
    }
}

pub struct Indented<W: Write> {
    inner: W,
    pub level: usize,
    pub indent: IndentKind,

    pub enter_block_on: HashSet<u8>,
    pub exit_block_on: HashSet<u8>,
}

impl<W: Write> Indented<W> {
    pub fn new(writer: W, indent: IndentKind, level: usize, enter_block_on: impl AsRef<[u8]>, exit_block_on: impl AsRef<[u8]>) -> Indented<W> {
        Indented {
            inner: writer,
            level,
            indent,

            enter_block_on: HashSet::from_iter(enter_block_on.as_ref().iter().cloned()),
            exit_block_on: HashSet::from_iter(exit_block_on.as_ref().iter().cloned()),
        }
    }
    
    #[inline]
    pub fn enter_block(&mut self) {
        self.level += 1;
    }

    #[inline]
    pub fn exit_block(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        } else {
            tracing::warn!("tried exiting block indentation on 0 indentation")
        }
    }

    #[inline]
    pub fn indent_string(&self) -> String {
        if self.level == 0 {
            return String::new();
        }

        self.indent.to_string().repeat(self.level)
    }

    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for Indented<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut total = 0;
        for byte in buf {
            let count = match *byte as char {
                '\n' => {
                    let nl_len = self.inner.write(b"\n")?;
                    let indent_len = self.inner.write(self.indent_string().as_bytes())?;
                    nl_len + indent_len
                }
                _ if self.enter_block_on.contains(byte) => {
                    self.enter_block();
                    self.inner.write(&[*byte])?
                }
                _ if self.exit_block_on.contains(byte) => {
                    self.exit_block();
                    self.inner.write(&[*byte])?
                }
                _ => self.inner.write(&[*byte])?,
            };
            total += count;
        }

        Ok(total)
    }
    
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let written = self.write(buf)?;
        // TODO: Do a proper check
        // needs to handle indentation levels
        // rope science - https://xi-editor.io/docs/rope_science_04.html
        if written < buf.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "didn't write enough code"))
        }
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
