use std::io::Write;

use crate::{op::Code, traits::WriteEncoded, Res};

#[derive(Debug, Default)]
pub struct Assembler {
    intern_buf: Vec<u8>,
}

impl Assembler {
    pub fn assemble(&mut self, words: &[&str]) -> Res<()> {
        words[0]
            .parse::<Code>()?
            .assemble(words, &mut self.intern_buf)?;
        Ok(())
    }

    pub fn push_raw(&mut self, instruction: u16) {
        self.intern_buf.push_raw(instruction);
    }

    pub fn buffer(&self) -> &[u8] {
        &self.intern_buf
    }
}

impl Write for Assembler {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.intern_buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
