// rust_safe_equivalent/src/main.rs
//
// Equivalente seguro en Rust de los ejemplos vulnerables en C.
// El compilador de Rust RECHAZA todos los patrones peligrosos.
//
// Ejecutar con: cargo run

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   Equivalentes Seguros en Rust — Demo        ║");
    println!("╚══════════════════════════════════════════════╝\n");

    demo_no_buffer_overflow();
    demo_no_dangling_pointer();
    demo_no_use_after_free();
    demo_no_double_free();
    demo_no_memory_leak();
}

// ============================================================
// EJEMPLO 1: Sin Buffer Overflow
// ============================================================
fn demo_no_buffer_overflow() {
    println!("--- EJEMPLO 1: Sin Buffer Overflow ---");

    // Vec<u8> crece dinámicamente — nunca desborda
    let mut buffer: Vec<u8> = Vec::new();
    let datos = b"Este string es MUCHO mas largo que cualquier buffer fijo!";

    // push_str, extend, etc. siempre verifican capacidad
    buffer.extend_from_slice(datos);

    println!("Buffer creció a {} bytes automáticamente", buffer.len());
    println!("Contenido: {}", String::from_utf8_lossy(&buffer));

    // Arrays de tamaño fijo también son seguros:
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // arr[10]  ← Esto compilaría, pero paniquearía en runtime con un mensaje claro:
    //            "index out of bounds: the len is 5 but the index is 10"
    //            NO hay corrupción silenciosa de memoria.

    // Acceso seguro con get():
    match arr.get(10) {
        Some(val) => println!("Valor: {}", val),
        None => println!("Índice fuera de rango — manejado con seguridad"),
    }

    println!();
}

// ============================================================
// EJEMPLO 2: Sin Dangling Pointer
// ============================================================
fn demo_no_dangling_pointer() {
    println!("--- EJEMPLO 2: Sin Dangling Pointer ---");

    // En Rust no existe free() manual.
    // El valor se destruye automáticamente cuando sale de scope.
    {
        let valor = Box::new(42); // Heap allocation
        println!("Valor dentro del scope: {}", valor);
    } // ← `valor` se destruye AQUÍ automáticamente (Drop trait)

    // Intentar usar `valor` fuera del scope es un ERROR DE COMPILACIÓN:
    // println!("{}", valor);  // ERROR: borrow of moved value: `valor`

    // El compilador garantiza que las referencias siempre son válidas:
    let referencia: &i32;
    let numero = 100;
    referencia = &numero; // OK: numero vive más que referencia

    println!("Referencia válida: {}", referencia);

    // Esto NO compila — dangling reference detectado en compile-time:
    // referencia = {
    //     let temporal = 50;
    //     &temporal  // ERROR: `temporal` no vive suficiente
    // };

    println!("Nunca hay dangling pointers — el compilador lo garantiza.\n");
}

// ============================================================
// EJEMPLO 3: Sin Use-After-Free
// ============================================================
fn demo_no_use_after_free() {
    println!("--- EJEMPLO 3: Sin Use-After-Free ---");

    let a = Box::new(100i32);
    println!("a = {}", a);

    // En Rust, "mover" un valor transfiere la propiedad:
    let b = a; // `a` ya no es válido. Ownership transferido a `b`.
    println!("b = {} (recibió ownership de a)", b);

    // Intentar usar `a` después del move es ERROR DE COMPILACIÓN:
    // println!("a = {}", a);  // ERROR: borrow of moved value: `a`

    // b se destruye al final de la función.
    // No hay forma de acceder a memoria liberada.

    println!("Use-after-free es imposible — el compilador lo previene.\n");
}

// ============================================================
// EJEMPLO 4: Sin Double Free
// ============================================================
fn demo_no_double_free() {
    println!("--- EJEMPLO 4: Sin Double Free ---");

    let valor = Box::new(77i32);
    println!("Valor: {}", valor);

    // No existe free() explícito en Rust safe code.
    // El valor se libera automáticamente una sola vez al salir del scope.
    // Es imposible llamar "free" dos veces sobre el mismo valor.

    // La propiedad única (ownership) garantiza esto:
    // - Solo UN dueño puede destruir el valor
    // - Cuando el dueño sale de scope, Rust llama Drop exactamente UNA vez

    drop(valor); // Podemos destruir explícitamente...
    // drop(valor); // ERROR DE COMPILACIÓN: use of moved value: `valor`

    println!("Double free es imposible en Rust safe code.");
    println!("El sistema de ownership garantiza un único drop.\n");
}

// ============================================================
// EJEMPLO 5: Sin Memory Leaks (en la práctica)
// ============================================================
fn demo_no_memory_leak() {
    println!("--- EJEMPLO 5: Sin Memory Leaks ---");

    for i in 0..5 {
        // Vec se destruye al final de cada iteración automáticamente
        let datos: Vec<i32> = vec![0; 1024]; // 4 KB
        println!("Asignados {} elementos (iteración {}) — se liberarán al final del scope", datos.len(), i);
    } // ← Vec destruido aquí, memoria liberada, sin leak

    println!("Memoria liberada automáticamente en cada iteración.");
    println!("RAII (Resource Acquisition Is Initialization) garantiza esto.");
    println!();

    // Nota: Rust puede tener memory leaks en casos específicos (Rc cycles),
    // pero NO como comportamiento accidental — requiere esfuerzo explícito.
    // Para esos casos existe: Weak<T>, RefCell, etc.
}

// ============================================================
// BONUS: Lo que el compilador rechaza
// ============================================================
// El siguiente código NO COMPILA. Está comentado para que puedas
// intentar descomentarlo y ver los mensajes de error del compilador:
//
// fn intentar_dangling_reference() -> &i32 {
//     let x = 5;
//     &x  // ERROR: cannot return reference to local variable `x`
//         // `x` no vive suficiente — sería un dangling pointer
// }
//
// fn intentar_doble_owner() {
//     let s1 = String::from("hola");
//     let s2 = s1;        // s1 movido a s2
//     println!("{}", s1); // ERROR: borrow of moved value: `s1`
// }
//
// fn intentar_mutacion_mientras_prestado() {
//     let mut v = vec![1, 2, 3];
//     let ref_primer = &v[0];    // referencia inmutable
//     v.push(4);                  // ERROR: cannot borrow `v` as mutable
//                                 // because it is also borrowed as immutable
//     println!("{}", ref_primer);
// }
