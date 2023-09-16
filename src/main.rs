mod info_sistema;
mod webserver;
mod websocket;

#[tokio::main]
async fn main() {
    let (transmisor, receptor) = tokio::sync::watch::channel("".to_string());

    let hilo_info_sistema = tokio::spawn(async move {
        info_sistema::recuperar_info_sistema(transmisor).await;
    });

    let hilo_websocket = tokio::spawn(async move {
        websocket::crear_websocket(receptor).await;
    });

    let hilo_webserver = tokio::spawn(async {
        webserver::crear_servidor_web().await;
    });

    let _ = tokio::join!(hilo_info_sistema, hilo_websocket, hilo_webserver);
}
