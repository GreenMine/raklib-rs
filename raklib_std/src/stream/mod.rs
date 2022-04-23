pub(crate) mod binary_stream;
mod bs_adapter;
mod end_of_stream;

pub use binary_stream::BinaryStream;
pub use bs_adapter::BSAdapter;
pub use end_of_stream::EndOfStream;

pub type Result<T> = std::result::Result<T, EndOfStream>;
