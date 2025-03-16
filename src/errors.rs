use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Erreur de connexion à Redis : {0}")]
    RedisConnection(#[from] RedisError),
    // #[error("Erreur interne du serveur")]
    // InternalServerError,
}
