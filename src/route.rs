use axum::{
    Json,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use redis::AsyncCommands;
use tracing::{info, error, warn};

use crate::{AppState, StatusUpdate};

// 🔵 Mise à jour du statut
pub async fn update_status(
    State(state): State<AppState>,
    Json(payload): Json<StatusUpdate>,
) -> impl IntoResponse {
    info!("🔵 Mise à jour du statut pour l'utilisateur: {}", payload.user_id);

    let mut con = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    // Stockage en Redis (avec expiration de 24h)
    let _: () = con
        .set_ex(payload.user_id.clone(), payload.status.clone(), 86400)
        .await
        .unwrap();

    // Envoi d'un message aux WebSockets
    let message = serde_json::to_string(&payload).unwrap();
    let broadcaster = state.broadcaster.lock().await;
    let _ = broadcaster.send(message);

    Json(payload)
}

// 🟢 Récupération du statut d’un utilisateur
pub async fn get_status(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    info!("🟢 Récupération du statut de l'utilisateur: {}", user_id);

    let mut con = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let status: Option<String> = con.get(&user_id).await.unwrap();
    Json(status.unwrap_or("déconnecté".to_string()))
}

// 🔥 WebSocket pour les mises à jour en temps réel
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

pub async fn handle_ws(mut socket: WebSocket, state: AppState) {
    info!("📡 Nouvelle connexion WebSocket établie.");

    let mut rx = state.broadcaster.lock().await.subscribe();

    while let Ok(message) = rx.recv().await {
        if socket.send(Message::Text(message.into())).await.is_err() {
            error!("❌ Erreur lors de l'envoi du message WebSocket.");
            break;
        }
    }
    warn!("⚠️ Connexion WebSocket fermée.");

}
