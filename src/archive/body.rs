use super::Metadata;
use crate::prelude::*;

#[derive(derivative::Derivative, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derivative(Debug)]
pub struct Body {
  pub metadata: Vec<Metadata>,
  #[derivative(Debug = "ignore")]
  pub data: Vec<Vec<u8>>,
  pub trailing: Vec<u8>,
}

impl Body {
  pub fn read<R: Read + Seek>(reader: &mut R, key: Key) -> Result<Self> {
    let total_file_entries = reader.read_encrypted_u32::<BigEndian>(key)?;

    let mut metadata = Vec::new();

    for _ in 0..total_file_entries {
      let file_info = Metadata::read(reader, key)?;
      metadata.push(file_info);
    }

    let mut data = Vec::new();
    for metadata in metadata.iter() {
      let mut bytes = Vec::with_capacity(metadata.file_size.try_into()?);

      if metadata.is_obfuscated {
        for _ in 0..metadata.file_size {
          bytes.push(reader.read_encrypted_u8(key)?)
        }
      } else {
        for _ in 0..metadata.file_size {
          bytes.push(reader.read_u8()?)
        }
      }

      data.push(bytes);
    }

    let mut trailing = Vec::new();
    reader.read_to_end(&mut trailing)?;

    Ok(Self { metadata, data, trailing })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W, key: Key) -> Result<()> {
    for metadata in &self.metadata {
      metadata.write(writer, key)?;
    }

    for data in &self.data {
      for byte in data {
        writer.write_encrypted_u8(*byte, key)?;
      }
    }

    writer.write_all(&self.trailing)?;

    Ok(())
  }
}
