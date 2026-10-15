use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Ответ: (http_status, body_bytes). Ответчик вызывает с номером принятого соединения, начиная с 1.
pub async fn start_test_server(
    responder: Arc<dyn Fn(u32) -> (u16, Vec<u8>) + Send + Sync>,
) -> (String, Arc<AtomicU32>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let address = listener.local_addr().expect("local addr");
    let hits = Arc::new(AtomicU32::new(0));
    let hits_clone = hits.clone();

    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };
            let hits = hits_clone.clone();
            let responder = responder.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let _ = socket.read(&mut buf).await; // дочитываем начало запроса
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

    (format!("http://{address}"), hits)
}

pub fn json_response(embedding: Vec<f64>) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({ "embedding": embedding })).unwrap()
}
