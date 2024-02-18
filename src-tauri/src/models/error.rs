use thiserror::Error; 
use serde::Serialize; 

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("IO Error: {0}")] 
    IOError(#[from] std::io::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
            
            serializer.serialize_str(self.to_string().as_ref())
    }
}