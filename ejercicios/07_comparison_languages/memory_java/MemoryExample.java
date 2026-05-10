// MemoryExample.java
// Gestión de Memoria en Java — Garbage Collector
//
// Java usa un Garbage Collector (GC) para gestionar la memoria.
// El GC libera al programador de gestión manual, pero introduce:
//   - Pausas impredecibles (GC pauses / "Stop the World")
//   - Overhead de memoria (el heap de Java es mucho más grande)
//   - Menor control sobre cuándo se libera la memoria
//
// Compilar: javac MemoryExample.java
// Ejecutar: java MemoryExample

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class MemoryExample {

    // =========================================================
    // 1. El Problema: NullPointerException
    // =========================================================
    static String buscarUsuario(int id) {
        if (id == 1) return "Alice";
        return null; // ← Fuente de NullPointerException
    }

    static void demoNullProblema() {
        System.out.println("\n--- 1. El Problema de null ---");

        String usuario = buscarUsuario(999);
        // Si no verificamos null, esto lanza NullPointerException:
        // System.out.println(usuario.toUpperCase()); // ← NPE!

        // Debemos verificar manualmente — el compilador NO lo exige:
        if (usuario != null) {
            System.out.println("Usuario: " + usuario.toUpperCase());
        } else {
            System.out.println("Usuario no encontrado");
        }
        // En Rust, Optional<T>/Option<T> hace esto OBLIGATORIO en compile-time
    }

    // =========================================================
    // 2. Optional<T> — La Solución Moderna de Java (Java 8+)
    // =========================================================
    static Optional<String> buscarUsuarioSeguro(int id) {
        if (id == 1) return Optional.of("Alice");
        return Optional.empty(); // En lugar de null
    }

    static void demoOptional() {
        System.out.println("\n--- 2. Optional<T> en Java ---");

        Optional<String> resultado = buscarUsuarioSeguro(999);

        // Similar a Option<T> de Rust, pero NO garantizado por el compilador
        // — el programador puede seguir usando null en lugar de Optional
        resultado.ifPresent(u -> System.out.println("Encontrado: " + u));

        String valor = resultado.orElse("Valor por defecto");
        System.out.println("Con orElse: " + valor);

        Optional<String> usuario1 = buscarUsuarioSeguro(1);
        usuario1
            .map(String::toUpperCase)
            .ifPresent(u -> System.out.println("Uppercase: " + u));
    }

    // =========================================================
    // 3. Garbage Collector — Gestión Automática (con trade-offs)
    // =========================================================
    static void demoGarbageCollector() {
        System.out.println("\n--- 3. Garbage Collector ---");

        // Java asigna y libera automáticamente — pero con overhead
        System.out.println("Creando objetos — el GC los liberará eventualmente:");

        for (int i = 0; i < 5; i++) {
            // Estos objetos se vuelven elegibles para GC cuando salen de scope
            List<Integer> lista = new ArrayList<>();
            for (int j = 0; j < 1000; j++) {
                lista.add(j);
            }
            System.out.println("  Iteración " + i + ": lista de " + lista.size() + " elementos");
            // lista sale de scope — elegible para GC
            // PERO: el GC no la libera inmediatamente
        }

        System.out.println("Objetos fuera de scope — serán liberados EVENTUALMENTE por el GC");
        System.out.println("Nota: la memoria puede no liberarse hasta la próxima pausa del GC");

        // Puedes sugerir GC (no garantizado):
        System.gc(); // Solo una SUGERENCIA — el GC puede ignorarla
        System.out.println("System.gc() llamado — el GC puede o no ejecutarse ahora");
    }

    // =========================================================
    // 4. Problema: GC Pauses
    // =========================================================
    static void demoGCPauses() {
        System.out.println("\n--- 4. GC Pauses (Stop-the-World) ---");

        System.out.println("En aplicaciones de alta latencia, el GC puede causar:");
        System.out.println("  - Pausas de ms a segundos (según el GC y el heap)");
        System.out.println("  - 'Stop-the-World': todos los hilos se pausan durante el GC");
        System.out.println("  - Comportamiento impredecible bajo presión de memoria");
        System.out.println();
        System.out.println("GCs modernos de Java (G1, ZGC, Shenandoah) reducen esto,");
        System.out.println("pero no lo eliminan completamente.");
        System.out.println();
        System.out.println("Rust: no hay GC → no hay GC pauses → latencia predecible");
    }

    // =========================================================
    // 5. Threads en Java — Riesgo de Race Conditions
    // =========================================================
    static int contadorInseguro = 0; // Variable compartida sin sincronización

    static void demoRaceCondition() throws InterruptedException {
        System.out.println("\n--- 5. Race Condition en Java ---");

        // Sin sincronización, esto produce resultados incorrectos:
        Thread[] hilos = new Thread[10];
        for (int i = 0; i < 10; i++) {
            hilos[i] = new Thread(() -> {
                for (int j = 0; j < 1000; j++) {
                    contadorInseguro++; // ← RACE CONDITION: read-modify-write no es atómico
                }
            });
            hilos[i].start();
        }

        for (Thread h : hilos) h.join();

        System.out.println("Esperado: 10000");
        System.out.println("Obtenido: " + contadorInseguro + " (posiblemente menor — race condition)");
        System.out.println("→ Java no previene race conditions en compile-time");
        System.out.println("→ Rust: si intentas esto, el compilador te lo impide");
    }

    // =========================================================
    // main
    // =========================================================
    public static void main(String[] args) throws InterruptedException {
        System.out.println("╔══════════════════════════════════════════╗");
        System.out.println("║   Gestión de Memoria en Java (GC)        ║");
        System.out.println("╚══════════════════════════════════════════╝");

        demoNullProblema();
        demoOptional();
        demoGarbageCollector();
        demoGCPauses();
        demoRaceCondition();

        System.out.println("\n=== Resumen: Java vs Rust ===");
        System.out.println("Java  → GC automático, pausas impredecibles, null peligroso");
        System.out.println("Rust  → Ownership determinístico, sin GC, sin null, sin race conditions");
        System.out.println("Java  → Más fácil de aprender, ecosistema maduro, lento comparado a C/Rust");
        System.out.println("Rust  → Curva pronunciada, máximo rendimiento, seguridad garantizada");
    }
}
