// 05_fearless_concurrency/src/main.rs
//
// Concurrencia Sin Miedo (Fearless Concurrency) en Rust
// Ejecutar con: cargo run
//
// El sistema de tipos de Rust hace que las condiciones de carrera
// sean IMPOSIBLES en compile time — no como convención, sino
// como garantía matemática del compilador.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 5: Fearless Concurrency         ║");
    println!("╚══════════════════════════════════════════╝\n");

    hilos_basicos();
    move_closures_en_hilos();
    compartir_datos_con_arc_mutex();
    canales_para_comunicacion();
    contador_concurrente();
}

// ============================================================
// 1. Hilos Básicos
// ============================================================
fn hilos_basicos() {
    println!("=== 1. Hilos Básicos ===");

    let hilo = thread::spawn(|| {
        for i in 1..=5 {
            println!("  Hilo secundario: iteración {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("Hilo principal: iteración {}", i);
        thread::sleep(Duration::from_millis(15));
    }

    // join() espera a que el hilo termine
    hilo.join().expect("El hilo falló");
    println!("Ambos hilos completados ✅\n");
}

// ============================================================
// 2. Move Closures — Transferir Ownership a Hilos
// ============================================================
fn move_closures_en_hilos() {
    println!("=== 2. Move Closures en Hilos ===");

    let mensaje = String::from("Hola desde el hilo principal!");

    // `move` transfiere ownership de `mensaje` al hilo
    // Sin `move`, el compilador rechazaría el código porque el hilo
    // podría outlive la variable local `mensaje`
    let hilo = thread::spawn(move || {
        println!("  Hilo recibió: '{}'", mensaje);
        // mensaje es propiedad del hilo — no hay problema de lifetime
    });

    // println!("{}", mensaje);  // ❌ ERROR: mensaje fue movido al hilo

    hilo.join().expect("El hilo falló");
    println!("Move closure permite compartir datos con seguridad ✅\n");
}

// ============================================================
// 3. Arc<Mutex<T>> — Datos Compartidos Entre Hilos
// ============================================================
fn compartir_datos_con_arc_mutex() {
    println!("=== 3. Arc<Mutex<T>> — Compartir Datos ===");

    // Arc = Atomic Reference Counting — como Rc<T> pero thread-safe
    // Mutex = Mutual Exclusion — garantiza acceso exclusivo
    let contador = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];

    for id in 0..5 {
        let contador_clone = Arc::clone(&contador); // Clonar el Arc (no el dato)

        let handle = thread::spawn(move || {
            // lock() bloquea hasta obtener acceso exclusivo
            let mut num = contador_clone.lock().expect("Mutex envenenado");
            *num += 10;
            println!("  Hilo {}: contador = {}", id, *num);
            // El lock se libera automáticamente cuando `num` sale de scope
        });

        handles.push(handle);
    }

    // Esperar todos los hilos
    for handle in handles {
        handle.join().expect("Hilo falló");
    }

    let valor_final = *contador.lock().unwrap();
    println!("Valor final: {} (esperado: 50) ✅\n", valor_final);

    // En C, este mismo patrón sin el Mutex causaría data race
    // En Rust, intentar compartir datos entre hilos SIN sincronización
    // es un ERROR DE COMPILACIÓN:
    //
    // let datos = vec![1, 2, 3];
    // thread::spawn(|| { println!("{:?}", datos); });
    // thread::spawn(|| { datos.push(4); });
    // ❌ ERROR: datos no implementa Send+Sync de forma segura
}

// ============================================================
// 4. Canales (Channels) — Comunicación Entre Hilos
// ============================================================
fn canales_para_comunicacion() {
    println!("=== 4. Canales (Message Passing) ===");

    // mpsc = Multiple Producer, Single Consumer
    let (tx, rx) = std::sync::mpsc::channel();

    // Productor en hilo separado
    let handle = thread::spawn(move || {
        let mensajes = vec!["Mensaje 1", "Mensaje 2", "Mensaje 3", "fin"];

        for msg in mensajes {
            tx.send(msg).expect("Error al enviar");
            thread::sleep(Duration::from_millis(20));
        }
    });

    // Consumidor en hilo principal
    for recibido in &rx {
        println!("  Recibido: '{}'", recibido);
        if recibido == "fin" {
            break;
        }
    }

    handle.join().expect("Hilo productor falló");

    println!("Comunicación via canales ✅");
    println!("→ Los canales transfieren OWNERSHIP — no hay datos compartidos\n");
}

// ============================================================
// 5. Ejemplo Completo: Contador Concurrente
// ============================================================
fn contador_concurrente() {
    println!("=== 5. Contador Concurrente — Ejemplo Real ===");

    let total_tareas = 20;
    let num_hilos = 4;

    let completadas = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    println!("Procesando {} tareas con {} hilos...", total_tareas, num_hilos);

    for id_hilo in 0..num_hilos {
        let completadas = Arc::clone(&completadas);
        let tareas_por_hilo = total_tareas / num_hilos;

        let handle = thread::spawn(move || {
            for _ in 0..tareas_por_hilo {
                // Simular trabajo
                thread::sleep(Duration::from_millis(5));

                let mut count = completadas.lock().unwrap();
                *count += 1;
            }
            println!("  Hilo {} completó sus {} tareas", id_hilo, tareas_por_hilo);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total = *completadas.lock().unwrap();
    println!("Total tareas completadas: {}/{} ✅\n", total, total_tareas);
}

// ============================================================
// RESUMEN: Traits de Seguridad en Concurrencia
// ============================================================
//
// Rust garantiza concurrencia segura con dos traits marcadores:
//
// Send — el tipo puede transferirse a otro hilo
//   → La mayoría de tipos son Send
//   → Rc<T> NO es Send (usar Arc<T> en su lugar)
//
// Sync — el tipo puede referenciarse desde múltiples hilos
//   → T es Sync si &T es Send
//   → RefCell<T> NO es Sync (usar Mutex<T> en su lugar)
//
// El compilador verifica Send y Sync automáticamente.
// Si intentas compartir datos inseguros entre hilos: ERROR DE COMPILACIÓN.
// No en runtime. No como crash. En compile time, con mensaje descriptivo.
//
// Esto es "Fearless Concurrency" — escribe código concurrente
// confiando en que el compilador detectará los bugs, no tú.
