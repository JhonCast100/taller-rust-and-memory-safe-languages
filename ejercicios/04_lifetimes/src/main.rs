// 04_lifetimes/src/main.rs
//
// Lifetimes (Tiempos de Vida) en Rust
// Ejecutar con: cargo run
//
// Los lifetimes son la forma de Rust de garantizar que las referencias
// sean siempre válidas. Son anotaciones que el compilador usa para
// verificar que no hay dangling references.

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 4: Lifetimes                    ║");
    println!("╚══════════════════════════════════════════╝\n");

    lifetimes_implicitos();
    lifetimes_explicitos();
    lifetime_en_structs();
    lifetime_static();
}

// ============================================================
// 1. Lifetimes Implícitos (Lifetime Elision)
// ============================================================
fn lifetimes_implicitos() {
    println!("=== 1. Lifetimes Implícitos ===");

    // El compilador infiere lifetimes simples automáticamente
    // (lifetime elision rules — Rust aplica reglas para no requerir anotaciones)

    let s = String::from("hola");
    let longitud = obtener_longitud(&s); // El compilador infiere: la ref vive menos que s
    println!("'{}' tiene longitud: {}", s, longitud);

    // Equivalente explícito (el compilador hace esto internamente):
    // fn obtener_longitud<'a>(s: &'a String) -> usize { s.len() }

    println!("El compilador infirió los lifetimes automáticamente ✅\n");
}

fn obtener_longitud(s: &String) -> usize {
    s.len()
}

// ============================================================
// 2. Lifetimes Explícitos — cuando el compilador necesita ayuda
// ============================================================
fn lifetimes_explicitos() {
    println!("=== 2. Lifetimes Explícitos ===");

    // Esta función necesita lifetime explícito porque retorna una referencia
    // y el compilador no puede inferir de cuál parámetro viene:

    let string1 = String::from("string largo es largo");
    let resultado;

    {
        let string2 = String::from("xy");
        resultado = el_mas_largo(string1.as_str(), string2.as_str());
        println!("El más largo: '{}'", resultado);
        // resultado puede usarse aquí porque string2 aún existe
    }
    // No podríamos usar resultado aquí si referenciara string2
    // (que ya salió de scope) — el compilador lo detectaría

    // Ejemplo con distintos lifetimes:
    let cadena1 = String::from("larga cadena es larga");
    let cadena2 = String::from("xy");
    let res = el_mas_largo(&cadena1, &cadena2);
    println!("Más largo entre '{}' y '{}': '{}'", cadena1, cadena2, res);

    println!();
}

// <'a> es un parámetro de lifetime
// El resultado vive al menos tanto como el más corto de x e y
fn el_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Esta función NO necesita lifetime anotado — siempre retorna el primero:
// fn siempre_primero<'a>(x: &'a str, _y: &str) -> &'a str { x }

// Esta TAMPOCO — retorna un string owned, no una referencia:
// fn combinar(x: &str, y: &str) -> String { format!("{} {}", x, y) }

// ============================================================
// 3. Lifetimes en Structs
// ============================================================
fn lifetime_en_structs() {
    println!("=== 3. Lifetimes en Structs ===");

    // Un struct que contiene una referencia necesita lifetime:
    // (la instancia NO puede outlive la referencia que contiene)

    let novela = String::from("Call me Ishmael. Some years ago...");

    let primera_oracion;
    {
        // NombreParte vive menos que novela — ✅ OK
        let parte = NombreParte {
            parte: primera_oracion_de(&novela),
        };
        primera_oracion = parte.parte;
        println!("Primera oración: '{}'", primera_oracion);
    }

    // Si NombreParte outlive novela, sería un dangling reference
    // El compilador lo previene con el lifetime 'a

    // Struct con lifetime para análisis de texto:
    let texto = String::from("Rust es genial para sistemas");
    let analizador = Analizador {
        contenido: &texto,
    };
    println!("Palabras: {}", analizador.contar_palabras());
    println!("Primera palabra: {}", analizador.primera_palabra());

    println!();
}

// Struct que contiene una referencia — requiere lifetime anotation
#[derive(Debug)]
struct NombreParte<'a> {
    parte: &'a str, // Esta referencia debe vivir al menos tanto como el struct
}

fn primera_oracion_de(texto: &str) -> &str {
    match texto.find('.') {
        Some(i) => &texto[..=i],
        None => texto,
    }
}

struct Analizador<'a> {
    contenido: &'a str,
}

impl<'a> Analizador<'a> {
    fn contar_palabras(&self) -> usize {
        self.contenido.split_whitespace().count()
    }

    fn primera_palabra(&self) -> &str {
        self.contenido
            .split_whitespace()
            .next()
            .unwrap_or("")
    }
}

// ============================================================
// 4. 'static — El lifetime más largo
// ============================================================
fn lifetime_static() {
    println!("=== 4. Lifetime 'static ===");

    // 'static significa que la referencia puede vivir todo el programa
    // Los string literals son 'static — están en el binario

    let s: &'static str = "Soy un string literal — vivo todo el programa";
    println!("{}", s);

    // Las constantes también son 'static:
    const MAX_PUNTOS: u32 = 100_000;
    println!("MAX_PUNTOS (static): {}", MAX_PUNTOS);

    // Cuidado: 'static no siempre es la solución correcta.
    // A veces el compilador sugiere 'static cuando el real problema
    // es que la referencia no vive suficiente.
    // La solución correcta suele ser ajustar los lifetimes, no usar 'static.

    println!("\nResumen de lifetimes:");
    println!("  'a, 'b, etc. → lifetimes genéricos (inferidos o anotados)");
    println!("  'static      → vive todo el programa");
    println!("  La mayoría del tiempo, el compilador los infiere ✅");

    println!();
}

// ============================================================
// EJEMPLO FINAL: Juntando todo
// ============================================================
// Esta función usa lifetimes + generics + trait bounds:
fn el_mas_largo_con_anuncio<'a, T>(
    x: &'a str,
    y: &'a str,
    anuncio: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Atención: {}", anuncio);
    if x.len() > y.len() { x } else { y }
}

// ============================================================
// RESUMEN DE LIFETIMES
// ============================================================
//
// Los lifetimes responden a: "¿Por cuánto tiempo es válida esta referencia?"
//
// Garantías que proveen:
// ✅ Ninguna referencia puede outlive el dato al que apunta
// ✅ No hay dangling pointers — detectado en compile time
// ✅ No hay costo en runtime — solo anotaciones para el compilador
//
// La mayoría del tiempo NO necesitas anotarlos (lifetime elision).
// Solo cuando el compilador no puede inferir el lifetime solo.
