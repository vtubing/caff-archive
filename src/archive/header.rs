use crate::prelude::*;

mod magic;
mod preview_image;
mod version;

use crate::Padding;
use magic::Magic;
use preview_image::PreviewImage;
use version::Version;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Header {
  #[cfg_attr(test, map(|magic: Magic| Magic::default()))]
  pub archive_magic: Magic,
  pub archive_version: Version,
  pub format_identifier: [u8; 4],
  pub format_version: Version,
  pub key: Key,
  unknown: Padding<8>,
  pub preview_image: PreviewImage,
  padding: Padding<12>,
}

impl Header {
  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let archive_magic = Magic::read(reader)?;
      let archive_version = Version::read(reader)?;
      let mut format_identifier = [0u8; 4];
      reader.read_exact(&mut format_identifier)?;
      let format_version = Version::read(reader)?;
      let key = Key::read(reader)?;
      let unknown = Padding::read(reader)?;
      let preview_image = PreviewImage::read(reader)?;
      let padding = Padding::read(reader)?;

      let value = Self {
        archive_magic,
        archive_version,
        format_identifier,
        format_version,
        key,
        unknown,
        preview_image,
        padding,
      };

      #[cfg(feature = "logging")]
      log::debug!("read value={value:?}");

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      let Self {
        archive_magic,
        archive_version,
        format_identifier,
        format_version,
        key: obfuscate_key,
        unknown,
        preview_image,
        padding,
      } = self;

      archive_magic.write(writer)?;
      archive_version.write(writer)?;
      writer.write_all(format_identifier)?;
      format_version.write(writer)?;
      obfuscate_key.write(writer)?;
      unknown.write(writer)?;
      preview_image.write(writer)?;
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
  fn roundtrip(expected: Header) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = Header::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }
}
