use tokio::sync::watch::Sender;
mod obtener_info;
//use sysinfo::{System, SystemExt};

pub async fn recuperar_info_sistema(transmisor: Sender<String>) {
    loop {
        match obtener_info::devolver_json_info_sistema().await {
            Ok(json) => match transmisor.send(json) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("No se ha podido enviar el mensaje: {error}");
                }
            },
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        //tokio::time::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL).await;
    }
}
