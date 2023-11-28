use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum ImageType {
  #[default]
  Unknown,
  Argb,
  Rgb,
  None = i8::MAX as isize,
}

impl From<i8> for ImageType {
  fn from(value: i8) -> Self {
    match value {
      1 => Self::Argb,
      2 => Self::Rgb,
      i8::MAX => Self::None,
      _ => Self::Unknown,
    }
  }
}

impl From<ImageType> for i8 {
  fn from(value: ImageType) -> Self {
    match value {
      ImageType::Unknown => 0,
      ImageType::Argb => 1,
      ImageType::Rgb => 2,
      ImageType::None => i8::MAX,
    }
  }
}

impl ImageType {
  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let raw = reader.read_i8()?;
      let value = raw.into();

      #[cfg(feature = "discovery")]
      if value == Self::Unknown {
        log::debug!("read raw={raw}, value={value:?}");
      }

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      writer.write_i8((*self).into())?;
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip(expected: ImageType) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = ImageType::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }

  #[proptest]
  fn from_i8(value: i8) {
    ImageType::from(value);
  }

  #[proptest]
  fn into_i8(value: ImageType) {
    i8::from(value);
  }
}
