use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use errors::AppError;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};

mod errors;

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
async fn main() -> Result<(), AppError>  {
    // Connexion à Redis
    let redis_client = match connect_to_redis() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("❌ Impossible de se connecter à Redis : {:?}", err);
            return Err(err);
        }
    };

    // Canal pour envoyer des mises à jour en temps réel
    let (tx, _rx) = broadcast::channel(10);

    let state = AppState {
        redis_client: Arc::new(redis_client),
        broadcaster: Arc::new(Mutex::new(tx)),
    };

    // Routes de l'API
    let app = Router::new()
        .route("/status", post(update_status))
        .route("/status/{user_id}", get(get_status))
        .route("/ws", get(|ws: WebSocketUpgrade, state: State<AppState>| async move {
            websocket_handler(ws, state).await
        }))
        .with_state(state);

    // Démarrage du serveur
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serveur démarré sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn connect_to_redis() -> Result<Client, AppError> {
    let client = Client::open("redis://127.0.0.1/")?;
    Ok(client)
}

// 🔵 Mise à jour du statut
async fn update_status(
    State(state): State<AppState>,
    Json(payload): Json<StatusUpdate>,
) -> impl IntoResponse {
    let mut con = state.redis_client.get_multiplexed_async_connection().await.unwrap();
    
    // Stockage en Redis (avec expiration de 24h)
    let _: () = con.set_ex(payload.user_id.clone(), payload.status.clone(), 86400)
        .await
        .unwrap();

    // Envoi d'un message aux WebSockets
    let message = serde_json::to_string(&payload).unwrap();
    let broadcaster = state.broadcaster.lock().await;
    let _ = broadcaster.send(message);

    Json(payload)
}

// 🟢 Récupération du statut d’un utilisateur
async fn get_status(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut con = state.redis_client.get_multiplexed_async_connection().await.unwrap();
    let status: Option<String> = con.get(&user_id).await.unwrap();
    Json(status.unwrap_or("déconnecté".to_string()))
}

// 🔥 WebSocket pour les mises à jour en temps réel
async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: AppState) {
    let mut rx = state.broadcaster.lock().await.subscribe();

    while let Ok(message) = rx.recv().await {
        if socket.send(Message::Text(message.into())).await.is_err() {
            break;
        }
    }
}

#[cfg(test)]
mod mock_tests {
    use super::*;

    #[cfg(feature = "mock")]
    fn mock_connect_to_redis() -> Result<Client, AppError> {
        Err(AppError::RedisConnection(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "Mock: Redis non disponible",
        ))))
    }

    #[cfg(feature = "mock")]
    #[tokio::test]
    async fn test_mock_connect_to_redis_failure() {
        let result = mock_connect_to_redis();
        assert!(result.is_err(), "La connexion Redis aurait dû échouer dans le mock.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test] // Test asynchrone
    async fn test_connect_to_redis_success() {
        let result = connect_to_redis();
        assert!(result.is_ok(), "La connexion à Redis aurait dû réussir.");
    }
}
