use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug)]
pub enum DatapineErr {
    EmptyCommand,
    UnknownCommand,
    InvalidInsert,
    InvalidGet,
    InvalidRemove,
    InvalidKNN,
    InvalidCosine,
    InvalidDump,
}


impl Error for DatapineErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatapineErr::EmptyCommand => None,
            DatapineErr::UnknownCommand => None,
            DatapineErr::InvalidInsert => None,
            DatapineErr::InvalidGet => None,
            DatapineErr::InvalidRemove => None,
            DatapineErr::InvalidKNN => None,
            DatapineErr::InvalidCosine => None,
            DatapineErr::InvalidDump => None,
        }
    }
}

impl fmt::Display for DatapineErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatapineErr::EmptyCommand => write!(f, "Empty command"),
            DatapineErr::UnknownCommand => write!(f, "Unknown command"),
            DatapineErr::InvalidInsert => write!(f, "Invalid insert"),
            DatapineErr::InvalidGet => write!(f, "Invalid get"),
            DatapineErr::InvalidRemove => write!(f, "Invalid remove"),
            DatapineErr::InvalidKNN => write!(f, "Invalid KNN"),
            DatapineErr::InvalidCosine => write!(f, "Invalid cosine similarity"),
            DatapineErr::InvalidDump => write!(f, "Invalid dump"),
        }
    }
}

impl From<DatapineErr> for io::Error {
    fn from(err: DatapineErr) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}