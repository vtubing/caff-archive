use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum ColorType {
  #[default]
  Unknown,
  Png,
  None = i8::MAX as isize,
}

impl From<i8> for ColorType {
  fn from(value: i8) -> Self {
    match value {
      1 => Self::Png,
      i8::MAX => Self::None,
      _ => Self::Unknown,
    }
  }
}

impl From<ColorType> for i8 {
  fn from(value: ColorType) -> Self {
    match value {
      ColorType::Unknown => 0,
      ColorType::Png => 1,
      ColorType::None => i8::MAX,
    }
  }
}

impl ColorType {
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
  fn roundtrip(expected: ColorType) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = ColorType::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }

  #[proptest]
  fn from_i8(value: i8) {
    ColorType::from(value);
  }

  #[proptest]
  fn into_i8(value: ColorType) {
    i8::from(value);
  }
}
