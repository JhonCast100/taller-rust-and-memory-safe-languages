// 06_common_patterns/src/main.rs
//
// Patrones Idiomáticos en Rust — Escribir Código Seguro y Expresivo
// Ejecutar con: cargo run

use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Módulo 6: Patrones Idiomáticos         ║");
    println!("╚══════════════════════════════════════════╝\n");

    patron_option();
    patron_result();
    iteradores();
    pattern_matching();
    structs_y_enums();
}

// ============================================================
// 1. Option<T> — Valores Opcionales Sin Null
// ============================================================
fn patron_option() {
    println!("=== 1. Option<T> — Sin NullPointerException ===");

    // En lugar de null, Rust usa Option<T>:
    // enum Option<T> { Some(T), None }
    //
    // Debes manejar el caso None EXPLÍCITAMENTE — el compilador lo exige

    let algunos: Option<i32> = Some(42);
    let ninguno: Option<i32> = None;

    // Extraer el valor de forma segura:
    match algunos {
        Some(val) => println!("Tenemos: {}", val),
        None => println!("No hay valor"),
    }

    // unwrap_or — valor por defecto si es None:
    let valor = ninguno.unwrap_or(0);
    println!("unwrap_or: {}", valor);

    // map — transformar el valor si existe:
    let doble = algunos.map(|x| x * 2);
    println!("map(x*2): {:?}", doble);

    // and_then — encadenar operaciones opcionales:
    let resultado = algunos
        .map(|x| x + 8)
        .filter(|x| *x > 40)
        .map(|x| format!("Resultado: {}", x));
    println!("Cadena de operaciones: {:?}", resultado);

    // Caso real: buscar en un HashMap
    let mut mapa: HashMap<&str, i32> = HashMap::new();
    mapa.insert("clave1", 100);

    let encontrado = mapa.get("clave1");
    let no_encontrado = mapa.get("clave_inexistente");

    println!("Encontrado: {:?}", encontrado);
    println!("No encontrado: {:?}", no_encontrado);

    // if let — más ergonómico para un solo caso:
    if let Some(val) = mapa.get("clave1") {
        println!("if let: valor = {}", val);
    }

    println!();
}

