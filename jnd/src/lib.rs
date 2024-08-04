pub mod disassembler;
pub mod errors;
pub mod interrupts;
pub mod macros;
pub mod mem;
pub mod op;
pub mod reg;
pub mod vm;

pub type Res<T> = Result<T, errors::Jerror>;
