use std::fmt::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Indent {
    Space,
    Tab,
}

impl Indent {
    pub fn repeat(&self, count: usize) -> String {
        match self {
            Indent::Space => " ".repeat(count),
            Indent::Tab => "\t".repeat(count),
        }
    }
}

pub struct CodeWriter<W: Write> {
    inner: W,

    pub state_indent: usize,

    pub indent_kind: Indent,
    pub indent_level_count: usize,
}

impl<W: Write> CodeWriter<W> {
    pub fn new(writer: W) -> CodeWriter<W> {
        CodeWriter {
            inner: writer,

            state_indent: 0,

            indent_kind: Indent::Space,
            indent_level_count: 4,
        }
    }

    #[inline]
    pub fn enter_block(&mut self) {
        self.state_indent += 1;
    }

    #[inline]
    pub fn exit_block(&mut self) {
        if self.state_indent > 0 {
            self.state_indent -= 1;
        } else {
            tracing::warn!("tried exiting block indentation on 0 indentation")
        }
    }

    #[inline]
    pub fn indent_string(&self) -> String {
        self.indent_kind.repeat(self.indent_level_count)
    }
}

impl<W: Write> Write for CodeWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            match c {
                '\n' => {
                    self.inner.write_char('\n')?;
                    self.inner.write_str(self.indent_string().as_str())?;
                }
                other => {
                    self.inner.write_char(other)?;
                }
            }
        }
        Ok(())
    }
}
