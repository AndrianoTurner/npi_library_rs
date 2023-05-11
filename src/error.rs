

#[derive(thiserror::Error,Debug,PartialEq)]
pub enum Error{
    #[error("Database error")]
    DatabaseError,
    #[error("Converter error")]
    ConverterError,
    #[error("Track Manager error")]
    TrackError,
    #[error("File Utils error")]
    FileUtilsError,
    #[error("Doc Manager Error")]
    DocManagerError,
}

impl From<reqwest::Error> for Error{
    fn from(_: reqwest::Error) -> Self {
        Self::ConverterError
    }
}

impl From<sqlx::Error> for Error{
    fn from(_: sqlx::Error) -> Self {
        Self::DatabaseError
    }
}
