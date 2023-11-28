use crate::prelude::*;

mod color_type;
mod image_type;

use color_type::ColorType;
use image_type::ImageType;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PreviewImage {
  pub image_type: ImageType,
  pub color_type: ColorType,
  unknown: Padding<2>,
  pub width: u16,
  pub height: u16,
  padding: Padding<8>,
}

impl PreviewImage {
  pub fn is_empty(&self) -> bool {
    self.image_type == ImageType::None && self.color_type == ColorType::None && self.unknown.is_empty() && self.width == 0 && self.height == 0 && self.padding.is_empty()
  }

  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let image_type = ImageType::read(reader)?;
      let color_type = ColorType::read(reader)?;
      let unknown = Padding::read(reader)?;
      let width = reader.read_u16::<BigEndian>()?;
      let height = reader.read_u16::<BigEndian>()?;
      let padding = Padding::read(reader)?;

      let value = Self {
        image_type,
        color_type,
        unknown,
        width,
        height,
        padding,
      };

      #[cfg(feature = "discovery")]
      if !value.is_empty() {
        log::debug!("read value={value:?}");
      }

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      let Self {
        image_type,
        color_type,
        unknown,
        width,
        height,
        padding,
      } = self;

      image_type.write(writer)?;
      color_type.write(writer)?;
      unknown.write(writer)?;
      writer.write_u16::<BigEndian>(*width)?;
      writer.write_u16::<BigEndian>(*height)?;
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
  fn roundtrip(expected: PreviewImage) {
    let mut buffer = Cursor::new(Vec::new());

    expected.write(&mut buffer).unwrap();
    buffer.rewind().unwrap();
    let actual = PreviewImage::read(&mut buffer).unwrap();

    assert_eq!(expected, actual);
  }
}
