---
marp: true
theme: default
paginate: true

---
# Programación Concurrente 2025
## Presentación de trabajos prácticos
Docentes: Emilio López Gabeiras, Rodrigo Pazos
Grupo: Francisco Zizzi, Tomás Roldán Borjas

---
# Objetivo

Esta presentación tiene por objetivo describir los problemas de cada TP brevemente, presentar el código esencial para su resolución y responder a las preguntas de reflexión que surgen en cada caso.

Por este mótivo, y para mantener el código conciso, nos enfocamos específicamente en las líneas que resuelven el problema planteado por cada TP, omitiendo detalles como firmas de métodos, parsing de requests, etc.
 
---
# TP1: Servidor HTTP en Rust
**Problema**: Servidor monohilo que recibe `GET /pi/:i` y calcula π usando la serie de Leibniz.

**Solución** (`tp1/src/main.rs`):
```rust
for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    let response = handle_request(&mut stream);
    stream.write_all(response.as_bytes()).unwrap();
}
```
- `handle_request` llama a  `compute_pi(n)`, que aplica la serie de Leibniz.

---
# TP1: Preguntas y Respuestas
**¿Qué sucede al recibir 2 requests simultáneas?**
- El segundo espera a que termine el primero; el servidor bloquea.

**¿Por qué ocurre?**
- No hay concurrencia: estamos manejando un solo thread, por lo cual las requests se atienden secuencialmente.

**¿Cómo solucionarlo solo con `std`?**
- Usar `std::thread::spawn` para cada conexión (ver TP2).

---
# TP2: Servidor Concurrente con Threads
**Problema**: Evolucionar TP1 para atender múltiples solicitudes simultáneas.

**Solución** (`tp2/src/main.rs`):
```rust
for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            let result = handle_request(&mut stream);
            stream.write_all(result.as_bytes()).unwrap();
        });
    }
```
- Cada petición se maneja en un hilo independiente.

---
# TP2: Preguntas y Respuestas
**¿Qué efectos se observan al aumentar `-n` y `-c` usando Apache Benchmark? ¿Se nota diferencia en tiempos?**
- Al principio, aumenta la velocidad promedio del tratamiento de cada request.
- Al aumentar la cantidad de requests concurrentes mucho más allá de la cantidad de cores, la velocidad vuelve a disminuir.

**¿A qué se debe?**
- Existe una sobrecarga de hilos, por lo cual se dan muchos context switches y la concurrencia ni siquiera se aprovecha.

---
# TP3: Thread Pool
**Problema**: Reducir overhead de context switches en TP2 usando un pool de hilos.

**Solución** (`tp3/src/server/pooling.rs`):
```rust
    let (tx, rx) = channel::<Box<dyn Send + FnOnce()>>();
    let rx_arc = Arc::new(Mutex::new(rx));

    for _ in 0..N_THREADS {
        let arc_clone = rx_arc.clone();
        thread::spawn(|| {
            check_and_run_tasks(arc_clone);
        });
    }
```
- Pool fijo de N hilos, enviándoles las tareas mediante `mpsc`.

---
# TP3: Preguntas y Respuestas
**Bajo carga concurrente intensa, ¿se observa una mejora respecto al TP2?**
- Sí. Se reduce latencia y overhead al reusar hilos en lugar de crear nuevos.

**¿Cómo afecta tamaño del pool?**
- Para un pool de N_THREADS ≈ # Cores del CPU, maneja una gran carga de concurrencia mejor que el TP2.
- Para un número mucho mayor, se vuelve al mismo problema.

---
# TP4: LogAnalyzer – Escritura
**Problema**: Servidor HTTP que reciba archivos de log, cuente "exception" y provea estadísticas, garantizando la mayor eficiencia de escritura y lectura, y limitando las subidas de archivos simultáneas a 4 como máximo.

**Solución**
Se utiliza un Read-Write Lock para las estadísticas, pudiendo bloquear al escribir y leer sin bloqueos, y un semáforo para limitar la cantidad de subidas

---

**Subida de archivo** (`tp4/src/controllers/file_upload.rs`):
```rust
    let semaphore = server.get_arc_semaphore();
    let permit = semaphore.try_acquire(); // Semáforo con límite de 4
    match permit {
        Ok(_) => {},
        Err(_) => { // Falla cuando 4 hilos ya notificaron al semáforo
            return utils::response::create_response(429, "Processing too many files".to_string()); 
        }
    }

    // ...Salto de código

    map_arc.write().unwrap().insert(file_name.clone(), count); // Lock absoluto del Map para escribir
```

**Lectura de estadísticas** (`tp4/src/controllers/stats.rs`):
```rust
    // Se obtiene acceso de lectura (no bloqueante, por ser un R/W lock)
    let count_map = count_map_arc.read().unwrap().clone();
```

---
# TP MiniGrep
**Problema**: Buscar un patrón en varios archivos en tres modos: secuencial, hilo por archivo y hilo por chunk.

**Solución secuencial** (`mini_grep/src/lib.rs`):
```rust
pub fn grep_seq(pattern: String, file_names: Vec<String>) -> Vec<String> {
    file_names
        .into_iter()
        .map(|file_name| filter_lines_from_file(file_name, pattern.clone()))
        .flatten()
        .collect::<Vec<_>>()
}
```

---

