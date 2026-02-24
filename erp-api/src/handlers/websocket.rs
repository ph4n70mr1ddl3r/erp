use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Query,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use crate::db::AppState;

pub type WebSocketManager = Arc<WebSocketManagerInner>;

pub struct WebSocketManagerInner {
    clients: RwLock<HashMap<Uuid, Vec<Uuid>>>,
    tx: broadcast::Sender<WebSocketMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event_type: String,
    pub payload: serde_json::Value,
    pub target_users: Option<Vec<Uuid>>,
}

impl WebSocketManagerInner {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            clients: RwLock::new(HashMap::new()),
            tx,
        }
    }

    pub async fn add_client(&self, user_id: Uuid, client_id: Uuid) {
        let mut clients = self.clients.write().await;
        clients.entry(user_id).or_insert_with(Vec::new).push(client_id);
    }

    pub async fn remove_client(&self, user_id: Uuid, client_id: Uuid) {
        let mut clients = self.clients.write().await;
        if let Some(user_clients) = clients.get_mut(&user_id) {
            user_clients.retain(|id| *id != client_id);
            if user_clients.is_empty() {
                clients.remove(&user_id);
            }
        }
    }

    pub async fn get_client_count(&self, user_id: &Uuid) -> usize {
        let clients = self.clients.read().await;
        clients.get(user_id).map(|c| c.len()).unwrap_or(0)
    }

    pub async fn get_total_client_count(&self) -> usize {
        let clients = self.clients.read().await;
        clients.values().map(|c| c.len()).sum()
    }

    pub fn broadcast(&self, message: WebSocketMessage) {
        let _ = self.tx.send(message);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.tx.subscribe()
    }
}

impl Default for WebSocketManagerInner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    pub token: Option<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WebSocketQuery>,
) -> Response {
    let user_id = if let Some(token) = query.token {
        let svc = erp_auth::AuthService::new();
        match svc.validate_token(&token) {
            Ok(data) => Uuid::parse_str(&data.claims.sub).ok(),
            Err(_) => None,
        }
    } else {
        None
    };

    ws.on_upgrade(move |socket| handle_websocket(socket, state.ws_manager.clone(), user_id))
}

async fn handle_websocket(socket: WebSocket, ws_manager: WebSocketManager, user_id: Option<Uuid>) {
    let user_id = match user_id {
        Some(id) => id,
        None => {
            let _ = socket.close().await;
            return;
        }
    };

    let client_id = Uuid::new_v4();
    ws_manager.add_client(user_id, client_id).await;

    let (mut sender, mut receiver) = socket.split();
    let mut rx = ws_manager.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let should_send = match &msg.target_users {
                Some(users) => users.contains(&user_id),
                None => true,
            };

            if should_send {
                let json = serde_json::to_string(&msg).unwrap_or_default();
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(Message::Close(_)) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    ws_manager.remove_client(user_id, client_id).await;
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationEvent {
    pub id: Uuid,
    pub event_type: String,
    pub title: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationEvent {
    pub fn new(event_type: &str, title: &str, message: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            title: title.to_string(),
            message: message.to_string(),
            data: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn to_websocket_message(self, target_users: Vec<Uuid>) -> WebSocketMessage {
        WebSocketMessage {
            event_type: self.event_type.clone(),
            payload: serde_json::to_value(self).unwrap_or(serde_json::json!({})),
            target_users: Some(target_users),
        }
    }
}

pub fn notify_users(ws_manager: &WebSocketManager, user_ids: Vec<Uuid>, event: NotificationEvent) {
    let msg = event.to_websocket_message(user_ids);
    ws_manager.broadcast(msg);
}

pub fn notify_all(ws_manager: &WebSocketManager, event: NotificationEvent) {
    let msg = WebSocketMessage {
        event_type: event.event_type.clone(),
        payload: serde_json::to_value(event).unwrap_or(serde_json::json!({})),
        target_users: None,
    };
    ws_manager.broadcast(msg);
}

pub async fn get_ws_stats(
    State(state): State<AppState>,
) -> crate::error::ApiResult<axum::Json<serde_json::Value>> {
    let total = state.ws_manager.get_total_client_count().await;
    Ok(axum::Json(serde_json::json!({
        "total_connections": total
    })))
}
