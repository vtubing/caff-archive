use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Padding<const N: usize>([u8; N]);

impl<const N: usize> From<[u8; N]> for Padding<N> {
  fn from(value: [u8; N]) -> Self {
    Self(value)
  }
}

impl<const N: usize> From<Padding<N>> for [u8; N] {
  fn from(value: Padding<N>) -> Self {
    value.into_inner()
  }
}

impl<const N: usize> Default for Padding<N> {
  fn default() -> Self {
    Self([0u8; N])
  }
}

impl<const N: usize> std::fmt::Debug for Padding<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_empty() {
      f.write_fmt(format_args!("Padding({} bytes) - EMPTY", N))
    } else if self.is_all_u8_max() {
      f.write_fmt(format_args!("Padding({} bytes) - ALL MAXES", N))
    } else {
      f.write_fmt(format_args!("Padding({} bytes) - NOT EMPTY - {:02X?}", N, self.0))
    }
  }
}

impl<const N: usize> Padding<N> {
  pub fn is_empty(&self) -> bool {
    self.0.iter().all(|value| value == &0)
  }

  pub fn is_all_u8_max(&self) -> bool {
    self.0.iter().all(|value| value == &u8::MAX)
  }

  pub fn into_inner(self) -> [u8; N] {
    self.0
  }

  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let mut bytes = [0u8; N];
      reader.read_exact(&mut bytes)?;

      let value = Self(bytes);

      #[cfg(feature = "logging")]
      if !value.is_empty() {
        if value.is_all_u8_max() {
          log::debug!("read value={value:?}")
        } else {
          log::warn!("read value={value:?}")
        }
      }

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      writer.write_all(&self.0)?;
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip(expected: Padding<4>) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = Padding::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }
}
