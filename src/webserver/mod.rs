use tokio::net::{TcpListener, TcpStream};

const HTML_PAGINA: &str = r#"HTTP/1.1 200 OK
Content-Type: text/html

<!DOCTYPE html>
<html lang="es">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Monitorización del sistema</title>
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto:300,300italic,700,700italic">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/milligram/1.4.1/milligram.css">
</head>

<body>

    <div class="container">
        <blockquote>
            <h3>Sistema operativo</h3>
            <p><span id="soNombre"></span> (<span id="soVersion"></span>) kernel: <span id="soVersionKernel"></span>.
                Numero de procesadores: <span id="numeroProcesadores"></span></p>
            <ul>
                <li>Memoria usada <span id="memoriaUsada"></span> de <span id="memoriaTotal"></span> disponible.</li>
                <li>Memoria swap usada <span id="memoriaSwapUsada"></span> de <span id="memoriaSwapTotal"></span>
                    disponible.</li>
            </ul>
        </blockquote>

        <blockquote>
            <h3>Procesos</h3>
            <div id="procesos">

            </div>
        </blockquote>

    </div>

    <script>

        const soNombre = document.getElementById("soNombre")
        const soVersion = document.getElementById("soVersion")
        const soVersionKernel = document.getElementById("soVersionKernel")
        const numeroProcesadores = document.getElementById("numeroProcesadores")

        const memoriaUsada = document.getElementById("memoriaUsada")
        const memoriaTotal = document.getElementById("memoriaTotal")
        const memoriaSwapUsada = document.getElementById("memoriaSwapUsada")
        const memoriaSwapTotal = document.getElementById("memoriaSwapTotal")

        const procesos = document.getElementById("procesos")

        ws = new WebSocket("ws://localhost:8766"),

            ws.onopen = () => {
                console.log("conexión ws correcta")
            }

        ws.onmessage = (evento) => {
            const data = JSON.parse(evento.data)

            const jsonSistemaOperativo = data.sistema_operativo
            soNombre.innerText = jsonSistemaOperativo.nombre
            soVersion.innerText = jsonSistemaOperativo.version
            soVersionKernel.innerText = jsonSistemaOperativo.version_kernel

            const jsonProcesadores = data.procesadores
            numeroProcesadores.innerText = jsonProcesadores.length

            const jsonMemoria = data.memoria
            memoriaUsada.innerText = jsonMemoria.memoria_usada
            memoriaTotal.innerText = jsonMemoria.memoria_total
            memoriaSwapUsada.innerText = jsonMemoria.memoria_swap_usada
            memoriaSwapTotal.innerText = jsonMemoria.memoria_swap_total

            const jsonProcesos = data.procesos
            let cuerpoProcesos = ""
            for (proceso of jsonProcesos) {
                cuerpoProcesos += `
            <tr>
                <td>${proceso.pid}</td>
                <td>${proceso.nombre}</td>
                <td>${proceso.status}</td>
                <td>${proceso.memoria}</td>
            </tr>`
            }
            const tablaProcesos = `
            <table>
                <thead>
                    <tr>
                        <th>pid</th>
                        <th>nombre</th>
                        <th>status</th>
                        <th>memoria</th>
                    </tr>
                </thead>
                <tbody>${cuerpoProcesos}</tbody>
            </table>`

            procesos.innerHTML = tablaProcesos

        }
    </script>
</body>

</html>
"#;

async fn servir_pagina(stream: TcpStream) {
    stream.writable().await.unwrap();
    stream.try_write(HTML_PAGINA.as_bytes()).unwrap();
}

pub async fn crear_servidor_web() {
    let manejador = TcpListener::bind("127.0.0.1:8765").await.unwrap();
    println!("Servidor web iniciado en http://localhost:8765");

    while let Ok((stream, _)) = manejador.accept().await {
        tokio::spawn(async move {
            servir_pagina(stream).await;
        });
    }
}
