// 08_real_world_use_cases/src/main.rs
//
// Rust en Producción — Casos de Uso Reales
// Ejecutar con: cargo run
//
// Este módulo simula patrones de código usados en aplicaciones
// de producción reales donde Rust brilla.

use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 8: Rust en Producción           ║");
    println!("╚══════════════════════════════════════════╝\n");

    caso_cli_tool();
    caso_parser_seguro();
    caso_cache_concurrente();
    caso_blockchain_hash();
    casos_reales_en_industria();
}

// ============================================================
// 1. Herramienta CLI — Patrón común en CLI tools
// ============================================================
fn caso_cli_tool() {
    println!("=== 1. Herramienta CLI ===");
    println!("(Patrón usado en: ripgrep, bat, fd, exa, starship)\n");

    // Simular argparse y procesamiento de archivos
    #[derive(Debug)]
    struct Config {
        patron: String,
        sensible_mayusculas: bool,
        max_resultados: usize,
    }

    impl Config {
        fn new(patron: &str) -> Self {
            Config {
                patron: patron.to_lowercase(),
                sensible_mayusculas: false,
                max_resultados: 100,
            }
        }
    }

    fn buscar_en_lineas<'a>(config: &Config, lineas: &'a [&str]) -> Vec<(usize, &'a str)> {
        lineas.iter()
            .enumerate()
            .filter(|(_, linea)| {
                let texto = if config.sensible_mayusculas {
                    linea.to_string()
                } else {
                    linea.to_lowercase()
                };
                texto.contains(&config.patron)
            })
            .map(|(i, linea)| (i + 1, *linea))
            .take(config.max_resultados)
            .collect()
    }

    let config = Config::new("rust");
    let contenido = vec![
        "Rust es un lenguaje de programación de sistemas",
        "Go es popular para microservicios",
        "Rust garantiza memory safety sin GC",
        "Python es ideal para data science",
        "El compilador de Rust previene data races",
    ];

    let resultados = buscar_en_lineas(&config, &contenido);

    println!("Búsqueda de '{}' en {} líneas:", config.patron, contenido.len());
    for (num, linea) in &resultados {
        println!("  Línea {}: {}", num, linea);
    }
    println!("Encontradas: {} coincidencias\n", resultados.len());
}

// ============================================================
// 2. Parser Seguro — Procesamiento de Datos
// ============================================================
fn caso_parser_seguro() {
    println!("=== 2. Parser Seguro ===");
    println!("(Patrón usado en: serde, nom, parsers de protocolos)\n");

    // Mini parser de configuración
    #[derive(Debug, PartialEq)]
    enum Valor {
        Texto(String),
        Numero(f64),
        Booleano(bool),
        Lista(Vec<Valor>),
    }

    impl fmt::Display for Valor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Valor::Texto(s) => write!(f, "\"{}\"", s),
                Valor::Numero(n) => write!(f, "{}", n),
                Valor::Booleano(b) => write!(f, "{}", b),
                Valor::Lista(items) => {
                    write!(f, "[")?;
                    for (i, item) in items.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", item)?;
                    }
                    write!(f, "]")
                }
            }
        }
    }

    fn parsear_valor(s: &str) -> Result<Valor, String> {
        let s = s.trim();

        if s == "true" { return Ok(Valor::Booleano(true)); }
        if s == "false" { return Ok(Valor::Booleano(false)); }

        if let Ok(n) = s.parse::<f64>() {
            return Ok(Valor::Numero(n));
        }

        if s.starts_with('"') && s.ends_with('"') {
            return Ok(Valor::Texto(s[1..s.len()-1].to_string()));
        }

        Err(format!("Valor no reconocido: '{}'", s))
    }

    fn parsear_config(config: &str) -> Result<HashMap<String, Valor>, String> {
        let mut mapa = HashMap::new();

        for linea in config.lines() {
            let linea = linea.trim();
            if linea.is_empty() || linea.starts_with('#') { continue; }

            let partes: Vec<&str> = linea.splitn(2, '=').collect();
            if partes.len() != 2 {
                return Err(format!("Línea inválida: '{}'", linea));
            }

            let clave = partes[0].trim().to_string();
            let valor = parsear_valor(partes[1].trim())?;
            mapa.insert(clave, valor);
        }

        Ok(mapa)
    }

    let config_texto = r#"
# Configuración de ejemplo
nombre = "mi-aplicacion"
version = 1.5
debug = true
max_conexiones = 100
"#;

    match parsear_config(config_texto) {
        Ok(config) => {
            println!("Config parseada exitosamente:");
            let mut claves: Vec<&String> = config.keys().collect();
            claves.sort();
            for clave in claves {
                println!("  {} = {}", clave, config[clave]);
            }
        }
        Err(e) => println!("Error de parseo: {}", e),
    }

    // Probar con config inválida:
    match parsear_config("clave_sin_valor") {
        Ok(_) => println!("Esto no debería pasar"),
        Err(e) => println!("Error esperado: {}", e),
    }

    println!();
}

