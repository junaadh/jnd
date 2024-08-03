use core::fmt;

use super::Erroring;

#[derive(Debug)]
pub enum VMErr {
    WriteOutOfBounds,
    ReadOutOfBound,
    InvalidRegister,
    MemReadFail,
    MemRead2Fail,
    MemWriteFail,
    MemWrite2Fail,
    UnknownInstruction,
    InterruptHandlerNotFound,
    InterruptHandlerInsert,
}

impl Erroring for VMErr {
    fn err(&self) -> &str {
        match self {
            Self::WriteOutOfBounds => "write out of bounds",
            Self::ReadOutOfBound => "read out of bounds",
            Self::InvalidRegister => "accessed register invalid",
            Self::MemReadFail => "mem read failed",
            Self::MemRead2Fail => "mem read 16bit failed",
            Self::MemWriteFail => "mem write failed",
            Self::MemWrite2Fail => "mem write 16bit failed",
            Self::UnknownInstruction => "instruction unknown",
            Self::InterruptHandlerNotFound => "no known handlers for the interrupt",
            Self::InterruptHandlerInsert => "failed to insert interrupt handler",
        }
    }
}

impl fmt::Display for VMErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err())
    }
}

#[macro_export]
macro_rules! vme {
    ($err: expr, $kind: ident, $($args:tt)*) => {{
        use $crate::errors::Erroring;
        $err.vme(format!("{}: {}", $crate::errors::vme::VMErr::$kind.err(), format_args!($($args)*)).as_str())
    }};

    ($kind:ident, $($args:tt)*) => {{
        use $crate::errors::Erroring;
        $crate::errors::Jerror::Vme(format!("{}: {}", $crate::errors::vme::VMErr::$kind.err(), format_args!($($args)*)))
    }};
}

#[cfg(test)]
mod test {
    use crate::errors::Erroring;

    const ARM: usize = 0xdeadbeef;

    #[test]
    fn vme_macro_arm1() {
        let control = super::super::Jerror::Vme(format!(
            "{}: {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123"
        ));
        let test = vme!(WriteOutOfBounds, "testing 123");
        assert_eq!(control, test)
    }

    #[test]
    fn vme_macro_arm1_2() {
        let control = super::super::Jerror::Vme(format!(
            "{}: {} {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123",
            ARM
        ));
        let test = vme!(WriteOutOfBounds, "testing 123 {}", ARM);
        assert_eq!(control, test)
    }

    #[test]
    fn vme_macro_arm2() {
        let test = super::super::Jerror::Vme(format!(
            "{}: {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123"
        ));
        let test = vme!(test, MemReadFail, "123");

        let control = super::super::Jerror::Vme(format!(
            "{}: {}: {}: {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123",
            super::VMErr::MemReadFail,
            "123"
        ));

        assert_eq!(test, control)
    }

    #[test]
    fn vme_macro_arm2_2() {
        let test = super::super::Jerror::Vme(format!(
            "{}: {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123",
        ));
        let test = vme!(test, MemReadFail, "123 {}", ARM);

        let control = super::super::Jerror::Vme(format!(
            "{}: {}: {}: {} {}",
            super::VMErr::WriteOutOfBounds.err(),
            "testing 123",
            super::VMErr::MemReadFail,
            "123",
            ARM
        ));

        assert_eq!(test, control)
    }
}
