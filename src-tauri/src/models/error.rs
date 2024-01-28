use thiserror::Error; 
use serde::Serialize; 

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error)
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
            
            serializer.serialize_str(self.to_string().as_ref())
    }
}