pub mod api_error;

use std::{fmt::Display, num::ParseIntError, string::FromUtf8Error};

use deadpool_postgres::PoolError;
use diesel::result::Error as DbError;
use strum::ParseError;
use tokio_diesel::AsyncError;

#[derive(Debug)]
pub enum Error {
    NotFound,
    ParseInt(ParseIntError),
    Utf8Error(FromUtf8Error),
    Utf8StrError(std::str::Utf8Error),
    ParseError,
    Undefined,
    DbError(DbError), // old error, will be removed
    IoError(std::io::Error),
    Checkout(r2d2::Error),
    PoolError(PoolError),
    Postgres(tokio_postgres::Error), // new db error
}

/// Map a diesel not-found error to Error::NotFound
/// Other diesel errors will be handled noramlly
pub fn map_notfound_async(err: AsyncError) -> Error {
    match err {
        AsyncError::Checkout(c) => Error::Checkout(c),
        AsyncError::Error(e) => map_notfound(e),
    }
}

/// Map a diesel not-found error to Error::NotFound
/// Other diesel errors will be handled noramlly
pub fn map_notfound(err: DbError) -> Error {
    match err {
        DbError::NotFound => Error::NotFound,
        _ => err.into(),
    }
}

pub fn db_to_option<T>(res: Result<T, Error>) -> Result<Option<T>, Error> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(err) => match err {
            Error::NotFound => Ok(None),
            Error::DbError(e) => match e {
                DbError::NotFound => Ok(None),
                _ => Err(e.into()),
            },
            _ => Err(err),
        },
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::Postgres(err)
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Self::PoolError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        Self::DbError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::VariantNotFound => Self::ParseError,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<AsyncError> for Error {
    fn from(err: AsyncError) -> Self {
        match err {
            AsyncError::Checkout(co) => Self::Checkout(co),
            AsyncError::Error(err) => Self::DbError(err),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Utf8StrError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
