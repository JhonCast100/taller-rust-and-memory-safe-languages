# 🦀 Rust y Lenguajes Seguros para la Memoria
## Workshop Completo — Memory Safety in Systems Programming

> *"El lenguaje de programación Rust está diseñado para garantizar la seguridad de la memoria en tiempo de compilación, ofreciendo sólidas garantías contra errores de programación comunes sin sacrificar el rendimiento."*  
> — Proyecto Rust, Documentación Oficial

---

## 📋 Descripción

Este taller explora los fundamentos de la **seguridad de memoria** en software moderno, con foco en Rust como lenguaje que resuelve problemas clásicos de C/C++ sin sacrificar rendimiento. Aprenderás los conceptos de **ownership**, **borrowing** y **lifetimes** a través de ejemplos prácticos y comparaciones con otros lenguajes.

---

## 🗂️ Estructura del Repositorio

```
rust-memory-safety-workshop/
├── README.md                          ← Este archivo
├── rust-memory-safety-workshop/
│   └── workshop_overview.md           ← Resumen de conceptos para la presentación
├── 01_memory_vulnerabilities/
│   ├── README.md
│   ├── buffer_overflow_c/
│   │   └── vulnerable.c               ← Ejemplo vulnerable en C
│   └── rust_safe_equivalent/
│       └── src/main.rs                ← Equivalente seguro en Rust
├── 02_ownership/
│   └── src/main.rs                    ← Fundamentos de ownership
├── 03_borrowing_and_references/
│   └── src/main.rs                    ← Borrowing y referencias
├── 04_lifetimes/
│   └── src/main.rs                    ← Lifetimes explícitos
├── 05_fearless_concurrency/
│   └── src/main.rs                    ← Concurrencia sin condiciones de carrera
├── 06_common_patterns/
│   └── src/main.rs                    ← Patrones idiomáticos en Rust
├── 07_comparison_languages/
│   ├── memory_java/
│   │   └── MemoryExample.java         ← GC en Java
│   ├── memory_go/
│   │   └── memory_example.go          ← GC en Go
│   └── memory_rust/
│       └── src/main.rs                ← Sin GC en Rust
└── 08_real_world_use_cases/
    └── src/main.rs                    ← Casos de uso reales
```

---

## 🎯 Objetivos de Aprendizaje

El objetivo de este taller es desarrollar las siguientes habilidades:

1. **Identificar** vulnerabilidades de memoria comunes (buffer overflow, dangling pointers, use-after-free)
2. **Comprender** el sistema de ownership de Rust y cómo previene errores en tiempo de compilación
3. **Aplicar** reglas de borrowing para escribir código seguro y eficiente
4. **Usar** lifetimes para controlar la validez de referencias
5. **Escribir** código concurrente libre de condiciones de carrera
6. **Comparar** el enfoque de Rust con GC de Java/Go y manejo manual de C/C++

---

## 🚀 Requisitos Previos

### Instalar Rust
```bash
# Windows
# Descargar rustup-init.exe desde https://rustup.rs

# Verificar instalación
rustc --version   # rustc 1.78.0 o superior
cargo --version
```

### Instalar MSYS2
```bash
# Windows
# Descargar MSYS2 desde https://www.msys2.org/

# Abrir MSYS2 UCRT64 y ejecutar
pacman -Syu

# Instalar GCC
pacman -S mingw-w64-ucrt-x86_64-gcc

# Verificar instalación
gcc --version
```

### Instalar Go
```bash
# Windows
# Descargar Go desde https://go.dev/dl/
```

### Instalar Java
```bash
# Windows
# Descargar JDK desde https://www.oracle.com/java/technologies/downloads/
```

### Herramientas Recomendadas
- **Visual Studio Code**

---

## 🏃 Cómo Ejecutar los Ejemplos

Generar el archivo `Cargo.toml` para cada sección con código Rust:

