use core::panic;

pub mod asme;
pub mod vme;

pub trait Erroring {
    fn err(&self) -> &str;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Jerror {
    Vme(String),
    Asme(String),
}

impl Jerror {
    pub fn vme(&self, err: &str) -> Self {
        match self {
            Self::Vme(e) => Self::Vme(format!("{e}: {err}")),
            _ => panic!("This shouldnt happen. Jerror::vme shoudlnt access asme"),
        }
    }

    pub fn asme(&self, err: &str) -> Self {
        match self {
            Self::Asme(e) => Self::Asme(format!("{e}: {err}")),
            _ => panic!("This shouldnt happen. Jerror::asme shoudlnt access vme"),
        }
    }
}
