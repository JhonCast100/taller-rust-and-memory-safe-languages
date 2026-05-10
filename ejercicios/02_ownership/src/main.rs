// 02_ownership/src/main.rs
//
// Fundamentos de Ownership en Rust
// Ejecutar con: cargo run

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 2: Ownership en Rust            ║");
    println!("╚══════════════════════════════════════════╝\n");

    ownership_basico();
    ownership_y_scope();
    move_semantics();
    copy_types();
    ownership_en_funciones();
    ownership_con_retorno();
}

// ============================================================
// 1. Ownership Básico
// ============================================================
fn ownership_basico() {
    println!("=== 1. Ownership Básico ===");

    // String vive en el heap — tiene owner
    let saludo = String::from("Hola, Rust!");

    println!("Owner 'saludo': {}", saludo);
    println!("Longitud: {} bytes", saludo.len());
    println!("Capacidad: {} bytes", saludo.capacity());

    // Al salir de la función, `saludo` se destruye automáticamente
    // Rust llama drop(saludo) — equivalente a free() pero automático y garantizado

    println!();
}

// ============================================================
// 2. Ownership y Scope
// ============================================================
fn ownership_y_scope() {
    println!("=== 2. Ownership y Scope ===");

    // Los valores viven exactamente mientras dure su scope
    {
        let s = String::from("dentro del bloque");
        println!("'s' existe: {}", s);
    } // ← s se destruye aquí — memoria liberada inmediatamente

    // println!("{}", s);  // ❌ ERROR: s no existe fuera del bloque

    // Múltiples scopes anidados:
    {
        let exterior = String::from("exterior");
        {
            let interior = String::from("interior");
            println!("Ambos visibles aquí: {} y {}", exterior, interior);
        } // interior destruido
        println!("Solo exterior: {}", exterior);
    } // exterior destruido

    println!();
}

// ============================================================
// 3. Move Semantics
// ============================================================
fn move_semantics() {
    println!("=== 3. Move Semantics ===");

    let s1 = String::from("propietario original");
    println!("s1 tiene el valor: {}", s1);

    // Mover el ownership a s2
    let s2 = s1; // s1 ya no es válido
    println!("s2 ahora tiene el valor: {}", s2);

    // Intentar usar s1 aquí daría error de compilación:
    // println!("{}", s1);
    // ERROR: borrow of moved value: `s1`
    // value borrowed here after move

    println!("¿Por qué? Previene double-free:");
    println!("  → Solo s2 puede liberar la memoria");
    println!("  → Cuando s2 sale de scope, se libera UNA VEZ");

    // Clone para copiar explícitamente (costoso, pero explícito):
    let s3 = String::from("original");
    let s4 = s3.clone(); // Copia profunda — ambos son válidos
    println!("\nClone: s3='{}', s4='{}' (copia independiente)", s3, s4);

    println!();
}

// ============================================================
// 4. Copy Types — Tipos que se copian en lugar de moverse
// ============================================================
fn copy_types() {
    println!("=== 4. Copy Types ===");

    // Tipos en el stack implementan Copy — no hay costo de heap
    let x: i32 = 42;
    let y = x; // x se COPIA, no se mueve
    println!("x={}, y={} — ambos válidos después de la asignación", x, y);

    // Otros Copy types:
    let a: f64 = 3.14;
    let b = a;
    println!("f64: a={}, b={}", a, b);

    let flag1: bool = true;
    let flag2 = flag1;
    println!("bool: flag1={}, flag2={}", flag1, flag2);

    let c1: char = '🦀';
    let c2 = c1;
    println!("char: c1={}, c2={}", c1, c2);

    // Tuplas de Copy types también son Copy:
    let t1: (i32, f64) = (1, 2.0);
    let t2 = t1;
    println!("tupla: t1={:?}, t2={:?}", t1, t2);

    // String NO es Copy (heap-allocated, tamaño variable)
    // Vec<T>, HashMap, etc. tampoco son Copy

    println!();
}

// ============================================================
// 5. Ownership en Funciones
// ============================================================
fn ownership_en_funciones() {
    println!("=== 5. Ownership en Funciones ===");

    let s = String::from("hola");
    println!("Antes de llamar: s existe ({})", s);

    tomar_ownership(s); // s se mueve a la función

    // println!("{}", s);  // ❌ ERROR: s fue movido a la función

    println!("Después de llamar: s ya no es válido en este scope");

    // Con tipos Copy, la función recibe una copia:
    let n = 100i32;
    hacer_copia(n);
    println!("n sigue válido: {}", n); // OK — n se copió

    println!();
}

fn tomar_ownership(texto: String) {
    println!("  → Función recibió ownership de: {}", texto);
} // texto se destruye aquí

fn hacer_copia(numero: i32) {
    println!("  → Función recibió copia de: {}", numero);
} // numero (la copia) se destruye aquí — el original sigue

// ============================================================
// 6. Retornar Ownership
// ============================================================
fn ownership_con_retorno() {
    println!("=== 6. Retornar Ownership ===");

    // Una función puede devolver ownership al llamador
    let s1 = crear_string();
    println!("s1 recibió ownership: {}", s1);

    let s2 = String::from("datos");
    println!("s2 antes: {}", s2);

    let s3 = recibir_y_devolver(s2);
    // s2 ya no válido — ownership fue a s3 via la función
    println!("s3 recibió de vuelta: {}", s3);

    // Problema: tener que devolver ownership es verboso.
    // La solución elegante es Borrowing (Módulo 3).
    println!("\n→ Ver Módulo 3 para la solución: Borrowing & Referencias");

    println!();
}

fn crear_string() -> String {
    let s = String::from("string creado en función");
    s // El ownership se mueve al llamador
}

fn recibir_y_devolver(s: String) -> String {
    // Recibe ownership, lo usa, lo devuelve
    println!("  → Función usó: {}", s);
    s // Devuelve ownership
}

// ============================================================
// RESUMEN DE REGLAS DE OWNERSHIP
// ============================================================
// 1. Cada valor tiene exactamente un owner
// 2. Solo un owner a la vez
// 3. Cuando el owner sale de scope → drop() automático
//
// Consecuencias:
// ✅ No hay double-free (solo un owner puede destruir)
// ✅ No hay use-after-free (el compiler sabe cuándo es inválido)
// ✅ No hay memory leaks (drop garantizado al salir de scope)
// ✅ No hay GC overhead (determinístico, en compile time)
