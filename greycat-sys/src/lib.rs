#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod ext;
#[allow(unnecessary_transmutes)]
#[allow(clippy::useless_transmute)]
#[allow(clippy::transmute_int_to_bool)]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::ptr_offset_with_cast)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
mod generated;

pub use generated::*;
