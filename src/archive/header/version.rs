use crate::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

impl From<[u8; 3]> for Version {
  fn from([major, minor, patch]: [u8; 3]) -> Self {
    Self { major, minor, patch }
  }
}

impl From<Version> for [u8; 3] {
  fn from(Version { major, minor, patch }: Version) -> Self {
    [major, minor, patch]
  }
}

impl std::fmt::Debug for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
  }
}

impl Version {
  pub fn new(major: u8, minor: u8, patch: u8) -> Self {
    [major, minor, patch].into()
  }

  pub fn is_empty(&self) -> bool {
    self.major == 0 && self.minor == 0 && self.patch == 0
  }

  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let major = reader.read_u8()?;
      let minor = reader.read_u8()?;
      let patch = reader.read_u8()?;

      let value = Self { major, minor, patch };

      #[cfg(feature = "discovery")]
      if !value.is_empty() {
        log::info!("read value={value:?}");
      }

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      let Self { major, minor, patch } = self;

      writer.write_u8(major)?;
      writer.write_u8(minor)?;
      writer.write_u8(patch)?;

      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip(expected: Version) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = Version::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }

  #[proptest]
  fn from_bytes(value: [u8; 3]) {
    Version::from(value);
  }

  #[proptest]
  fn into_bytes(value: Version) {
    <[u8; 3]>::from(value);
  }
}
