//! The `Ipld` crate.
#![deny(missing_docs)]
#![deny(warnings)]

pub mod block;
pub mod encode_decode;
pub mod error;
pub mod mem;
pub mod path;
pub mod store;

#[cfg(feature = "dag-cbor")]
pub use libipld_cbor as cbor;
#[cfg(all(feature = "dag-cbor", feature = "derive"))]
pub use libipld_cbor_derive::DagCbor;
pub use libipld_core::*;
#[cfg(feature = "dag-json")]
pub use libipld_json as json;
pub use libipld_macro::*;
#[cfg(feature = "dag-pb")]
pub use libipld_pb as pb;

/// The maximum block size is 1MiB.
pub const MAX_BLOCK_SIZE: usize = 1_048_576;
