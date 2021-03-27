// Provides custom errors for good UX
use std::fmt;

pub enum Error {
    BadInput,
    BadHost,
    BadOwner,
    BadRepo,
    ClientTimeout,
    ClientWithStatus(reqwest::StatusCode),
    ClientOther,
    Io(std::io::ErrorKind),
    StripPrefixError,
    DesinationNotEmpty,
    NoMatchingFiles,
    CloneError(String),
}

pub type AppResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadInput => write!(f, "Invalid source specified."),
            Error::BadHost => write!(f, "Only GitHub is supported."),
            Error::BadOwner => write!(f, "No owner detected in source."),
            Error::BadRepo => write!(f, "No repo detected in source."),
            Error::DesinationNotEmpty => write!(f, "Destination not empty."),
            Error::Io(k) => write!(f, "IO error: {:?}", k),
            Error::ClientTimeout => write!(f, "Timeout during request"),
            Error::ClientWithStatus(status) => write!(f, "Got status code: {}.", status),
            Error::ClientOther => write!(f, "Unknown client error."),
            Error::StripPrefixError => write!(f, "Strip prefix error writing files."),
            Error::NoMatchingFiles => write!(f, "No matching files found for filter."),
            Error::CloneError(err) => write!(f, "Error cloning repo.\n{}", err),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(err: std::io::Error) -> Error {
        Error::Io(err.kind())
    }
}

impl From<reqwest::Error> for Error {
    #[inline]
    fn from(err: reqwest::Error) -> Error {
        if err.is_timeout() {
            return Error::ClientTimeout;
        }
        if let Some(s) = err.status() {
            return Error::ClientWithStatus(s);
        }
        Error::ClientOther
    }
}

impl From<std::path::StripPrefixError> for Error {
    #[inline]
    fn from(_: std::path::StripPrefixError) -> Error {
        Error::StripPrefixError
    }
}
