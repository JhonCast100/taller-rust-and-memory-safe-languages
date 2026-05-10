// 07_comparison_languages/memory_rust/src/main.rs
//
// Gestión de Memoria en Rust — Sin Garbage Collector
// Ejecutar con: cargo run

use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Gestión de Memoria en Rust (sin GC)    ║");
    println!("╚══════════════════════════════════════════╝\n");

    demo_raii();
    demo_stack_vs_heap();
    demo_smart_pointers();
    demo_sin_null();
    demo_latencia_deterministica();
    resumen_comparativo();
}

// ============================================================
// 1. RAII — Destrucción Determinística
// ============================================================
fn demo_raii() {
    println!("=== 1. RAII — Destrucción Determinística ===");

    // RAII = Resource Acquisition Is Initialization
    // En Rust, cuando un valor sale de scope, se destruye INMEDIATAMENTE
    // No hay GC pause. No hay espera. Es determinístico.

    println!("Entrando al scope:");
    {
        let datos = vec![1, 2, 3, 4, 5]; // Asignado en heap
        println!("  Vec creado con {} elementos", datos.len());
        // ... usar datos ...
    } // ← Drop llamado AQUÍ. Inmediatamente. Sin GC.
    println!("Saliendo del scope — Vec destruido inmediatamente\n");

    // Demo con tipo personalizado que muestra el drop:
    struct Recurso {
        nombre: String,
    }

    impl Drop for Recurso {
        fn drop(&mut self) {
            println!("  [DROP] '{}' destruido — memoria liberada", self.nombre);
        }
    }

    println!("Creando recursos:");
    let _r1 = Recurso { nombre: "Recurso-A".to_string() };
    {
        let _r2 = Recurso { nombre: "Recurso-B".to_string() };
        let _r3 = Recurso { nombre: "Recurso-C".to_string() };
        println!("  Dentro del scope: A, B, C existen");
    } // B y C destruidos aquí
    println!("  Fuera del bloque: solo A existe");
    // A destruido aquí al final de la función
    println!();
}

// ============================================================
// 2. Stack vs Heap — Control Explícito
// ============================================================
fn demo_stack_vs_heap() {
    println!("=== 2. Stack vs Heap ===");

    // Stack — tipos de tamaño fijo, muy rápido:
    let _a: i32 = 42;        // Stack
    let _b: [f64; 10] = [0.0; 10]; // Stack: 80 bytes
    let _c: (bool, char) = (true, 'R'); // Stack

    println!("Tipos en stack: acceso instantáneo, liberación automática");

    // Heap — tipos dinámicos, requieren allocación:
    let _s = String::from("hola"); // Heap: data del string
    let _v: Vec<i32> = vec![1, 2, 3]; // Heap: buffer del vec
    let _b = Box::new(42i32);        // Heap: explícito con Box<T>

    println!("Tipos en heap: Box<T>, Vec<T>, String, HashMap, etc.");
    println!("Todos se liberan automáticamente vía Drop al salir de scope");

    // Comparación de rendimiento conceptual:
    println!();
    println!("Comparativa (conceptual):");
    println!("  C/C++  → Heap manual: malloc/free, riesgo de leaks y doble-free");
    println!("  Java   → Todo objeto en heap, GC decide cuándo liberar");
    println!("  Go     → Escape analysis decide stack/heap, GC libera heap");
    println!("  Rust   → Stack cuando posible, Heap con Drop determinístico");

    println!();
}

// ============================================================
// 3. Smart Pointers — Gestión Avanzada
// ============================================================
fn demo_smart_pointers() {
    println!("=== 3. Smart Pointers ===");

    // Box<T> — Puntero heap básico, ownership único:
    let boxed = Box::new(42);
    println!("Box<i32>: {}", boxed);
    // Se destruye automáticamente — sin free() manual

    // Rc<T> — Reference Counting, múltiples owners (single-threaded):
    use std::rc::Rc;
    let shared = Rc::new(String::from("compartido"));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);
    println!("Rc<String>: '{}' — {} referencias", shared, Rc::strong_count(&shared));
    drop(clone1);
    println!("Después de drop(clone1): {} referencias", Rc::strong_count(&shared));
    drop(clone2);
    println!("Después de drop(clone2): {} referencias", Rc::strong_count(&shared));
    // Cuando el count llega a 0, se destruye automáticamente

    // Arc<T> — Atomic Rc, para múltiples hilos (thread-safe):
    use std::sync::Arc;
    let thread_shared = Arc::new(vec![1, 2, 3]);
    let clone_para_hilo = Arc::clone(&thread_shared);
    let handle = std::thread::spawn(move || {
        println!("Hilo accede al Arc: {:?}", clone_para_hilo);
    });
    handle.join().unwrap();
    println!("Arc<Vec<i32>>: {:?} — thread-safe reference counting", thread_shared);

    println!();
}

