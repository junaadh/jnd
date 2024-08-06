use std::fmt::Arguments;

#[macro_export]
macro_rules! jassert_eq {
    ($left: expr, $right: expr) => {
        $crate::macros::assert_eq($left as usize, $right as usize)
    };
}

#[macro_export]
macro_rules! jassert {
    ($expr: expr, $($args: tt)*) => {
        $crate::macros::assert($expr, format_args!($($args)*))
    };
}

pub fn assert_eq(left: usize, right: usize) -> crate::Res<()> {
    if left == right {
        Ok(())
    } else {
        Err(crate::asme!(
            MissingArg,
            "expected: {left}, actual: {right}"
        ))
    }
}

pub fn assert(expr: bool, args: Arguments) -> crate::Res<()> {
    if expr {
        Ok(())
    } else {
        Err(crate::asme!(MissingArg, "{}", args))
    }
}
