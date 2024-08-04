use core::fmt;

use super::Erroring;

#[derive(Debug)]
pub enum AsmErr {
    ReadFile,
    OpenFile,
    Parse,
    Write,
}

impl Erroring for AsmErr {
    fn err(&self) -> &str {
        match self {
            Self::ReadFile => "failed to read file to buffer",
            Self::OpenFile => "failed to open file",
            Self::Parse => "failed to parse token",
            Self::Write => "failed to write binary to stdout",
        }
    }
}

impl fmt::Display for AsmErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err())
    }
}

#[macro_export]
macro_rules! asme {
    ($err: expr, $kind: ident, $($args:tt)*) => {{
        use $crate::errors::Erroring;
        $err.vme(format!("{}: {}", $crate::errors::vme::AsmErr::$kind.err(), format_args!($($args)*)).as_str())
    }};

    ($kind:ident, $($args:tt)*) => {{
        use $crate::errors::Erroring;
        $crate::errors::Jerror::Vme(format!("{}: {}", $crate::errors::vme::AsmErr::$kind.err(), format_args!($($args)*)))
    }};
}
