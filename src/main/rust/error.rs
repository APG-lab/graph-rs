
use std::io;
use std::num;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum GraphError
{
    #[error("Algorithm error: {0}")]
    AlgorithmError (String),

    #[error("Conversion error: {0}")]
    ConversionError (String),

    #[error("Edge error: {0}")]
    EdgeError (String),

    #[error("IO error: {0}")]
    IOError (String),

    #[error("Vertex error: {0}")]
    VertexError (String)
}

impl From<io::Error> for GraphError
{
    fn from (err: io::Error)
        -> GraphError
    {
        GraphError::IOError (err.to_string ())
    }
}

impl From<num::TryFromIntError> for GraphError
{
    fn from (err: num::TryFromIntError)
        -> GraphError
    {
        GraphError::ConversionError (err.to_string ())
    }
}

