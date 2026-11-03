use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const TEST_LOCAL_SERVER: &str = "127.0.0.1";

pub async fn start_test_server(
    responder: Arc<dyn Fn(u32) -> (u16, Vec<u8>) + Send + Sync>,
) -> (String, Arc<AtomicU32>, Arc<Mutex<String>>) {
    let listener = tokio::net::TcpListener::bind((TEST_LOCAL_SERVER, 0))
        .await
        .expect("failed to bind local test server");

    let address = listener.local_addr().expect("failed to launch test server");
    let hits = Arc::new(AtomicU32::new(0));
    let head = Arc::new(Mutex::new(String::new()));
    let hits_task = hits.clone();
    let head_task = head.clone();

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            let hits = hits_task.clone();
            let head = head_task.clone();
            let responder = responder.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let read = socket.read(&mut buf).await.unwrap_or(0);
                {
                    let mut guard = head.lock().expect("head lock");
                    if guard.is_empty() {
                        *guard = String::from_utf8_lossy(&buf[..read]).to_string();
                    }
                }
                let counter = hits.fetch_add(1, Ordering::SeqCst) + 1;
                let (status, body) = responder(counter);
                let reason = if status == 200 { "OK" } else { "ERROR" };
                let header = format!(
                    "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(header.as_bytes()).await;
                let _ = socket.write_all(&body).await;
            });
        }
    });

    (format!("http://{address}"), hits, head)
}

/// Ответ в формате OpenAI: `{ "data": [{ "embedding": [...] }], ... }`.
pub fn json_response(embedding: Vec<f64>) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "object": "list",
        "data": [
            {
                "object": "embedding",
                "index": 0,
                "embedding": embedding,
            }
        ],
        "model": "bge",
        "usage": { "prompt_tokens": 1, "total_tokens": 1 },
    }))
    .unwrap()
}