```bash
# Navegar a una sección
cd ejercicios\02_ownership\src

# Inicializar Rust
cargo init

# Compilar y ejecutar
cargo run

# Solo compilar (útil para ver errores del compilador)
cargo build

# Ejecutar con output detallado
RUST_BACKTRACE=1 cargo run
```

**Para los ejemplos en C:**


```bash
#Ejecutar
cd ejercicios\01_memory_vulnerabilities\buffer_overflow_c
gcc -o vulnerable vulnerable.c
.\vulnerable.exe
```

**Para los ejemplos en Go:**

```bash
# Ejecutar
cd ejercicios\07_comparison_languages\memory_go
go run memory_example.go
```

**Para los ejemplos en Java:**

```bash
# Ejecutar
cd ejercicios\07_comparison_languages\memory_java
javac MemoryExample.java
java MemoryExample
```

---

## 📚 Módulos del Taller

### Módulo 1 — El Problema: Vulnerabilidades de Memoria
Exploración de errores clásicos en C: buffer overflow, dangling pointers, use-after-free y condiciones de carrera. Ver código real vulnerable y sus consecuencias.

### Módulo 2 — Ownership: El Corazón de Rust
El sistema de propiedad que garantiza que cada valor tiene exactamente un dueño. Cuando el dueño sale de scope, el valor se destruye automáticamente.

### Módulo 3 — Borrowing y Referencias
Cómo prestar acceso a datos sin transferir propiedad. Las reglas del borrow checker: máximo una referencia mutable O múltiples referencias inmutables.

### Módulo 4 — Lifetimes
Anotaciones que le dicen al compilador cuánto tiempo viven las referencias. Previenen dangling pointers en tiempo de compilación.

### Módulo 5 — Concurrencia sin Miedo
El sistema de tipos de Rust hace imposible las condiciones de carrera. `Send` y `Sync` garantizan seguridad entre hilos.

### Módulo 6 — Patrones Idiomáticos
`Option<T>`, `Result<T, E>`, iteradores, y otros patrones que hacen el código Rust seguro y expresivo.

### Módulo 7 — Comparación con Java y Go
Análisis del enfoque de Garbage Collection vs. ownership. Ventajas y trade-offs de cada modelo.

### Módulo 8 — Rust en Producción
Casos reales: WebAssembly, sistemas operativos, blockchain, herramientas CLI.

---

## 📊 Comparativa Rápida

| Característica | C/C++ | Java | Go | Rust |
|---|---|---|---|---|
| Gestión de memoria | Manual | GC | GC | Ownership |
| Seguridad de memoria | ❌ Manual | ✅ GC | ✅ GC | ✅ Compilador |
| Sin GC | ✅ | ❌ | ❌ | ✅ |
| Rendimiento | ⚡ Máximo | 🔶 Bueno | 🔶 Bueno | ⚡ Máximo |
| Seguridad por defecto | ❌ | 🔶 | 🔶 | ✅ |
| Concurrencia segura | ❌ | 🔶 | 🔶 | ✅ |

---

## 🌍 Rust en el Mundo Real

- **Linux Kernel** — Componentes escritos en Rust desde v6.1
- **Android** — Nuevo código del sistema preferentemente en Rust (Google)
- **Windows** — Microsoft reescribiendo componentes críticos en Rust
- **WebAssembly** — Rust es el lenguaje más popular para WASM
- **Blockchain** — Solana, Polkadot y otros ecosistemas usan Rust
- **Herramientas CLI** — `ripgrep`, `bat`, `exa`, `fd`, `starship`

---

## 📖 Recursos Adicionales

- 📘 [The Rust Programming Language (El libro)](https://doc.rust-lang.org/book/)
- 🎮 [Rustlings — Ejercicios interactivos](https://github.com/rust-lang/rustlings)
- 🔬 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 📦 [crates.io — Ecosystem de paquetes](https://crates.io)
- 💬 [Rust Users Forum](https://users.rust-lang.org)
- 🎓 [Microsoft Rust Training](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)

---

*Explorando la seguridad de memoria en sistemas modernos.*  