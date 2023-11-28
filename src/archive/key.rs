use crate::prelude::*;
use std::ops::{BitXor, BitXorAssign};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Key(u32);

impl std::fmt::Debug for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Key({:#010X?})", self.0))
  }
}

impl From<u32> for Key {
  fn from(value: u32) -> Self {
    Self(value)
  }
}

impl Default for Key {
  fn default() -> Self {
    Self::from(0)
  }
}

impl From<Key> for u32 {
  fn from(key: Key) -> Self {
    key.0
  }
}

impl From<Key> for u16 {
  fn from(key: Key) -> Self {
    (key.0 & u32::from(u16::MAX)) as u16
  }
}

impl From<Key> for u8 {
  fn from(key: Key) -> Self {
    (key.0 & u32::from(u8::MAX)) as u8
  }
}

impl<T> BitXor<T> for Key
where
  T: From<Key> + BitXor<T, Output = T>,
{
  type Output = T;
  fn bitxor(self, value: T) -> Self::Output {
    T::from(self) ^ value
  }
}

impl BitXor<Key> for u32 {
  type Output = u32;
  fn bitxor(self, key: Key) -> Self::Output {
    key ^ self
  }
}

impl BitXor<Key> for u16 {
  type Output = u16;
  fn bitxor(self, key: Key) -> Self::Output {
    key ^ self
  }
}

impl BitXor<Key> for u8 {
  type Output = u8;
  fn bitxor(self, key: Key) -> Self::Output {
    key ^ self
  }
}

impl BitXorAssign<Key> for u32 {
  fn bitxor_assign(&mut self, key: Key) {
    self.bitxor_assign(Self::from(key))
  }
}

impl BitXorAssign<Key> for u8 {
  fn bitxor_assign(&mut self, key: Key) {
    self.bitxor_assign(Self::from(key))
  }
}

impl Key {
  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let value = reader.read_u32::<BigEndian>()?.into();
      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      writer.write_u32::<BigEndian>(self.into())?;
      Ok(())
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_strategy::proptest;

  #[proptest]
  fn roundtrip(expected: Key) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = Key::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }

  #[proptest]
  fn default(value: u32) {
    let expected = value;
    let actual = Key::default() ^ value;

    assert_eq!(expected, actual)
  }

  mod bitxor {
    use super::*;

    #[proptest]
    fn u32(key: Key, value: u32) {
      let expected = value ^ u32::from(key);
      let actual = value ^ key;

      assert_eq!(expected, actual);
    }

    #[proptest]
    fn u16(key: Key, value: u16) {
      let expected = value ^ u16::from(key);
      let actual = value ^ key;

      assert_eq!(expected, actual);
    }

    #[proptest]
    fn u8(key: Key, value: u8) {
      let expected = value ^ u8::from(key);
      let actual = value ^ key;

      assert_eq!(expected, actual);
    }
  }
}
