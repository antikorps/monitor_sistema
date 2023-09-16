use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch::Receiver;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

async fn gestionar_websocket(stream: TcpStream, receptor: Arc<Mutex<Receiver<String>>>) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws_stream) => ws_stream,
        Err(error) => {
            eprintln!("Error al aceptar WebSocket: {:?}", error);
            return;
        }
    };

    let (mut salida, _) = ws_stream.split();

    let mut receptor_desbloqueado = receptor.lock().await;
    while receptor_desbloqueado.changed().await.is_ok() {
        let mensaje = receptor_desbloqueado.borrow().clone();
        match salida.send(Message::Text(mensaje.to_string())).await {
            Ok(_) => (),
            Err(_) => {
                // Cliente desconectado. Die, die, my darling
                break;
            }
        }
    }
}

pub async fn crear_websocket(receptor: Receiver<String>) {
    let manejador = TcpListener::bind("127.0.0.1:8766").await.unwrap();

    while let Ok((stream, _)) = manejador.accept().await {
        // Nueva conexión
        let receptor_seguro = Arc::new(Mutex::new(receptor.clone()));
        let receptor_clonado = Arc::clone(&receptor_seguro);

        tokio::spawn(async move {
            gestionar_websocket(stream, receptor_clonado).await;
        });
    }
}
