pub mod linear;

use crate::Res;

pub trait Addressable {
    fn read(&self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> Res<()>;

    fn read_u16(&self, addr: u16) -> Option<u16> {
        if let Some(x0) = self.read(addr) {
            if let Some(x1) = self.read(addr + 1) {
                return Some(x0 as u16 | ((x1 as u16) << 8));
            }
        }
        None
    }

    fn write_u16(&mut self, addr: u16, value: u16) -> Res<()> {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;
        self.write(addr, lower as u8)?;
        self.write(addr + 1, upper as u8)
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> Res<()> {
        for i in 0..n {
            let i = i as u16;
            if let Some(x) = self.read(from + i) {
                self.write(to + i, x)?;
            }
        }
        Ok(())
    }
}
