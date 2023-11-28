use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Metadata {
  #[cfg_attr(test, filter(!#file_name.is_empty()))]
  pub file_name: String,
  pub tag: String,
  pub(crate) unknown_1: Padding<4>,
  pub(crate) unknown_2: Padding<4>,
  pub file_size: u32,
  pub is_obfuscated: bool,
  pub compression: u8,
  pub(crate) padding: Padding<8>,
}

impl Metadata {
  pub fn read<R: Read + Seek>(reader: &mut R, key: Key) -> Result<Self> {
    reader.trace(|reader| {
      let file_name = reader.read_encrypted_string(key)?;
      if file_name.is_empty() {
        Err(Error::EmptyFilename)?;
      }

      let tag = reader.read_encrypted_string(key)?;
      let unknown_1 = Padding::read(reader)?;
      let unknown_2 = Padding::read(reader)?;
      let file_size = reader.read_encrypted_u32::<BigEndian>(key)?;
      let is_obfuscated = reader.read_encrypted_bool(key)?;
      let compression = reader.read_encrypted_u8(key)?;
      let padding = Padding::read(reader)?;

      let value = Self {
        file_name,
        tag,
        unknown_1,
        unknown_2,
        file_size,
        is_obfuscated,
        compression,
        padding,
      };

      #[cfg(feature = "discovery")]
      for value in [value.unknown_1, value.unknown_2] {
        if !(value.is_empty() || value.is_all_u8_max()) {
          let mut bytes = value.into_inner();
          log::debug!("read (encrypted? [u8]) unknown hex={:02X?}, dec={:?}, ", bytes, bytes);

          let e = u32::from_be_bytes(bytes);
          log::debug!("read (encrypted? u32?) unknown hex={:#010X?}, bin={:034b}, dec={}", e, e, e);

          let d = e ^ key;
          log::debug!("read (decrypted? u32?) unknown hex={:#010X?}, bin={:034b}, dec={}", d, d, d);

          for mut u in bytes {
            u ^= key
          }

          log::debug!("read (decrypted? [u8]) unknown hex={:02X?}, dec={:?}", bytes, bytes);
        }
      }

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W, key: Key) -> Result<()> {
    writer.trace(|writer| {
      let Self {
        file_name,
        tag,
        unknown_1,
        unknown_2,
        file_size,
        is_obfuscated,
        compression,
        padding,
      } = self;

      writer.write_encrypted_string(file_name, key)?;
      writer.write_encrypted_string(tag, key)?;
      unknown_1.write(writer)?;
      unknown_2.write(writer)?;
      writer.write_encrypted_u32::<BigEndian>(*file_size, key)?;
      writer.write_encrypted_bool(*is_obfuscated, key)?;
      writer.write_encrypted_u8(*compression, key)?;
      padding.write(writer)?;

      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip(expected: Metadata, key: Key) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer, key).unwrap();
    buffer.rewind().unwrap();
    let actual = Metadata::read(&mut buffer, key).unwrap();

    assert_eq!(expected, actual);
  }
}
