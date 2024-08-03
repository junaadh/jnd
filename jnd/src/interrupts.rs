use crate::{vm::Machine, Res};

pub fn halt(vm: &mut Machine) -> Res<()> {
    vm.state = false;
    Ok(())
}
