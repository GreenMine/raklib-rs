#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Decode(#[from] raklib_std::stream::Error),
    #[error(transparent)]
    Net(#[from] crate::net::Error),
}
