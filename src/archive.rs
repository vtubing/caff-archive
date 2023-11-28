use crate::prelude::*;

mod body;
mod entry;
mod header;
mod key;

pub use body::Body;
pub use entry::Metadata;
pub use header::Header;
pub use key::Key;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Archive {
  pub header: Header,
  pub body: Body,
}

impl Archive {
  pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
    reader.trace(|reader| {
      let header = Header::read(reader)?;
      let body = Body::read(reader, header.key)?;

      let value = Self { header, body };

      Ok(value)
    })
  }

  pub fn write<W: Write + Seek>(&self, writer: &mut W) -> Result<()> {
    writer.trace(|writer| {
      let Self { header, body } = self;

      header.write(writer)?;
      body.write(writer, header.key)?;

      Ok(())
    })
  }
}
