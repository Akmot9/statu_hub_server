use axum::{
    Router,
    extract::{State, ws::WebSocketUpgrade},
    routing::{get, post},
};
use dotenvy::dotenv;
use errors::AppError;
use redis::Client;
use route::{get_status, update_status, websocket_handler};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast},
};

mod errors;
mod route;

// Structure pour stocker l’état de l’application
#[derive(Clone)]
struct AppState {
    redis_client: Arc<Client>,
    broadcaster: Arc<Mutex<broadcast::Sender<String>>>,
}

// Structure pour la mise à jour du statut
#[derive(Debug, Deserialize, Serialize)]
struct StatusUpdate {
    user_id: String,
    status: String, // "connecté" ou "déconnecté"
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Charger le fichier .env s'il existe
    dotenv().ok();

    // Récupération des variables d'environnement avec gestion des erreurs
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let broadcast_buffer_size: usize = env::var("BROADCAST_BUFFER_SIZE")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .map_err(|_| {
            AppError::InvalidConfig("BROADCAST_BUFFER_SIZE doit être un nombre".to_string())
        })?;
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .map_err(|_| AppError::InvalidConfig("SERVER_PORT doit être un nombre".to_string()))?;

    // Connexion à Redis
    let redis_client = connect_to_redis(&redis_url)?;

    // Canal pour envoyer des mises à jour en temps réel
    let (tx, _rx) = broadcast::channel(broadcast_buffer_size);

    let state = AppState {
        redis_client: Arc::new(redis_client),
        broadcaster: Arc::new(Mutex::new(tx)),
    };

    // Routes de l'API
    let app = Router::new()
        .route("/status", post(update_status))
        .route("/status/{user_id}", get(get_status))
        .route(
            "/ws",
            get(|ws: WebSocketUpgrade, state: State<AppState>| async move {
                websocket_handler(ws, state).await
            }),
        )
        .with_state(state);

    // Démarrage du serveur
    let listener = TcpListener::bind(format!("0.0.0.0:{}", server_port)).await?;
    println!("🚀 Serveur démarré sur http://127.0.0.1:{}", server_port);
    axum::serve(listener, app).await?;

    Ok(())
}

fn connect_to_redis(redis_url: &str) -> Result<Client, AppError> {
    Client::open(redis_url).map_err(AppError::RedisConnection)
}

#[cfg(test)]
mod mock_tests {
    use crate::AppError;
    use crate::Client;

    #[cfg(feature = "mock")]
    fn mock_connect_to_redis() -> Result<redis::Client, crate::errors::AppError> {
        Err(crate::errors::AppError::RedisConnection(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "Mock: Redis non disponible",
        ))))
    }

    #[cfg(feature = "mock")]
    #[tokio::test]
    async fn test_mock_connect_to_redis_failure() {
        let result = mock_connect_to_redis();
        assert!(
            result.is_err(),
            "La connexion Redis aurait dû échouer dans le mock."
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] // Test asynchrone
    async fn test_connect_to_redis_success() {
        let redis_url = "redis://127.0.0.1/";
        let result = connect_to_redis(redis_url);
        assert!(result.is_ok(), "La connexion à Redis aurait dû réussir.");
    }
}