// ============================================================
// 3. Cache Concurrente — Patrón de Backend
// ============================================================
fn caso_cache_concurrente() {
    println!("=== 3. Cache Concurrente ===");
    println!("(Patrón usado en backends web, microservicios)\n");

    use std::sync::{Arc, RwLock};
    use std::thread;

    // RwLock permite múltiples lectores O un escritor
    type Cache = Arc<RwLock<HashMap<String, String>>>;

    fn crear_cache() -> Cache {
        Arc::new(RwLock::new(HashMap::new()))
    }

    fn get(cache: &Cache, key: &str) -> Option<String> {
        // read() permite acceso concurrente — múltiples lectores
        cache.read().unwrap().get(key).cloned()
    }

    fn set(cache: &Cache, key: String, value: String) {
        // write() es exclusivo — bloquea lectores y otros escritores
        cache.write().unwrap().insert(key, value);
    }

    let cache = crear_cache();

    // Poblar cache:
    set(&cache, "usuario:1".to_string(), "Alice".to_string());
    set(&cache, "usuario:2".to_string(), "Bob".to_string());
    set(&cache, "config:max".to_string(), "100".to_string());

    // Múltiples hilos leyendo concurrentemente:
    let mut handles = vec![];
    for i in 1..=3 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let key = format!("usuario:{}", i);
            match get(&cache_clone, &key) {
                Some(val) => println!("  Hilo {}: {} = {}", i, key, val),
                None => println!("  Hilo {}: {} no encontrado", i, key),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Escritura concurrente desde un hilo:
    let cache_clone = Arc::clone(&cache);
    let writer = thread::spawn(move || {
        set(&cache_clone, "nuevo:key".to_string(), "valor fresco".to_string());
        println!("  Escritor: clave agregada al cache");
    });
    writer.join().unwrap();

    println!("Cache con {} entradas ✅", cache.read().unwrap().len());
    println!();
}

// ============================================================
// 4. Hash Criptográfico — Patrón Blockchain
// ============================================================
fn caso_blockchain_hash() {
    println!("=== 4. Hash Simplificado (Patrón Blockchain) ===");
    println!("(Patrón usado en: Solana, Polkadot, Near Protocol)\n");

    // Hash simple para demo (NO criptográficamente seguro)
    // En producción: usar sha2, blake3, etc.
    fn hash_simple(data: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in data.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }

    #[derive(Debug)]
    struct Bloque {
        indice: u64,
        datos: String,
        hash_previo: u64,
        hash: u64,
        timestamp: u64,
    }

    impl Bloque {
        fn nuevo(indice: u64, datos: String, hash_previo: u64) -> Self {
            let timestamp = 1_700_000_000 + indice * 10; // Simulado
            let contenido = format!("{}{}{}{}", indice, &datos, hash_previo, timestamp);
            let hash = hash_simple(&contenido);
            Bloque { indice, datos, hash_previo, hash, timestamp }
        }

        fn es_valido(&self, bloque_previo: &Bloque) -> bool {
            self.hash_previo == bloque_previo.hash
        }
    }

    struct Blockchain {
        cadena: Vec<Bloque>,
    }

    impl Blockchain {
        fn nueva() -> Self {
            let genesis = Bloque::nuevo(0, "Bloque Génesis".to_string(), 0);
            Blockchain { cadena: vec![genesis] }
        }

        fn agregar(&mut self, datos: String) {
            let ultimo_hash = self.cadena.last().unwrap().hash;
            let indice = self.cadena.len() as u64;
            let bloque = Bloque::nuevo(indice, datos, ultimo_hash);
            self.cadena.push(bloque);
        }

        fn es_valida(&self) -> bool {
            self.cadena.windows(2).all(|par| par[1].es_valido(&par[0]))
        }
    }

    let mut chain = Blockchain::nueva();
    chain.agregar("Tx: Alice → Bob: 50 RustCoin".to_string());
    chain.agregar("Tx: Bob → Carol: 30 RustCoin".to_string());
    chain.agregar("Tx: Carol → Alice: 10 RustCoin".to_string());

    for bloque in &chain.cadena {
        println!("  Bloque #{}: hash={:016x} datos='{}'",
            bloque.indice, bloque.hash, bloque.datos);
    }

    println!("\n  Cadena válida: {} ✅", chain.es_valida());
    println!();
}

// ============================================================
// 5. Rust en la Industria Real
// ============================================================
fn casos_reales_en_industria() {
    println!("=== 5. Rust en la Industria — Adopción Real ===\n");

    let casos = vec![
        ("🐧 Linux Kernel",
         "v6.1 (2022): primera versión con código Rust oficial.",
         "Drivers y subsistemas donde la safety es crítica."),

        ("🤖 Android (Google)",
         "Nuevo código del sistema operativo preferentemente en Rust.",
         "Redujo vulnerabilidades de memoria en 70% en componentes migrados."),

        ("🪟 Windows (Microsoft)",
         "Reescribiendo componentes críticos del kernel en Rust.",
         "npm CLI, Azure IoT, y otros componentes ya usan Rust."),

        ("🌐 WebAssembly",
         "Rust es el lenguaje más popular para compilar a WASM.",
         "Figma, Cloudflare Workers, Firefox componentes internos."),

        ("⛓️  Blockchain",
         "Solana, Polkadot/Substrate, Near Protocol escritos en Rust.",
         "Safety de memoria es crítica para contratos inteligentes."),

        ("🛠️  Herramientas CLI",
         "ripgrep (rg), bat, fd, exa, starship, cargo mismo.",
         "Más rápidos que equivalentes en C/Go con código más seguro."),

        ("🦊 Firefox (Mozilla)",
         "Rust nació en Mozilla. Stylo CSS engine, WebRender en Rust.",
         "Elimina clases enteras de CVEs de memoria."),

        ("☁️  AWS / Cloudflare",
         "Firecracker VMM (AWS Lambda), Pingora (Cloudflare) en Rust.",
         "Latencia predecible sin GC pauses en infraestructura crítica."),
    ];

    for (empresa, descripcion, impacto) in casos {
        println!("  {} — {}", empresa, descripcion);
        println!("    Impacto: {}\n", impacto);
    }

    println!("Rust lleva 8 años consecutivos siendo el lenguaje");
    println!("más amado en la encuesta anual de Stack Overflow (2016-2024).");
}
