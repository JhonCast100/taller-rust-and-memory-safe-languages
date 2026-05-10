/*
 * vulnerable.c — Demostración de Vulnerabilidades de Memoria en C
 *
 * ADVERTENCIA: Este código es intencionalmente INSEGURO para fines educativos.
 * Nunca escribir código así en producción.
 *
 * Compile con: gcc -o vulnerable vulnerable.c -fno-stack-protector
 * Ejecutar con: ./vulnerable
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ============================================================
 * EJEMPLO 1: Buffer Overflow
 * ============================================================ */
void demo_buffer_overflow() {
    printf("\n--- EJEMPLO 1: Buffer Overflow ---\n");

    char buffer[10];  // Solo 10 bytes de capacidad
    char *input = "Este_string_es_MUCHO_mas_largo_que_el_buffer!";

    printf("Tamaño del buffer: %zu bytes\n", sizeof(buffer));
    printf("Tamaño del input:  %zu bytes\n", strlen(input));
    printf("Copiando con strcpy (INSEGURO)...\n");

    // strcpy NO verifica el tamaño — escribe más allá del buffer
    // Esto puede corromper el stack y otras variables locales
    strcpy(buffer, input);  // ← VULNERABILIDAD: desbordamiento de buffer

    printf("Resultado (si no crashea): %s\n", buffer);
    // En muchos sistemas esto causa Segmentation Fault
    // En otros, silenciosamente corrompe memoria adyacente
}

/* ============================================================
 * EJEMPLO 2: Dangling Pointer (Puntero Colgante)
 * ============================================================ */
void demo_dangling_pointer() {
    printf("\n--- EJEMPLO 2: Dangling Pointer ---\n");

    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == NULL) {
        fprintf(stderr, "Error: malloc falló\n");
        return;
    }

    *ptr = 42;
    printf("Valor antes de free: %d\n", *ptr);

    free(ptr);  // La memoria fue devuelta al sistema operativo
    // ptr sigue apuntando a esa dirección — es un "dangling pointer"

    // Acceder a ptr ahora es Comportamiento Indefinido (Undefined Behavior)
    // Puede: retornar basura, crashear, o ejecutar código arbitrario
    printf("Valor DESPUÉS de free (UB!): %d\n", *ptr);  // ← VULNERABILIDAD
    // No hacer esto en código real. El compilador puede reordenar o
    // eliminar esto de formas inesperadas.
}

/* ============================================================
 * EJEMPLO 3: Use-After-Free
 * ============================================================ */
void demo_use_after_free() {
    printf("\n--- EJEMPLO 3: Use-After-Free ---\n");

    int *a = (int*)malloc(sizeof(int));
    *a = 100;
    printf("a = %d (en dirección %p)\n", *a, (void*)a);

    free(a);  // Liberamos la memoria

    // El sistema puede reasignar INMEDIATAMENTE ese bloque
    int *b = (int*)malloc(sizeof(int));
    *b = 200;
    printf("b = %d (en dirección %p)\n", *b, (void*)b);

    // Si a y b apuntan a la misma dirección (probable en este caso):
    *a = 9999;  // ← VULNERABILIDAD: modifica silenciosamente los datos de b!
    printf("b después del write a través de 'a' (UB!): %d\n", *b);
    // Esperaríamos 200, pero puede ser 9999

    free(b);
    // NUNCA llamar free(a) de nuevo — double free!
}

/* ============================================================
 * EJEMPLO 4: Double Free
 * ============================================================ */
void demo_double_free() {
    printf("\n--- EJEMPLO 4: Double Free ---\n");

    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 77;
    printf("Valor: %d\n", *ptr);

    free(ptr);
    printf("Primera liberación: OK\n");

    // Segunda liberación de la misma memoria — corrupción del heap
    // En la práctica esto suele causar abort() o segfault
    // Comentado para que el programa no crashee en el demo:
    // free(ptr);  // ← VULNERABILIDAD: double free
    printf("Segunda liberación: OMITIDA (causaría crash/heap corruption)\n");
    printf("En un exploit real, double-free puede dar control del heap al atacante.\n");
}

/* ============================================================
 * EJEMPLO 5: Memory Leak (Fuga de Memoria)
 * ============================================================ */
void demo_memory_leak() {
    printf("\n--- EJEMPLO 5: Memory Leak ---\n");

    for (int i = 0; i < 5; i++) {
        // Cada iteración asigna memoria pero NUNCA la libera
        int *leak = (int*)malloc(1024 * sizeof(int));  // 4 KB por iteración
        if (leak == NULL) continue;
        leak[0] = i;
        printf("Asignados 4 KB (iteración %d) — nunca se liberarán\n", i);
        // free(leak);  ← Falta este free → memory leak
    }

    printf("El proceso termina y el OS recupera la memoria,\n");
    printf("pero en programas de larga duración esto agota la RAM.\n");
}

/* ============================================================
 * main
 * ============================================================ */
int main() {
    printf("╔══════════════════════════════════════════════╗\n");
    printf("║   Vulnerabilidades de Memoria en C — Demo   ║\n");
    printf("╚══════════════════════════════════════════════╝\n");
    printf("ADVERTENCIA: Código intencionalmente inseguro.\n");

    // Nota: buffer_overflow puede crashear el proceso,
    // por eso está comentado. Descomenta para ver el efecto.
    // demo_buffer_overflow();

    demo_dangling_pointer();
    demo_use_after_free();
    demo_double_free();
    demo_memory_leak();

    printf("\n✓ Demo completo.\n");
    printf("Ahora mira rust_safe_equivalent/src/main.rs\n");
    printf("para ver cómo Rust previene TODOS estos errores.\n\n");

    return 0;
}
