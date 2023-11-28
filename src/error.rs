#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("bad magic in CAFF header")]
  BadMagic,
  #[error("empty filename in entry metadata")]
  EmptyFilename,
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  TryFromInt(#[from] std::num::TryFromIntError),
  #[error(transparent)]
  Utf8(#[from] std::string::FromUtf8Error),
  #[error("varint support not implemented")]
  VarIntSupportNotImplemented,
}
