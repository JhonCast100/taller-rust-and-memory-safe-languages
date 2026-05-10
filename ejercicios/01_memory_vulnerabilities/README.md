# Módulo 1 — Vulnerabilidades de Memoria

## ¿Qué es la Seguridad de Memoria?

La seguridad de memoria garantiza que los programas **solo accedan a ubicaciones de memoria autorizadas**. Cuando esto falla, surgen vulnerabilidades críticas que pueden causar crashes, corrupción de datos o ejecución de código malicioso.

Según estudios de Microsoft y Google, **~70% de las vulnerabilidades críticas de seguridad** en software de sistemas son causadas por errores de manejo de memoria.

---

## Vulnerabilidades Comunes

### 1. Buffer Overflow (Desbordamiento de Búfer)
Se escribe más datos de los que caben en un búfer, corrompiendo memoria adyacente.

```c
char buffer[10];
strcpy(buffer, "Este string es demasiado largo!");  // 💥 Escribe fuera del búfer
```

**Consecuencias:** Corrupción de stack, ejecución de código arbitrario (base de exploits clásicos como stack smashing).

---

### 2. Dangling Pointer (Puntero Colgante)
Se accede a memoria que ya fue liberada.

```c
int *ptr = malloc(sizeof(int));
*ptr = 42;
free(ptr);
printf("%d\n", *ptr);  // 💥 Comportamiento indefinido — ptr cuelga en el vacío
```

**Consecuencias:** Comportamiento impredecible, crashes, vulnerabilidades de seguridad.

---

### 3. Use-After-Free
Similar al dangling pointer, pero la memoria liberada es reutilizada por el sistema y luego corrompida.

```c
int *a = malloc(sizeof(int));
free(a);
int *b = malloc(sizeof(int));  // b puede apuntar al mismo bloque que a
*a = 9999;                     // 💥 Corrompe datos de b
printf("%d\n", *b);            // Imprime 9999 en lugar del valor de b
```

**Consecuencias:** Explotable para ejecución de código arbitrario (CVE-2021-30551 en Chrome, por ejemplo).

---

### 4. Double Free
Liberar la misma memoria dos veces.

```c
int *ptr = malloc(sizeof(int));
free(ptr);
free(ptr);  // 💥 Comportamiento indefinido — heap corruption
```

**Consecuencias:** Corrupción del heap, crash, posible explotación.

---

### 5. Race Condition (Condición de Carrera)
Dos hilos acceden a datos compartidos sin sincronización.

```c
int contador = 0;  // Variable compartida

// Hilo 1:          // Hilo 2:
contador++;         contador++;
// Ambos leen 0, ambos escriben 1 → resultado final: 1 en lugar de 2
```

**Consecuencias:** Estado de datos inconsistente, bugs difíciles de reproducir y debuggear.

---

## Archivos en Este Módulo

- `buffer_overflow_c/vulnerable.c` — Ejemplo real de buffer overflow en C
- `rust_safe_equivalent/src/main.rs` — El mismo programa, seguro por diseño en Rust

## Cómo Ejecutar

```bash
# Ejemplo C (requiere gcc)
cd buffer_overflow_c
gcc -o vulnerable vulnerable.c -fno-stack-protector  # Desactivar protecciones para ver el efecto
./vulnerable

# Ejemplo Rust
cd ../rust_safe_equivalent
cargo run
```

> ⚠️ **Nota:** El ejemplo C con `-fno-stack-protector` desactiva protecciones del compilador para ilustrar la vulnerabilidad. **Nunca usar en producción.**
