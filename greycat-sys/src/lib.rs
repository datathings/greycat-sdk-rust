#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod ext;
#[allow(unnecessary_transmutes)]
#[allow(clippy::missing_safety_doc)]
mod generated;

pub use generated::*;
