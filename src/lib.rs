#![no_std]

mod rasengan;

pub mod prelude {
    pub use crate::rasengan::*;
}

pub use rasengan::*;