// ============================================================
// 4. Sin null — Option<T> como Garantía del Sistema de Tipos
// ============================================================
fn demo_sin_null() {
    println!("=== 4. Sin null — Option<T> ===");

    // En Rust NO EXISTE null. La ausencia se modela con Option<T>.
    // El compilador OBLIGA a manejar el caso None.

    fn buscar_usuario(id: u32) -> Option<String> {
        match id {
            1 => Some("Alice".to_string()),
            2 => Some("Bob".to_string()),
            _ => None, // No null — None explícito
        }
    }

    // El compilador no permite usar Option<String> como String directamente:
    let usuario = buscar_usuario(1);
    // println!("{}", usuario);  // ❌ ERROR: Option<String> no es String

    // Debes manejarlo explícitamente:
    match buscar_usuario(999) {
        Some(nombre) => println!("Usuario: {}", nombre),
        None => println!("Usuario 999 no existe — manejado con seguridad ✅"),
    }

    // Métodos ergonómicos:
    let nombre = buscar_usuario(1).unwrap_or_else(|| "Anónimo".to_string());
    println!("Nombre: {}", nombre);

    let longitud = buscar_usuario(2).map(|n| n.len());
    println!("Longitud del nombre: {:?}", longitud);

    println!("→ NullPointerException es IMPOSIBLE en Rust safe code");

    println!();
}

// ============================================================
// 5. Latencia Determinística
// ============================================================
fn demo_latencia_deterministica() {
    println!("=== 5. Latencia Determinística (sin GC pauses) ===");

    let mut tiempos = Vec::new();

    for i in 0..10 {
        let inicio = Instant::now();

        // Crear y destruir datos — sin GC intermedio:
        let datos: Vec<Vec<i32>> = (0..1000)
            .map(|_| vec![0i32; 1000])
            .collect();
        let suma: i32 = datos.iter().flatten().sum();
        let _ = suma; // Usar suma para evitar optimización

        let elapsed = inicio.elapsed();
        tiempos.push(elapsed);
        println!("  Iteración {}: {:?}", i, elapsed);
        // datos se destruye aquí — liberación inmediata
    }

    let promedio = tiempos.iter().sum::<std::time::Duration>() / tiempos.len() as u32;
    let max = tiempos.iter().max().unwrap();
    let min = tiempos.iter().min().unwrap();

    println!("Promedio: {:?} | Min: {:?} | Max: {:?}", promedio, min, max);
    println!("→ Latencia predecible: no hay pausa inesperada del GC");

    println!();
}

// ============================================================
// 6. Resumen Comparativo
// ============================================================
fn resumen_comparativo() {
    println!("=== Resumen Comparativo Final ===\n");

    let tabla = [
        ("Característica",    "C/C++",      "Java",       "Go",         "Rust"),
        ("Gestión memoria",   "Manual",     "GC",         "GC",         "Ownership"),
        ("Null safety",       "❌ No",       "⚠️  Parcial",  "⚠️  nil",     "✅ Option<T>"),
        ("Race conditions",   "❌ Runtime",  "⚠️  Runtime",  "⚠️  -race",   "✅ Compile"),
        ("GC pauses",         "✅ Sin GC",   "❌ Sí",       "⚠️  Mínimas",  "✅ Sin GC"),
        ("Velocidad",         "⚡ Máxima",   "🔶 Buena",    "🔶 Buena",    "⚡ Máxima"),
        ("Seguridad",         "❌ Manual",   "🔶 GC ayuda", "🔶 GC ayuda", "✅ Compilador"),
        ("Curva aprendizaje", "🔶 Alta",     "✅ Media",    "✅ Baja",     "❌ Alta inicial"),
    ];

    for (i, row) in tabla.iter().enumerate() {
        if i == 0 {
            println!("{:<25} {:<15} {:<15} {:<15} {}", row.0, row.1, row.2, row.3, row.4);
            println!("{}", "─".repeat(85));
        } else {
            println!("{:<25} {:<15} {:<15} {:<15} {}", row.0, row.1, row.2, row.3, row.4);
        }
    }

    println!("\nConclusion:");
    println!("Rust combina el rendimiento de C/C++ con la seguridad de Java/Go,");
    println!("garantizando memory safety en compile-time sin sacrificar velocidad.");
}