// ============================================================
// 2. Result<T, E> — Manejo de Errores
// ============================================================
fn patron_result() {
    println!("=== 2. Result<T, E> — Errores Explícitos ===");

    // Result<T, E> reemplaza excepciones:
    // enum Result<T, E> { Ok(T), Err(E) }

    // Caso éxito:
    let ok: Result<i32, String> = Ok(42);
    let err: Result<i32, String> = Err(String::from("algo salió mal"));

    println!("Ok: {:?}", ok);
    println!("Err: {:?}", err);

    // Usar ?  para propagar errores:
    match parsear_y_doblar("21") {
        Ok(val) => println!("Parseado y doblado: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match parsear_y_doblar("no_es_numero") {
        Ok(val) => println!("Resultado: {}", val),
        Err(e) => println!("Error esperado: {}", e),
    }

    // map_err — transformar el error:
    let resultado: Result<i32, String> = "42"
        .parse::<i32>()
        .map_err(|e| format!("Error de parseo: {}", e));
    println!("map_err: {:?}", resultado);

    // unwrap_or_else — valor por defecto con closure:
    let seguro = "texto_invalido"
        .parse::<i32>()
        .unwrap_or_else(|_| -1);
    println!("unwrap_or_else: {}", seguro);

    println!();
}

fn parsear_y_doblar(s: &str) -> Result<i32, String> {
    // El operador `?` retorna el Err automáticamente si falla
    let numero: i32 = s.parse().map_err(|e| format!("No es número: {}", e))?;
    Ok(numero * 2)
}

// ============================================================
// 3. Iteradores — Procesamiento Funcional
// ============================================================
fn iteradores() {
    println!("=== 3. Iteradores ===");

    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — transformar cada elemento:
    let cuadrados: Vec<i32> = numeros.iter().map(|x| x * x).collect();
    println!("Cuadrados: {:?}", cuadrados);

    // filter — seleccionar elementos:
    let pares: Vec<&i32> = numeros.iter().filter(|x| *x % 2 == 0).collect();
    println!("Pares: {:?}", pares);

    // Encadenados — zero-cost abstractions:
    let resultado: i32 = numeros.iter()
        .filter(|&&x| x % 2 == 0) // Solo pares
        .map(|&x| x * x)           // Elevar al cuadrado
        .sum();                     // Sumar todo
    println!("Suma de cuadrados de pares: {}", resultado);

    // fold — reducir a un único valor:
    let producto: i32 = numeros.iter().fold(1, |acc, x| acc * x);
    println!("Producto (1..10): {}", producto);

    // enumerate — índice + valor:
    for (i, val) in numeros.iter().enumerate().take(3) {
        println!("  [{}] = {}", i, val);
    }

    // any / all:
    let hay_mayores_que_5 = numeros.iter().any(|&x| x > 5);
    let todos_positivos = numeros.iter().all(|&x| x > 0);
    println!("¿Alguno > 5? {}", hay_mayores_que_5);
    println!("¿Todos positivos? {}", todos_positivos);

    // Iteradores son LAZY — no calculan hasta collect/sum/etc.
    // Zero overhead: el compilador los convierte en loops eficientes

    println!();
}

// ============================================================
// 4. Pattern Matching Exhaustivo
// ============================================================
fn pattern_matching() {
    println!("=== 4. Pattern Matching ===");

    // match es exhaustivo — DEBES cubrir todos los casos
    let numero = 7i32;

    let descripcion = match numero {
        1 => "uno",
        2 | 3 | 5 | 7 | 11 => "número primo",
        n if n < 0 => "negativo",
        _ => "otro",
    };
    println!("{} es: {}", numero, descripcion);

    // Desestructurar tuplas:
    let punto = (3, -2);
    let cuadrante = match punto {
        (x, y) if x > 0 && y > 0 => "I",
        (x, y) if x < 0 && y > 0 => "II",
        (x, y) if x < 0 && y < 0 => "III",
        (x, y) if x > 0 && y < 0 => "IV",
        _ => "en eje",
    };
    println!("Punto {:?} está en cuadrante {}", punto, cuadrante);

    // Desestructurar structs:
    let punto3d = Punto3D { x: 1.0, y: 0.0, z: 5.0 };
    match punto3d {
        Punto3D { z: 0.0, .. } => println!("En el plano XY"),
        Punto3D { x, y, z } => println!("3D en ({}, {}, {})", x, y, z),
    }

    println!();
}

#[derive(Debug)]
struct Punto3D {
    x: f64,
    y: f64,
    z: f64,
}

// ============================================================
// 5. Structs y Enums Expresivos
// ============================================================
fn structs_y_enums() {
    println!("=== 5. Structs y Enums Expresivos ===");

    // Enum con datos — como suma algebraica de tipos:
    #[derive(Debug)]
    enum Forma {
        Circulo { radio: f64 },
        Rectangulo { ancho: f64, alto: f64 },
        Triangulo { base: f64, altura: f64 },
    }

    impl Forma {
        fn area(&self) -> f64 {
            match self {
                Forma::Circulo { radio } => std::f64::consts::PI * radio * radio,
                Forma::Rectangulo { ancho, alto } => ancho * alto,
                Forma::Triangulo { base, altura } => (base * altura) / 2.0,
            }
        }

        fn descripcion(&self) -> &str {
            match self {
                Forma::Circulo { .. } => "círculo",
                Forma::Rectangulo { .. } => "rectángulo",
                Forma::Triangulo { .. } => "triángulo",
            }
        }
    }

    let formas = vec![
        Forma::Circulo { radio: 3.0 },
        Forma::Rectangulo { ancho: 4.0, alto: 5.0 },
        Forma::Triangulo { base: 6.0, altura: 8.0 },
    ];

    let area_total: f64 = formas.iter().map(|f| {
        let area = f.area();
        println!("  {} → área: {:.2}", f.descripcion(), area);
        area
    }).sum();

    println!("Área total: {:.2}", area_total);
    println!();
}

// ============================================================
// RESUMEN: Patrones que hacen Rust seguro y expresivo
// ============================================================
//
// Option<T>     → Elimina null — el compilador te obliga a manejar None
// Result<T, E>  → Elimina excepciones ocultas — los errores son tipos
// Iteradores    → Expresivos y zero-cost (el compilador los optimiza)
// match         → Exhaustivo — imposible olvidar un caso
// Enums ricos   → Modelar estados del dominio con tipos, no flags/strings
//
// Juntos, estos patrones hacen que los bugs sean errores de compilación,
// no sorpresas en producción.
