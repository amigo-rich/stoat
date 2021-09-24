use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    EmptyCategory,
    EmptyHash,
    EmptyLocationName,
    EmptySchema,
    InvalidLatitude(f32),
    InvalidLongitude(f32),
    IO(std::io::Error),
    NotADir(PathBuf),
    NotAFile(PathBuf),
    PathConversion,
    Rusqlite(rusqlite::Error),
    Send(std::sync::mpsc::SendError<PathBuf>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    Error::EmptyCategory => write!(f, "The provided category is empty"),
	    Error::EmptyHash => write!(f, "The provided hash is empty"),
	    Error::EmptyLocationName => write!(f, "The provided location name is empty"),
	    Error::EmptySchema => write!(f, "The provided schema is empty"),
	    Error::InvalidLatitude(v) => {
		write!(f, "The provided latitude ('{}') is invalid", v)
	    },
	    Error::InvalidLongitude(v) => {
		write!(f, "The provided longitude ('{}') is invalid", v)
	    },
	    Error::IO(e) => write!(f, "An IO Error occurred: '{}'", e),
	    Error::NotADir(pb) => {
		let str_or_unknown = pb.to_str().unwrap_or("unknown");
		write!(f, "The provided path ('{}') is not a directory", str_or_unknown)
	    },
	    Error::NotAFile(pb) => {
		let str_or_unknown = pb.to_str().unwrap_or("unknown");
		write!(f, "The provided path ('{}') is not a file", str_or_unknown)
	    },
	    Error::PathConversion => write!(f, "Converting a path to a str failed"),
	    Error::Rusqlite(e) => write!(f, "Rusqlite library error: '{}'", e),
	    Error::Send(e) => write!(f, "A Send error occurred: '{}'", e),
	}
    }
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
	Error::IO(ioe)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(re: rusqlite::Error) -> Self {
	Error::Rusqlite(re)
    }
}

impl From<std::sync::mpsc::SendError<PathBuf>> for Error {
    fn from(se: std::sync::mpsc::SendError<PathBuf>) -> Self {
	Error::Send(se)
    }
}
