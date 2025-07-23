#[rustfmt::skip]
mod inner {
    #![allow(unused)]
    use greycat::sys;
    
    pub(crate) const NULL: u8         = sys::gc_type_null as u8;
    pub(crate) const BOOL: u8         = sys::gc_type_bool as u8;
    pub(crate) const CHAR: u8         = sys::gc_type_char as u8;
    pub(crate) const INT: u8          = sys::gc_type_int as u8;
    pub(crate) const FLOAT: u8        = sys::gc_type_float as u8;
    pub(crate) const NODE: u8         = sys::gc_type_node as u8;
    pub(crate) const NODE_TIME: u8    = sys::gc_type_node_time as u8;
    pub(crate) const NODE_INDEX: u8   = sys::gc_type_node_index as u8;
    pub(crate) const NODE_LIST: u8    = sys::gc_type_node_list as u8;
    pub(crate) const NODE_GEO: u8     = sys::gc_type_node_geo as u8;
    pub(crate) const GEO: u8          = sys::gc_type_geo as u8;
    pub(crate) const TIME: u8         = sys::gc_type_time as u8;
    pub(crate) const DURATION: u8     = sys::gc_type_duration as u8;
    pub(crate) const CUBIC: u8        = sys::gc_type_cubic as u8;
    pub(crate) const STATIC_FIELD: u8 = sys::gc_type_static_field as u8;
    pub(crate) const OBJECT: u8       = sys::gc_type_object as u8;
    pub(crate) const T2: u8           = sys::gc_type_t2 as u8;
    pub(crate) const T3: u8           = sys::gc_type_t3 as u8;
    pub(crate) const T4: u8           = sys::gc_type_t4 as u8;
    pub(crate) const STR: u8          = sys::gc_type_str as u8;
    pub(crate) const T2F: u8          = sys::gc_type_t2f as u8;
    pub(crate) const T3F: u8          = sys::gc_type_t3f as u8;
    pub(crate) const T4F: u8          = sys::gc_type_t4f as u8;
    pub(crate) const BLOCK_REF: u8    = sys::gc_type_block_ref as u8;
    pub(crate) const BLOCK_INLINE: u8 = sys::gc_type_block_inline as u8;
    pub(crate) const FUNCTION: u8     = sys::gc_type_function as u8;
    pub(crate) const UNDEFINED: u8    = sys::gc_type_undefined as u8;
    pub(crate) const TYPE: u8         = sys::gc_type_type as u8;
    pub(crate) const FIELD: u8        = sys::gc_type_field as u8;
    pub(crate) const STRINGLIT: u8    = sys::gc_type_stringlit as u8;
    pub(crate) const ERROR: u8        = sys::gc_type_error as u8;
}

pub(crate) use inner::*;
