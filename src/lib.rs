#![allow(unused)]

mod archive;
mod error;
mod padding;

pub(crate) mod prelude {
  pub(crate) use crate::archive::Key;
  pub(crate) use crate::error::Error;
  pub(crate) use crate::padding::Padding;
  pub(crate) use crate::Result;
  pub(crate) use crate::{ReadEncryptedBytes, ReadTracing, WriteEncryptedBytes, WriteTracing};
  pub(crate) use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
  pub(crate) use std::io::{Cursor, Read, Seek, SeekFrom, Write};
  #[cfg(test)]
  pub(crate) use test_strategy::Arbitrary;
}

use prelude::*;

pub use archive::Archive;
pub use archive::Header;
pub use archive::Key;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait ReadEncryptedBytes: ReadBytesExt {
  fn read_encrypted_u8(&mut self, key: Key) -> std::io::Result<u8> {
    self.read_u8().map(|value| value ^ key)
  }

  fn read_encrypted_bool(&mut self, key: Key) -> std::io::Result<bool> {
    self.read_encrypted_u8(key).map(|value| value != 0)
  }

  fn read_encrypted_u32<T: ByteOrder>(&mut self, key: Key) -> std::io::Result<u32> {
    self.read_u32::<T>().map(|value| value ^ key)
  }

  fn read_encrypted_varint(&mut self, key: Key) -> Result<usize> {
    match self.read_encrypted_u8(key) {
      Err(error) => Err(error.into()),
      Ok(value) if value & 0x80 != 0 => Err(Error::VarIntSupportNotImplemented),
      Ok(value) => Ok((value & 0x7F).into()),
    }
  }

  fn read_encrypted_bytes(&mut self, key: Key) -> Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(self.read_encrypted_varint(key)?);
    for _ in 0..bytes.capacity() {
      let byte = self.read_encrypted_u8(key)?;
      bytes.push(byte);
    }

    Ok(bytes)
  }

  fn read_encrypted_string(&mut self, key: Key) -> Result<String> {
    let bytes = self.read_encrypted_bytes(key)?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
  }
}

impl<T> ReadEncryptedBytes for T where T: ReadBytesExt {}

pub(crate) trait WriteEncryptedBytes: WriteBytesExt {
  fn write_encrypted_u8(&mut self, value: u8, key: Key) -> std::io::Result<()> {
    self.write_u8(value ^ key)
  }

  fn write_encrypted_bool(&mut self, value: bool, key: Key) -> std::io::Result<()> {
    self.write_encrypted_u8(value.into(), key)
  }

  fn write_encrypted_u32<T: ByteOrder>(&mut self, value: u32, key: Key) -> std::io::Result<()> {
    self.write_u32::<T>(value ^ key)
  }

  fn write_encrypted_varint(&mut self, value: usize, key: Key) -> Result<()> {
    match u8::try_from(value) {
      Err(error) => Err(error.into()),
      Ok(value) if value & 0x80 != 0 => Err(Error::VarIntSupportNotImplemented),
      Ok(value) => self.write_encrypted_u8(value, key).map_err(Into::into),
    }
  }

  fn write_encrypted_bytes(&mut self, bytes: &[u8], key: Key) -> Result<()> {
    self.write_encrypted_varint(bytes.len(), key)?;

    for &byte in bytes {
      self.write_encrypted_u8(byte, key)?;
    }

    Ok(())
  }

  fn write_encrypted_string(&mut self, string: &str, key: Key) -> Result<()> {
    self.write_encrypted_bytes(string.as_bytes(), key)
  }
}

impl<T> WriteEncryptedBytes for T where T: WriteBytesExt {}

pub(crate) trait ReadTracing: Read + Seek {
  fn trace<F: FnMut(&mut Self) -> Result<T>, T>(&mut self, mut function: F) -> Result<T> {
    #[cfg(not(feature = "logging"))]
    return function(self);

    #[cfg(feature = "logging")]
    {
      let initial_stream_position = self.stream_position()?;
      log::trace!("read -> address={:#010X?}", initial_stream_position);

      let value = function(self)?;

      let final_stream_position = self.stream_position()?;
      log::trace!("read size={}", final_stream_position - initial_stream_position);
      log::trace!("read <- address={:#010X?}", final_stream_position);

      Ok(value)
    }
  }
}

impl<T> ReadTracing for T where T: Read + Seek {}

pub(crate) trait WriteTracing: Write + Seek {
  fn trace<F: FnMut(&mut Self) -> Result<T>, T>(&mut self, mut function: F) -> Result<T> {
    #[cfg(not(feature = "logging"))]
    return function(self);

    #[cfg(feature = "logging")]
    {
      let initial_stream_position = self.stream_position()?;
      log::trace!("write -> address={:#010X?}", initial_stream_position);

      let value = function(self)?;

      let final_stream_position = self.stream_position()?;
      log::trace!("write size={}", final_stream_position - initial_stream_position);
      log::trace!("write <- address={:#010X?}", final_stream_position);

      Ok(value)
    }
  }
}

impl<T> WriteTracing for T where T: Write + Seek {}
