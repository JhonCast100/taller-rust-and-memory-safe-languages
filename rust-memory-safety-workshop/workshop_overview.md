# Workshop Overview — Rust y Lenguajes Seguros para la Memoria

> Material de referencia para la presentación. Cada sección tiene código ejecutable en su carpeta correspondiente.

---

## Diapositiva 1 — El Desafío

**El problema:**
- C/C++: potencia máxima, pero gestión manual de memoria
- 70% de vulnerabilidades críticas son errores de memoria (Microsoft, Google)
- Los errores de memoria causan crashes, exploits y pérdidas millonarias

**Ejemplos reales:**
- Heartbleed (OpenSSL, 2014) — buffer over-read en C
- WannaCry (2017) — use-after-free en SMB de Windows
- Log4Shell (2021) — no de memoria, pero mismo patrón de descuido

---

## Diapositiva 2 — Vulnerabilidades Comunes

| Vulnerabilidad | Descripción | Consecuencia |
|---|---|---|
| Buffer Overflow | Escribir fuera del búfer | Corrupción de stack/heap |
| Dangling Pointer | Acceder memoria liberada | Comportamiento indefinido |
| Use-After-Free | Usar memoria re-asignada | Ejecución de código arbitrario |
| Double Free | Liberar dos veces | Corrupción del heap |
| Race Condition | Acceso concurrente no sync | Datos inconsistentes |

**Ver código:** `01_memory_vulnerabilities/`

---

## Diapositiva 3 — La Solución: Ownership

**Las 3 reglas de Rust:**
1. Cada valor tiene exactamente **un** dueño
2. Solo puede haber **un** dueño a la vez
3. Cuando el dueño sale de scope → el valor **se destruye**

```rust
{
    let s = String::from("hola");  // s es el owner
    // ... usar s ...
}  // s destruido AQUÍ — automático, sin free(), sin GC
```

**Ver código:** `02_ownership/`

---

## Diapositiva 4 — Borrowing: Prestar sin Perder

**La regla de oro:**
- Cualquier cantidad de `&T` (referencias inmutables — lectores)
- O exactamente una `&mut T` (referencia mutable — escritor)
- **Nunca ambas al mismo tiempo**

```rust
let s = String::from("hola");
let longitud = calcular_longitud(&s);  // Presta, no mueve
println!("{}", s);  // ✅ s sigue válido
```

**Ver código:** `03_borrowing_and_references/`

---

## Diapositiva 5 — Lifetimes: Válido por Diseño

```rust
fn el_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// El compilador garantiza que el resultado vive suficiente
// Dangling pointers: IMPOSIBLES por diseño
```

**Ver código:** `04_lifetimes/`

---

## Diapositiva 6 — Fearless Concurrency

```rust
use std::sync::{Arc, Mutex};

let contador = Arc::new(Mutex::new(0));
// Compartir entre hilos con garantías en COMPILE TIME
// Race conditions: detectadas por el compilador, no en runtime
```

**Ver código:** `05_fearless_concurrency/`

---

## Diapositiva 7 — Comparativa

| | C/C++ | Java | Go | **Rust** |
|---|---|---|---|---|
| Rendimiento | ⚡ Máximo | 🔶 | 🔶 | ⚡ **Máximo** |
| Memory Safety | ❌ Manual | ✅ GC | ✅ GC | ✅ **Compilador** |
| Sin GC | ✅ | ❌ | ❌ | ✅ |
| Null Safety | ❌ | ⚠️ | ⚠️ nil | ✅ Option<T> |
| Race Conditions | ❌ | ⚠️ | ⚠️ | ✅ **Imposibles** |

**Ver código:** `07_comparison_languages/`

---

## Diapositiva 8 — Rust en Producción

- 🐧 **Linux Kernel** — código Rust oficial desde v6.1
- 🤖 **Android** — nuevo código del sistema en Rust (Google)
- 🪟 **Windows** — componentes críticos migrados a Rust (Microsoft)
- 🌐 **WebAssembly** — lenguaje más popular para WASM
- ⛓️ **Blockchain** — Solana, Polkadot, Near Protocol
- 🛠️ **CLI Tools** — ripgrep, bat, fd, starship
- ☁️ **Cloud** — AWS Firecracker, Cloudflare Pingora

**8 años consecutivos: lenguaje más amado (Stack Overflow 2016-2024)**

**Ver código:** `08_real_world_use_cases/`

---

## Diapositiva 9 — Desafíos Honestos

| Desafío | Realidad |
|---|---|
| Curva de aprendizaje | Alta al inicio — el borrow checker requiere nuevo mindset |
| Tiempos de compilación | Más lentos que Go/Java — el compilador hace más trabajo |
| Ecosistema | En crecimiento — maduro en sistemas, creciendo en web |
| Expresividad | Muy alta una vez dominado |

**La inversión en aprender Rust paga dividendos en seguridad y rendimiento.**

---

## Diapositiva 10 — El Futuro

> *"Los lenguajes que garantizan la seguridad de la memoria son fundamentales para construir sistemas robustos, seguros y de alto rendimiento en un mundo cada vez más interconectado."*

**Tendencia clara:** Los grandes actores (DARPA, NSA, Casa Blanca en EEUU, Linux Foundation) recomiendan activamente lenguajes memory-safe como Rust para software de sistemas.

Rust no es solo una moda — es un cambio de paradigma que está redefiniendo cómo se escribe software de sistemas seguro.

---

*Repositorio: Todos los conceptos tienen código ejecutable en sus respectivas carpetas.*
