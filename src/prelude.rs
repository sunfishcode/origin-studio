//! The Rust Prelude

pub mod v1 {
    pub use crate::borrow::ToOwned;
    pub use crate::boxed::Box;
    pub use crate::string::{String, ToString};
    pub use crate::vec::Vec;
    pub use core::prelude::v1::*;
}
pub mod rust_2015 {
    pub use super::v1::*;
}
pub mod rust_2018 {
    pub use super::rust_2015::*;
}
pub mod rust_2021 {
    pub use super::rust_2018::*;
    pub use core::convert::{TryFrom, TryInto};
    pub use core::iter::FromIterator;
}
