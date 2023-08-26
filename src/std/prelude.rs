pub mod v1 {
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::string::{String, ToString};
    pub use alloc::vec::Vec;
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
