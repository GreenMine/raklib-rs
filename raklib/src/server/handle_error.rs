use raklib_std::stream::EndOfStream;
use std::io::Error;

pub enum HandleError {
    Decode(EndOfStream),
    Io(std::io::Error),
}

impl From<EndOfStream> for HandleError {
    fn from(eos: EndOfStream) -> Self {
        HandleError::Decode(eos)
    }
}

impl From<std::io::Error> for HandleError {
    fn from(io: Error) -> Self {
        HandleError::Io(io)
    }
}
