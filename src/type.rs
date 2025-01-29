use std::{error, result};

pub type Error = Box<dyn error::Error>;
pub type Result<T = (), E = Error> = result::Result<T, E>;
