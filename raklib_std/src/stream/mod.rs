pub use adapter::Adapter;
pub use binary_stream::BinaryStream;
pub use end_of_stream::EndOfStream;

mod adapter;
pub(crate) mod binary_stream;
mod end_of_stream;

pub type Result<T> = std::result::Result<T, EndOfStream>;
