use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

use crate::db;
use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(job_uuid): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, job_uuid, state))
}

async fn handle_socket(socket: WebSocket, job_uuid: String, state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Send existing logs
    if let Ok(Some(job)) = db::get_build_job_by_uuid(&state.pool, &job_uuid).await {
        if !job.log_output.is_empty() {
            let _ = sender.send(Message::Text(job.log_output.clone().into())).await;
        }
    }

    // Subscribe to live logs
    let mut log_rx = state.build_queue.subscribe_logs(&job_uuid).await;

    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg { break; }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        if let Some(ref mut rx) = log_rx {
            while let Some(line) = rx.recv().await {
                if sender.send(Message::Text(line.into())).await.is_err() { break; }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
