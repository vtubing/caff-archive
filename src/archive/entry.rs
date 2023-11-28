use crate::prelude::*;

mod metadata;
pub use metadata::Metadata;

pub struct Entry {
  pub metadata: Metadata,
  pub content: Vec<u8>,
}

impl Entry {
  pub fn new(metadata: Metadata, content: Vec<u8>) -> Self {
    Self { metadata, content }
  }
}
