// 03_borrowing_and_references/src/main.rs
//
// Borrowing y Referencias en Rust
// Ejecutar con: cargo run

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 3: Borrowing y Referencias      ║");
    println!("╚══════════════════════════════════════════╝\n");

    referencias_inmutables();
    referencias_mutables();
    reglas_del_borrow_checker();
    slices();
}

// ============================================================
// 1. Referencias Inmutables — &T
// ============================================================
fn referencias_inmutables() {
    println!("=== 1. Referencias Inmutables (&T) ===");

    let s = String::from("hola mundo");

    // Prestar s a la función — NO se mueve el ownership
    let longitud = calcular_longitud(&s);

    // s sigue siendo válido porque solo prestamos, no movemos
    println!("'{}' tiene {} caracteres", s, longitud);
    println!("s sigue siendo válido: ✅");

    // Múltiples referencias inmutables — OK:
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("Tres referencias al mismo valor: {}, {}, {}", r1, r2, r3);
    // ✅ Lectura concurrente es siempre segura — no hay modificaciones

    println!();
}

fn calcular_longitud(s: &String) -> usize {
    // s es una referencia — NO tiene ownership
    s.len()
    // s sale de scope pero NO se destruye la String — solo la referencia
}

// ============================================================
// 2. Referencias Mutables — &mut T
// ============================================================
fn referencias_mutables() {
    println!("=== 2. Referencias Mutables (&mut T) ===");

    let mut s = String::from("hola");
    println!("Antes: {}", s);

    agregar_texto(&mut s); // Préstamo mutable
    println!("Después: {}", s);

    // REGLA CLAVE: Solo una referencia mutable a la vez
    let r1 = &mut s;
    // let r2 = &mut s;  // ❌ ERROR: solo puede haber UN &mut a la vez
    //                   // Previene data races en compile time!
    r1.push_str(" — modificado");
    println!("Modificado via ref mutable: {}", s);

    // No se puede mezclar referencias mutables e inmutables:
    let s2 = String::from("ejemplo");
    let ref_inm = &s2;
    // let ref_mut = &mut s2;  // ❌ ERROR: no puede haber &mut mientras existe &
    //                         // porque ref_inm asumiría que el valor no cambia
    println!("ref_inm: {}", ref_inm); // OK

    println!();
}

fn agregar_texto(s: &mut String) {
    s.push_str(", mundo");
    println!("  → Función modificó via &mut: {}", s);
}

// ============================================================
// 3. Reglas del Borrow Checker
// ============================================================
fn reglas_del_borrow_checker() {
    println!("=== 3. Reglas del Borrow Checker ===");

    println!("Las dos reglas fundamentales del borrowing:");
    println!("  1. CUALQUIER cantidad de referencias inmutables (&T)");
    println!("  2. O EXACTAMENTE UNA referencia mutable (&mut T)");
    println!("  → Nunca ambas al mismo tiempo\n");

    // Demostración de Non-Lexical Lifetimes (NLL)
    // El borrow checker es INTELIGENTE — el préstamo termina cuando
    // la referencia deja de USARSE, no cuando sale del scope léxico:

    let mut v = vec![1, 2, 3, 4, 5];

    // Scope del borrow inmutable termina en la última línea que lo usa:
    {
        let primer = &v[0]; // borrow inmutable
        println!("Primer elemento: {}", primer);
    } // primer sale de scope — borrow termina

    v.push(6); // ✅ OK — borrow inmutable ya terminó
    println!("Vector después de push: {:?}", v);

    // Otro ejemplo de NLL:
    let mut texto = String::from("inicio");
    let r = &texto;
    println!("r = {}", r); // Último uso de r
    // r ya no se usa — borrow termina aquí (NLL)
    texto.push_str(" — fin"); // ✅ OK
    println!("texto = {}", texto);

    println!();
}

// ============================================================
// 4. Slices — Referencias a Porciones
// ============================================================
fn slices() {
    println!("=== 4. Slices ===");

    // String slices — referencia a una porción de un String
    let s = String::from("hola mundo");

    let hola = &s[0..4];  // "hola"
    let mundo = &s[5..10]; // "mundo"

    println!("String completo: {}", s);
    println!("Slice 1: '{}'", hola);
    println!("Slice 2: '{}'", mundo);

    // String literals son slices:
    let literal: &str = "soy un string literal";
    println!("Literal es un &str: {}", literal);

    // Función que funciona con &str (más flexible):
    let primera = primera_palabra(&s);
    println!("Primera palabra: '{}'", primera);

    // Slices de arrays:
    let numeros = [10, 20, 30, 40, 50];
    let mitad: &[i32] = &numeros[1..4]; // [20, 30, 40]
    println!("Slice de array: {:?}", mitad);

    // La magia de los slices:
    // Si s se modifica mientras existe un slice, ERROR DE COMPILACIÓN:
    // let slice = &s[0..4];
    // s.clear();           // ❌ ERROR: s es borrowed como inmutable
    // println!("{}", slice);

    println!();
}

fn primera_palabra(s: &str) -> &str {
    // Encuentra la primera palabra usando bytes
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i]; // Retorna slice hasta el primer espacio
        }
    }

    &s[..] // Si no hay espacio, toda la string es una palabra
}

// ============================================================
// RESUMEN: Reglas de Borrowing
// ============================================================
//
// En cualquier momento dado, puedes tener:
//   - UNA o más referencias inmutables: &T  (lectores)
//   - Exactamente UNA referencia mutable: &mut T  (escritor)
//   - NUNCA ambas al mismo tiempo
//
// Esto hace Data Races IMPOSIBLES en compile time:
//   - Data race = dos punteros acceden al mismo dato,
//     al menos uno escribe, sin sincronización
//   - Las reglas de Rust eliminan exactamente esta condición
//
// No hay dangling references:
//   - El compilador garantiza que las referencias viven
//     menos que el dato al que apuntan (lifetimes)
