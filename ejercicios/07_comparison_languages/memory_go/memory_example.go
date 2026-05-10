// memory_example.go
// Gestión de Memoria en Go — Garbage Collector Concurrente
//
// Go usa un GC concurrente (mark-and-sweep) que intenta minimizar
// las pausas STW. Es más simple que el GC de Java, pero igualmente
// introduce latencia no determinística.
//
// Ejecutar: go run memory_example.go

package main

import (
	"fmt"
	"runtime"
	"sync"
	"time"
)

// =========================================================
// 1. Nil en Go — Similar a null
// =========================================================
type Usuario struct {
	Nombre string
	Edad   int
}

func buscarUsuario(id int) *Usuario {
	if id == 1 {
		return &Usuario{Nombre: "Alice", Edad: 30}
	}
	return nil // Go usa nil — puede causar nil pointer dereference
}

func demoNilProblema() {
	fmt.Println("\n--- 1. El Problema de nil en Go ---")

	usuario := buscarUsuario(999)

	// Sin verificar nil, esto causa panic: nil pointer dereference
	// fmt.Println(usuario.Nombre)  // ← PANIC!

	// Debes verificar manualmente — Go no lo exige en compile-time:
	if usuario != nil {
		fmt.Printf("Usuario encontrado: %s\n", usuario.Nombre)
	} else {
		fmt.Println("Usuario no encontrado (nil)")
	}

	fmt.Println("→ Go no previene nil pointer dereference en compile-time")
	fmt.Println("→ Rust: Option<T> hace esto OBLIGATORIO")
}

// =========================================================
// 2. Garbage Collector de Go
// =========================================================
func demoGarbageCollector() {
	fmt.Println("\n--- 2. Garbage Collector de Go ---")

	// Go usa escape analysis para decidir stack vs heap:
	// Si una variable "escapa" a un scope mayor → heap
	// Si no escapa → stack (más rápido)

	var stats runtime.MemStats
	runtime.ReadMemStats(&stats)
	fmt.Printf("Memoria heap en uso antes: %d KB\n", stats.HeapInuse/1024)

	// Crear y abandonar slices — el GC los recogerá
	for i := 0; i < 5; i++ {
		datos := make([]int, 100_000) // 800 KB
		datos[0] = i
		fmt.Printf("  Creados %d enteros (iteración %d)\n", len(datos), i)
		// datos sale de scope → elegible para GC
	}

	runtime.GC() // Forzar GC (solo para demo — evitar en producción)
	runtime.ReadMemStats(&stats)
	fmt.Printf("Memoria heap en uso después del GC: %d KB\n", stats.HeapInuse/1024)
	fmt.Printf("Total de GC runs: %d\n", stats.NumGC)
}

// =========================================================
// 3. Goroutines y Race Conditions
// =========================================================
func demoRaceConditions() {
	fmt.Println("\n--- 3. Race Conditions en Go ---")

	// Sin sincronización — race condition:
	// (Correr con: go run -race memory_example.go para detectarlo)
	contador := 0
	var wg sync.WaitGroup

	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			contador++ // ← RACE CONDITION (sin mutex)
		}()
	}
	wg.Wait()
	fmt.Printf("Contador inseguro (esperado 100): %d\n", contador)

	// Solución en Go: sync.Mutex
	var mu sync.Mutex
	contadorSeguro := 0

	for i := 0; i < 100; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock()
			contadorSeguro++
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Printf("Contador con Mutex (esperado 100): %d ✅\n", contadorSeguro)

	fmt.Println("→ Go detecta races con -race flag (runtime)")
	fmt.Println("→ Rust: las detecta en COMPILE TIME — más robusto")
}

// =========================================================
// 4. Channels en Go — Comunicación Entre Goroutines
// =========================================================
func demoChannels() {
	fmt.Println("\n--- 4. Channels en Go ---")

	// Los channels de Go son similares a mpsc de Rust
	ch := make(chan string, 5) // Buffered channel

	// Productor:
	go func() {
		mensajes := []string{"msg1", "msg2", "msg3"}
		for _, m := range mensajes {
			ch <- m
			time.Sleep(10 * time.Millisecond)
		}
		close(ch) // Importante: cerrar el channel
	}()

	// Consumidor:
	for msg := range ch {
		fmt.Printf("  Recibido: %s\n", msg)
	}

	fmt.Println("Channels transferen datos entre goroutines ✅")
}

// =========================================================
// 5. Comparativa de Latencia GC
// =========================================================
func demoLatenciaGC() {
	fmt.Println("\n--- 5. Latencia del GC de Go ---")

	var tiemposTotalPausa time.Duration
	var pausas []time.Duration

	// Simular trabajo mientras el GC puede pausar:
	for i := 0; i < 10; i++ {
		inicio := time.Now()

		// Crear presión en el heap para provocar GC:
		data := make([][]byte, 1000)
		for j := range data {
			data[j] = make([]byte, 1024)
		}
		_ = data

		elapsed := time.Since(inicio)
		pausas = append(pausas, elapsed)
		tiemposTotalPausa += elapsed
	}

	promedio := tiemposTotalPausa / time.Duration(len(pausas))
	fmt.Printf("Tiempo promedio por iteración (incluye GC): %v\n", promedio)

	var gcStats runtime.MemStats
	runtime.ReadMemStats(&gcStats)
	fmt.Printf("Total pause time GC: %v\n", time.Duration(gcStats.PauseTotalNs))
	fmt.Printf("GC runs: %d\n", gcStats.NumGC)
	fmt.Println()
	fmt.Println("Go GC es bajo-latencia pero no determinístico.")
	fmt.Println("Rust: sin GC → latencia completamente determinística.")
}

// =========================================================
// main
// =========================================================
func main() {
	fmt.Println("╔══════════════════════════════════════════╗")
	fmt.Println("║   Gestión de Memoria en Go (GC)          ║")
	fmt.Println("╚══════════════════════════════════════════╝")

	demoNilProblema()
	demoGarbageCollector()
	demoRaceConditions()
	demoChannels()
	demoLatenciaGC()

	fmt.Println("=== Resumen: Go vs Rust ===")
	fmt.Println("Go   → GC concurrente, goroutines ligeras, simplicidad, nil peligroso")
	fmt.Println("Rust → Sin GC, ownership, sin nil, races imposibles en compile-time")
	fmt.Println("Go   → Ideal para backends web, microservicios, herramientas")
	fmt.Println("Rust → Ideal para sistemas, WebAssembly, latencia crítica")
}
