use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Magic {
  bytes: [u8; 4],
}

impl Default for Magic {
  fn default() -> Self {
    Self { bytes: Self::MAGIC }
  }
}

impl std::fmt::Debug for Magic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_valid() {
      f.write_fmt(format_args!("Magic({:?}) - VALID", self.bytes))
    } else {
      f.write_fmt(format_args!("Magic({:?}) - INVALID", self.bytes))
    }
  }
}

impl Magic {
  const MAGIC: [u8; 4] = [0x43, 0x41, 0x46, 0x46];

  pub fn is_valid(&self) -> bool {
    self.bytes == Self::MAGIC
  }

  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let mut bytes = [0u8; 4];
      reader.read_exact(&mut bytes)?;

      let value = Self { bytes };

      #[cfg(feature = "logging")]
      if !value.is_valid() {
        log::error!("read value={value:?}");
        return Err(Error::BadMagic);
      }
      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      writer.write_all(&self.bytes)?;
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip() {
    let expected = Magic::default();
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = Magic::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }
}
