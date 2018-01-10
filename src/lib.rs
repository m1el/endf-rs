#![allow(non_snake_case)]
#![deny(missing_docs)]

//! endf-rs - reading ENDF data files in Rust
//!
//! This library is designed for FORTRAN interop

pub mod error;
pub mod util;
pub use error::*;
pub use util::*;

pub mod decay;
pub mod description;
pub mod delayed_photon;
pub mod fission_yield;
pub mod tabular;

pub use decay::*;
pub use description::*;
pub use delayed_photon::*;
pub use fission_yield::*;
pub use tabular::*;

/*
decay mf=8 mt=457
decay mf=8 mt=459
cross sections mf=3 mt=*
*/
