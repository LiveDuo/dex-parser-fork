//! Dex is a library for reading Android's
//! [dex](https://source.android.com/devices/tech/dalvik/dex-format) file format.
// Silence warnings in error module for now
#![allow(bare_trait_objects)]

pub extern crate scroll;

#[macro_use]
extern crate scroll_derive;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;

extern crate getset;

pub use error::Error;

pub use crate::dex::{Dex, DexReader, Header};

#[macro_use]
mod utils;
pub mod annotation;
mod cache;
pub mod class;
pub mod code;
mod dex;
mod encoded_item;
pub mod encoded_value;
mod error;
pub mod field;
pub mod jtype;
pub mod method;
mod search;
mod source;
pub mod string;

/// The constant NO_INDEX is used to indicate that an index value is absent.
pub const NO_INDEX: uint = 0xffff_ffff;
const ENDIAN_CONSTANT: (ubyte, ubyte, ubyte, ubyte) = (0x12, 0x34, 0x56, 0x78);
const REVERSE_ENDIAN_CONSTANT: (ubyte, ubyte, ubyte, ubyte) = (0x78, 0x56, 0x34, 0x12);

/// 8-bit signed int
#[allow(non_camel_case_types)]
pub type byte = i8;
/// 32-bit unsigned int
#[allow(non_camel_case_types)]
pub type uint = u32;
/// 32-bit signed int
#[allow(non_camel_case_types)]
pub type int = i32;
/// 16-bit unsigned int
#[allow(non_camel_case_types)]
pub type ushort = u16;
/// 16-bit signed int
#[allow(non_camel_case_types)]
pub type short = i16;
/// 8-bit unsigned int
#[allow(non_camel_case_types)]
pub type ubyte = u8;
/// 64-bit unsigned int
#[allow(non_camel_case_types)]
pub type ulong = u64;
/// 64-bit signed int
#[allow(non_camel_case_types)]
pub type long = i64;

/// A `Result` of `T` or an error of `error::Error`
pub type Result<T> = std::result::Result<T, error::Error>;

// ref. https://source.android.com/devices/tech/dalvik/dex-format

/// The endianness of bytes.
pub type Endian = scroll::Endian;


#[cfg(test)]
mod tests {

    #[test]
    fn test_file() {
        let _dex = super::DexReader::from_file("resources/compiled.dex").unwrap();
        dbg!(_dex.inner);

        // let header = _dex.header();
        let _header = crate::Header {
            magic: [0; 8],
            checksum: 0,
            signature: [0; 20],
            file_size: 0,
            header_size: 0,
            endian_tag: [0; 4],
            link_size: 0,
            link_off: 0,
            map_off: 0,
            string_ids_size: 0,
            string_ids_off: 0,
            type_ids_size: 0,
            type_ids_off: 0,
            proto_ids_size: 0,
            proto_ids_off: 0,
            field_ids_size: 0,
            field_ids_off: 0,
            method_ids_size: 0,
            method_ids_off: 0,
            class_defs_size: 0,
            class_defs_off: 0,
            data_size: 0,
            data_off: 0
        };
    }

}
