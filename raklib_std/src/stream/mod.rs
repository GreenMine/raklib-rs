pub use adapter::Adapter;
pub use binary_stream::BinaryStream;

mod adapter;
pub(crate) mod binary_stream;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("trying to read {try_to_read} bytes, left {actual_left}")]
    EndOfStream {
        try_to_read: usize,
        actual_left: usize,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
