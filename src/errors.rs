use redis::RedisError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Erreur de connexion à Redis : {0}")]
    RedisConnection(#[from] RedisError),

    #[error("Erreur de configuration : {0}")]
    InvalidConfig(String),

    #[error("Erreur d'entrée/sortie : {0}")]
    Io(#[from] io::Error),
}
