use serde::Serialize;
use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

#[derive(Serialize, Debug)]
struct InfoEquipo {
    sistema_operativo: SistemaOperativo,
    procesadores: Vec<Procesador>,
    memoria: Memoria,
    discos: Vec<Disco>,
    componentes: Vec<Componente>,
    redes: Vec<Red>,
    procesos: Vec<Proceso>,
}

#[derive(Debug, Serialize)]
struct SistemaOperativo {
    nombre: String,
    version: String,
    version_kernel: String,
}

#[derive(Debug, Serialize)]
struct Procesador {
    nombre: String,
    marca: String,
    frecuencia: String,
}

#[derive(Debug, Serialize)]
struct Memoria {
    memoria_total: String,
    memoria_usada: String,
    memoria_swap_total: String,
    memoria_swap_usada: String,
}

#[derive(Debug, Serialize)]
struct Disco {
    tipo: String,
    nombre: String,
    sistema: String,
    punto_montado: String,
    capacidad: String,
    espacio: String,
    extraible: String,
}

#[derive(Debug, Serialize)]
struct Componente {
    etiqueta: String,
    temperatura: String,
    temperatura_maxima: String,
    temperatura_critica: String,
}

#[derive(Debug, Serialize)]
struct Red {
    nombre: String,
    recibido: String,
    transmitido: String,
}

#[derive(Debug, Serialize)]
struct Proceso {
    pid: String,
    nombre: String,
    status: String,
    ordenar_por_memoria: u64,
    memoria: String,
}

fn bytes_a_formato_humano(bytes: u64) -> String {
    let b = bytes as f64;
    if bytes >= 1073741824 {
        let medida = b / 1073741824 as f64;
        return format!("{:.2} GB", medida);
    } else if bytes >= 1048576 {
        let medida = b / 1048576 as f64;
        return format!("{:.2} MB", medida);
    } else if bytes >= 1024 {
        let medida = b / 1024 as f64;
        return format!("{:.2} KB", medida);
    } else {
        return format!("{} bytes", b);
    }
}

pub async fn devolver_json_info_sistema() -> Result<String, String> {
    let mut sys = System::new_all();

    sys.refresh_all();

    // Sistema operativo
    let mut nombre = "Desconocido".to_string();
    match sys.name() {
        Some(ok) => nombre = ok,
        None => (),
    }

    let mut version = "Desconocida".to_string();
    match sys.os_version() {
        Some(ok) => version = ok,
        None => (),
    }

    let mut version_kernel = "Desconocido".to_string();
    match sys.kernel_version() {
        Some(ok) => version_kernel = ok,
        None => (),
    }

    let sistema_operativo = SistemaOperativo {
        nombre,
        version,
        version_kernel,
    };

    // Procesadores
    let mut procesadores: Vec<Procesador> = Vec::new();
    for cpu in sys.cpus() {
        let nombre = cpu.name().to_string();
        let marca = cpu.brand().to_string();
        let frecuencia = format!("{} MHz", cpu.frequency());
        procesadores.push(Procesador {
            nombre,
            marca,
            frecuencia,
        })
    }

    // Memoria
    let memoria_total = bytes_a_formato_humano(sys.total_memory());
    let memoria_usada = bytes_a_formato_humano(sys.used_memory());
    let memoria_swap_total = bytes_a_formato_humano(sys.total_swap());
    let memoria_swap_usada = bytes_a_formato_humano(sys.used_swap());
    let memoria = Memoria {
        memoria_total,
        memoria_usada,
        memoria_swap_total,
        memoria_swap_usada,
    };

    // Discos
    let mut discos: Vec<Disco> = Vec::new();
    for disk in sys.disks() {
        let tipo = format!("{:?}", disk.kind());
        let nombre = format!("{:?}", disk.name());
        let sistema = String::from_utf8_lossy(disk.file_system()).to_string();
        let punto_montado = format!("{:?}", disk.mount_point());
        let capacidad = bytes_a_formato_humano(disk.total_space());
        let espacio = bytes_a_formato_humano(disk.available_space());
        let extraible = format!("{}", disk.is_removable());
        discos.push(Disco {
            tipo,
            nombre,
            sistema,
            punto_montado,
            capacidad,
            espacio,
            extraible,
        });
    }

    // Componentes
    let mut componentes: Vec<Componente> = Vec::new();
    for componente_info in sys.components() {
        let etiqueta = componente_info.label().to_string();
        let temperatura = format!("{} °C", componente_info.temperature());
        let temperatura_maxima = format!("{} °C", componente_info.max());
        let mut temperatura_critica = "Desconocida".to_string();
        match componente_info.critical() {
            Some(temp) => temperatura_critica = format!("{temp} °C"),
            None => (),
        }
        componentes.push(Componente {
            etiqueta,
            temperatura,
            temperatura_maxima,
            temperatura_critica,
        });
    }

    // Redes info
    let mut redes: Vec<Red> = Vec::new();
    for (red_nombre, data) in sys.networks() {
        let nombre = red_nombre.to_string();
        let recibido = bytes_a_formato_humano(data.received());
        let transmitido = bytes_a_formato_humano(data.transmitted());
        redes.push(Red {
            nombre,
            recibido,
            transmitido,
        });
    }

    // Procesos
    let mut procesos: Vec<Proceso> = Vec::new();
    for (_, proceso) in sys.processes() {
        let pid = proceso.pid().to_string();
        let nombre = proceso.name().to_string();
        let status = proceso.status().to_string();
        let memoria = bytes_a_formato_humano(proceso.memory());
        let ordenar_por_memoria = proceso.memory();
        procesos.push(Proceso {
            pid,
            nombre,
            status,
            memoria,
            ordenar_por_memoria,
        })
    }

    procesos.sort_by(|a, b| {
        b.ordenar_por_memoria
            .partial_cmp(&a.ordenar_por_memoria)
            .unwrap()
    });

    let info_equipo = InfoEquipo {
        sistema_operativo,
        procesadores,
        memoria,
        discos,
        componentes,
        redes,
        procesos,
    };

    match serde_json::to_string(&info_equipo) {
        Ok(json) => return Ok(json),
        Err(error) => {
            let mensaje_error =
                format!("no se ha podido serializar el json por un error: {}", error);
            return Err(mensaje_error);
        }
    }
}
