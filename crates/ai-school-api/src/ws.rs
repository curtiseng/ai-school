use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tracing::{debug, info, warn};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/ws/simulation", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("WebSocket client connected");

    let runner = state.runner.read().await;
    let mut rx = runner.subscribe();
    drop(runner);

    loop {
        tokio::select! {
            update = rx.recv() => {
                match update {
                    Ok(update) => {
                        match serde_json::to_string(&update) {
                            Ok(json) => {
                                if socket.send(Message::Text(json.into())).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                warn!(error = %e, "Failed to serialize SimulationUpdate");
                            }
                        }
                    }
                    Err(_) => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Text(text))) => {
                        debug!(msg = ?text, "WebSocket message received");
                    }
                    _ => {}
                }
            }
        }
    }

    info!("WebSocket client disconnected");
}