**Solución concurrente**:
```rust
    let threads: Vec<JoinHandle<Vec<String>>> = file_names.into_iter()
        .map(|file| { // Creamos un thread por archivo, para paralelizar
            let pattern_clone = pattern.clone();
            thread::spawn(|| filter_lines_from_file(file, pattern_clone).collect::<Vec<_>>())
        })
        .collect();
```
**Solución por chunks**:
```rust
    let mut br = BufReader::new(File::open(file_name).unwrap()).lines();
    // Esta es la parte clave del código. Se van incluyendo líneas hasta llenar un chunk:
    loop {
        let chunk: Vec<String> = br.by_ref().take(chunk_size).map(|line| line.unwrap()).collect::<Vec<_>>();

        if chunk.is_empty() {
            break;
        };

        add_new_chunk_thread(chunk, &mut chunk_threads, pattern.clone());
    }
```

---
# TP MiniGrep: Preguntas y Respuestas
**¿Tiempo secuencial vs concurrente?**
- La solución concurrente tarda aproximadamente la mitad

**Al reducir el tamano de los segmentos (chunks), ¿qué patrón se observa
en los tiempos de ejecución? ¿A qué se debe esto?**
- Los tiempos de ejecución van aumentando. A veces llega a tardar más que la solución concurrente. Esto se debe a un exceso de threads y context switches.

---
# TP5: Colas Concurrentes
**Problema**: Implementar FIFO con múltiples productores/consumidores: versión bloqueante vs no bloqueante.

**Solución**:
- **Bloqueante:** Utilizar `Mutex` + `Condvar`.
- **No bloqueante:** Lista con `AtomicPtr` y CAS.

Como la novedad en este TP son los algoritmos no bloqueantes, mostraremos principalmente esa implementación. A su vez, en este caso sí será importante ver todo el código para entender el funcionamiento.

---


**enqueue**

```rust
pub fn enqueue(&self, item: T) {
        let new_node: *mut Node<T> = Box::into_raw(Box::new(Node::new(item)));
        // Usamos un loop para intentar hasta que finalmente hagamos enqueue
        loop {
            let cur_tail: *mut Node<T> = self.tail.load(RELAXED);
            // Usamos unsafe porque manejamos raw pointers
            let tail_next: *mut Node<T> = unsafe { (*cur_tail).next.load(RELAXED) };

            // Verificamos que otro thread no haya cambiado el head
            if cur_tail == self.tail.load(RELAXED) {
                // Si el próximo no es nulo, "ayudamos" a avanzar del head
                if !tail_next.is_null() {
                    let _ = self.tail.compare_exchange(cur_tail, tail_next, ACQUIRE, RELAXED);
                } 
                // Probamos a insertar el elemento y luego avanzamos el head
                else if unsafe {
                    (*cur_tail).next.compare_exchange(null_mut(), new_node, RELEASE, RELAXED).is_ok()
                } {
                    let _ = self.tail.compare_exchange(cur_tail, new_node, RELAXED, RELAXED);
                    return;
                }
            }
        }
    }
```

---

**dequeue**

```rust
pub fn dequeue(&self) -> Option<T> {
        // Usamos un loop para intentar hasta que finalmente hagamos dequeue
        loop {
            let head = self.head.load(ACQUIRE);
            let tail = self.tail.load(ACQUIRE);
            let next = unsafe { (*head).next.load(ACQUIRE) };
            // Si el head es también el tail, en realidad tenemos un dummy node
            if head == tail {
                // Si el siguiente es null, verdaderamente la cola esta vacía
                if next.is_null() {
                    return None;
                }
                // Si no, justo un thread agregó un elemento y hay que avanzar el tail
                let _ = self.tail.compare_exchange(tail, next, RELEASE, ACQUIRE);
            } else {
                // Si next es nulo aquí, estamos en un estado transitorio e inconsistente.
                if next.is_null() {
                    continue;
                }
                // Si `compare_exchange` tiene éxito, significa que ningún otro hilo se nos adelantó
                if self.head.compare_exchange(head, next, RELEASE, ACQUIRE).is_ok() {
                    let item_option = unsafe { (*next).item.take() };
                    return item_option;
                }
            }
        }
    }
```

---
# TP5: Preguntas y Respuestas
**¿Diferencias de rendimiento?**
- El bloqueante es mejor para mayor cantidad de hilos, y el no bloqueante para pocos hilos (livelock para muchos).

**¿Dificultades técnicas al implementar la versión no bloqueante?**
- Tuvimos que arreglar un problema de violación de acceso a memoria.

**¿Bajo qué escenarios conviene usar cada una?**
- Usar non-blocking con baja contención, y blocking bajo alta contención.

**¿Si se mezclan bloqueante con no bloqueante?**
- Sería positivo si la alta contención está del lado del bloqueante, y viceversa.

---
# TP7: Threads vs Async
**Problema**: Comparar modelo de threads tradicionales vs async/await con Tokio para I/O y cálculo.

**Solución** (`tp7/src/main.rs`):
```rust
simulate_io_threads(TASKS);
simulate_io_async(TASKS).await;
let pi_t = liebniz_threads(TERMS);
let pi_a = liebniz_async(TERMS).await;
```
- I/O: `thread::sleep` vs `tokio::time::sleep`.
- Cálculo: subtareas unidas con `join` o `await`.

---
# TP7: Preguntas y Respuestas
**¿Qué escala mejor en I/O?**
- `async` maneja miles de tareas sin bloquear hilos.

**¿Y en cómputo intensivo?**
- Threads nativos aprovechan mejor CPU en cálculos pesados.
 