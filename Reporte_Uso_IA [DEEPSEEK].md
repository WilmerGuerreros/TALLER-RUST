# Reporte de uso responsable de IA

**Proyecto:** Taller de Rust — Análisis y Diseño de Algoritmos, ESAN 2026-2  
**Variante:** V2  
**Fecha del reporte:** 23/10/2026  
**Asistente utilizado:** DeepSeek V4.1 Flash Free, a través de OpenCode  
**Modalidad de uso:** tutoría, revisión de código, diagnóstico de errores, verificación numérica y redacción de apuntes

---

## Resumen ejecutivo

- Se trabajaron **C4, C5, Problema 2, Problema 4, Problema 6 y Problema 8** de la variante V2.
- La IA se usó para **revisar, diagnosticar, verificar y documentar**; el estudiante aportó el código inicial, las decisiones y la integración de las correcciones.
- Contribución estimada al producto final: **≈60 % estudiante / ≈40 % IA**. La documentación fue la parte con mayor apoyo de la IA.
- Pendiente principal: **C5 `raiz_entera`**. También queda ejecutar `cargo test` y revisar el `main` del Problema 8.
- El uso de IA queda declarado en la **sección 10**, con un bloque listo para `respuestas.md`.

---

## 1. Alcance y declaración

Este reporte resume el trabajo realizado en esta sesión sobre los ejercicios **C4 y C5** y los problemas **2, 4, 6 y 8** de la variante V2.

La IA se usó como **apoyo académico**: no reemplazó la entrega, no firmó el trabajo y no asistió a la defensa. Las decisiones sobre qué corregir, qué conservar y qué entregar fueron del estudiante.

La IA **no sobrescribió los archivos `.rs`** en esta sesión. Las correcciones se discutieron en el chat y se documentaron en archivos `.md`; las versiones que quedaron en los `.rs` fueron integradas por el estudiante.

No se atribuyen aportes al compañero de taller porque no hay registro de su participación en esta conversación. Este reporte cubre únicamente lo trabajado aquí.

---

## 2. Resumen del trabajo realizado

| Entregable | Qué se hizo | Estado |
|---|---|---|
| **C4 — `invertir_arreglo`** | Se revisó la inversión en el mismo arreglo con un solo temporal por intercambio. Se documentó la lógica y las pruebas. | Documentado en `C4_invertir_arreglo.md`; código en `C4.rs`. |
| **C4 — `Rectangulo`** | Se revisó el `struct`, `area(&self)` y `escalar(&mut self, k)`. Se documentó la diferencia entre `&self` y `&mut self`. | Documentado en `C4_rectangulo.md`; código en `C4_pt2.rs`. |
| **C5 — `fib_rec`** | Se corrigió el caso base para que `fib_rec(10)` diera 55 y 177 llamadas. Se documentó la relación entre el caso base y el conteo. | Documentado en `C5_fib_rec.md`. |
| **C5 — `raiz_entera`** | Se revisó la búsqueda binaria y se detectaron dos pendientes: la firma exacta y el caso base. Se conservó la versión original como evidencia del proceso. | Documentado en `C5_raiz_binaria.md`; **pendiente de corregir**. |
| **Problema 2 — `contar_largas`** | Se revisó la memoización de Collatz y se añadieron las comprobaciones para no indexar fuera del `Vec`. Se verificaron valores. | Documentado en `Problema2.md`; código en `Problema_2.rs`. |
| **Problema 4 — `goldbach`** | Se corrigieron dos errores de la criba y del conteo: `j += j` por `j += i`, y el recorrido desde 2 para incluir `2 + 2`. | Documentado en `Problema4.md`; código en `Problema_4.rs`. |
| **Problema 6 — `a_lo_sumo_dos`** | Se pasó de un enfoque que solo contaba a uno que compacta en el mismo vector y recorta con `truncate`. Se explicó también el error de préstamos de 6.a. | Documentado en `Problema6.md`; código en `Problema_6.rs`. |
| **Problema 8 — particiones** | Se documentaron la versión recursiva y la DP, se verificaron los conteos de llamadas y se calculó `particiones(400)`. Se señalaron cuidados sobre el `main`. | Documentado en `Problema8.md`; código en `Problema_8.rs`. |
| **Apunte adicional** | Quedó un apunte de estudio sobre la recurrencia de particiones y su árbol de llamadas. | `logica_particiones_recursivas.md`. |

---

## 3. Qué aporté yo

Mi aporte principal fue el trabajo de autoría y la orientación del proceso:

- Traje el enunciado del taller, la variante V2, las firmas exactas y las restricciones.
- Escribí las versiones iniciales de las soluciones: la estructura de las funciones, los nombres, los `main` de prueba y la lógica que quería implementar.
- Decidí pedir explicaciones y no limitarme a copiar código. Pregunté por qué fallaba cada cosa y cómo se corregía.
- Apliqué las correcciones a los archivos `.rs` que quedaron en la carpeta.
- Pedí que se conservaran mis versiones con errores, especialmente la de `raiz_entera`, como evidencia de mi proceso.
- Pregunté específicamente por `truncate`, por los casos borde, por los conteos de llamadas y por la complejidad.
- Definí el estilo que quería mantener: código simple, sin construcciones demasiado comprimidas, y explicaciones que pudiera defender.
- Pedí las pruebas, los documentos `.md` y este reporte de uso de IA.
- Decidí qué quedaba entregado y qué quedaba pendiente.

En resumen: **puse el problema, el código inicial, las decisiones y la intención de aprender**. La IA aportó diagnóstico, correcciones puntuales, verificación y redacción.

---

## 4. Qué aportó la IA

La IA tuvo un papel importante en estas tareas:

- **Lectura del taller:** extrajo el enunciado del `.docx` y contrastó firmas, restricciones y ejemplos.
- **Diagnóstico:** encontró errores concretos que no eran evidentes a simple vista, como el caso base de `fib_rec`, el `j += j` de la criba, el recorrido desde 3 en Goldbach, el conteo incompleto del último grupo en `a_lo_sumo_dos` y los límites de la memoización en Collatz.
- **Corrección:** propuso fragmentos de código y explicó por qué funcionaban.
- **Verificación:** comprobó valores con scripts de Python, por ejemplo los conteos de Collatz, `goldbach(100) = 6`, las llamadas de `particiones_rec` y `particiones(400)`.
- **Documentación:** redactó los archivos `.md` con lógica paso a paso, trazas, complejidad, pruebas y preguntas de defensa.
- **Señales de límite:** indicó lo que no estaba verificado, lo que quedaba pendiente y lo que el estudiante debía comprobar por su cuenta.

La IA **no inventó** los resultados experimentales: los valores numéricos se verificaron con scripts. Tampoco decidió la variante, el estilo de entrega ni la autoría.

---

## 5. ¿Cuánto hizo cada uno?

No existe una medición objetiva del esfuerzo ni del aprendizaje. La siguiente tabla es una **estimación aproximada de contribución al producto final**, no una nota ni una medida de lo aprendido.

| Área | Estudiante | IA | Comentario |
|---|---:|---:|---|
| Comprensión del problema y elección de la variante | 70 % | 30 % | El estudiante trajo el taller y las restricciones; la IA ayudó a ordenarlas y contrastarlas. |
| Código inicial | 85 % | 15 % | Las primeras versiones fueron del estudiante; la IA las revisó. |
| Diagnóstico de errores | 30 % | 70 % | La IA encontró varios bugs; el estudiante los comprendió y decidió corregirlos. |
| Corrección final del código | 55 % | 45 % | La IA propuso fragmentos; el estudiante los adaptó e integró. |
| Verificación numérica | 40 % | 60 % | La IA hizo scripts de comprobación; el estudiante debe ejecutar `cargo test`. |
| Documentación y explicaciones | 20 % | 80 % | La IA redactó los `.md`; el estudiante definió el enfoque y revisó. |
| Decisiones de entrega y autoría | 85 % | 15 % | El estudiante decidió qué conservar, qué corregir y qué presentar. |

**Estimación global:** aproximadamente **60 % estudiante / 40 % IA** en el producto final.

Esa cifra es solo orientativa. En el código final el aporte humano es mayor; en la documentación y el diagnóstico el aporte de la IA es mayor. El aprendizaje y la defensa siguen siendo responsabilidad del estudiante.

---

## 6. Errores, correcciones y aprendizajes

| Entregable | Error o limitación | Corrección | Quién lo detectó | Estado |
|---|---|---|---|---|
| **C5 — `fib_rec`** | El caso base `n == 1 || n == 2 → 1` producía 109 llamadas para `n = 10`, no 177. | Usar `n <= 1 → n` y contar cada llamada, incluida la base. | IA | Corregido y documentado. |
| **C5 — `raiz_entera`** | La firma tenía tres parámetros y el caso base devolvía `*lim_sup + 1`; para `n = 26` daba 7 en vez de 5. | Falta una envoltura con la firma exacta y corregir el caso base. | IA | **Pendiente**. La versión original se conservó como evidencia. |
| **Problema 2 — Collatz** | La memoización podía indexar fuera del `Vec` cuando la secuencia superaba `limite`. | Leer y guardar solo si `n < memo.len()`. | IA | Corregido y aplicado. |
| **Problema 4 — Goldbach** | La criba usaba `j += j`, que duplicaba el índice y dejaba múltiplos sin marcar. | Usar `j += i`. | IA | Corregido y aplicado. |
| **Problema 4 — Goldbach** | El conteo empezaba en `p = 3`, así que omitía `2 + 2` y `goldbach(4)` daba 0. | Empezar en `p = 2`. | IA | Corregido y aplicado. |
| **Problema 6 — `a_lo_sumo_dos`** | La versión inicial solo calculaba la longitud; no modificaba ni recortaba `v`. | Compactar con dos índices: `leer` y `escribir`. | IA | Corregido y aplicado. |
| **Problema 6 — `a_lo_sumo_dos`** | El último grupo con más de dos repetidos no se sumaba; `[1,1,1]` devolvía 0. | Escribir mientras se recorre y hacer `v.truncate(escribir)` al final. | IA | Corregido y aplicado. |
| **Problema 6 — `a_lo_sumo_dos`** | El enfoque con `remove` era `O(n)` por eliminación y podía salirse del índice. | Dos punteros y `truncate`; sin `remove` y sin otro vector. | IA | Corregido y aplicado. |
| **Problema 8 — particiones** | No había un error funcional en las dos funciones, pero el `main` podía confundir `K` con `n` y llamaba a la recursiva antes que a la DP. | Documentar que `particiones_rec(n,k)` es restringida y `particiones(n)` es total; no ejecutar la recursiva con 400. | IA | Documentado; el `main` sigue siendo el original. |

---

## 7. Cómo evolucionó el proceso

Se puede dividir en cinco etapas:

### 1. “Que compile”

Al inicio el objetivo era resolver y hacer que el código funcionara. Las preguntas eran del tipo “¿y ahora?” y los errores se corregían de forma puntual.

### 2. “Entender el porqué”

Después el enfoque cambió: ya no bastaba con recibir el código corregido. Empecé a preguntar por qué fallaba cada versión, qué hacía cada línea y por qué la corrección era correcta.

### 3. “Cubrir casos borde y complejidad”

Aparecieron preguntas sobre vectores vacíos, grupos repetidos al final, límites de la memoización, desbordamientos y complejidad. Eso hizo que las soluciones no solo funcionaran con el ejemplo del enunciado.

### 4. “Documentar y defender”

Pedí archivos `.md`, pruebas, trazas, tablas de complejidad y preguntas de defensa. El objetivo dejó de ser “tener el código” y pasó a ser “poder explicarlo y modificarlo”.

### 5. “Declarar el uso de IA”

Al final pedí este reporte. Eso cierra el proceso con transparencia: qué se usó, para qué, qué se corrigió y qué quedó pendiente.

La evidencia más clara de esta evolución es el Problema 6: se pasó de una versión que solo contaba elementos a una que **modifica el vector en el mismo lugar**, entiende `truncate` y explica el préstamo de 6.a. También se conservó la versión con errores de `raiz_entera` en lugar de borrarla.

---

## 8. Verificación realizada

| Caso | Resultado |
|---|---|
| `fib_rec(10)` | 55, con 177 llamadas. |
| `contar_largas(10, 10)` | 2 |
| `contar_largas(10, 8)` | 3 |
| `contar_largas(10, 16)` | 2 |
| `contar_largas(10, 17)` | 1 |
| `contar_largas(1_000_000, 100)` | 655 970 |
| `goldbach(4)` | 1 |
| `goldbach(100)` | 6 |
| `a_lo_sumo_dos([1,1,1,2,2,3])` | 5; el vector queda `[1,1,2,2,3]`. |
| `a_lo_sumo_dos([1,1,1,1,1])` | 2; el vector queda `[1,1]`. |
| `particiones_rec(10, 10)` | 42, con 277 llamadas. |
| `particiones_rec(20, 20)` | 627, con 5 427 llamadas. |
| `particiones_rec(25, 25)` | 1 958, con 18 591 llamadas. |
| `particiones(400)` | 6 727 090 051 741 041 926 |
| `raiz_entera(26)` con la versión actual | 7; el enunciado espera 5. **Pendiente.** |

La verificación numérica se hizo con scripts de Python. En este entorno no había `rustc`, así que **no se ejecutó `cargo test`**. La compilación y las pruebas de Rust deben ejecutarse en VizcachaIDE o en el entorno del curso antes de la entrega.

---

## 9. Límites y pendientes

- **C5 — `raiz_entera`:** falta la firma exacta `pub fn raiz_entera(n: u64) -> u64` y corregir el caso base. Es el pendiente más importante.
- **Problema 8 — `main`:** no ejecutar `particiones_rec(400, 400, ...)`; la recursiva no termina en tiempo razonable. Para 400 usar solo `particiones(400)`.
- **Problema 8 — `K` vs `n`:** `particiones_rec(n, K, ...)` cuenta particiones con partes ≤ K; `particiones(n)` cuenta todas. Si se ingresan valores distintos, los resultados no tienen por qué coincidir.
- **Problema 2 — compatibilidad:** el archivo `Problema_2.rs` usa `n.is_multiple_of(2)`. Si la versión de Rust del laboratorio no lo soporta, se puede reemplazar por `n % 2 == 0`.
- **Pruebas de Rust:** deben ejecutarse con `cargo test` y `cargo test --release`; la verificación de este reporte fue numérica, no una compilación real.
- **Defensa:** la IA no puede responder por el estudiante. Hay que poder explicar cada línea y modificarla en vivo.
- **Compañero:** no se le atribuye ningún aporte porque no hay registro de su trabajo en esta conversación.

---

## 10. Declaración para `respuestas.md`

Este bloque se puede adaptar y pegar en `respuestas.md`:

```markdown
### Declaración de uso de IA

Utilicé un asistente de IA (DeepSeek V4.1 Flash Free, vía OpenCode)
como apoyo para revisar código, diagnosticar errores, verificar
resultados y redactar apuntes.

Problemas en los que se usó: C4, C5, Problema 2, Problema 4,
Problema 6 y Problema 8, todos de la variante V2.

La conversación completa se adjunta a este reporte / se puede
consultar en el enlace correspondiente: [pendiente: enlace o captura].

Qué tuve que corregir o aprender:

- C5 `fib_rec`: el caso base correcto es `n <= 1`, no `n == 1 || n == 2`;
  con el caso base equivocado, `fib_rec(10)` daba 109 llamadas en vez de 177.
- C5 `raiz_entera`: falta la firma exacta y corregir el caso base; la
  versión actual da 7 para 26 y debería dar 5. Queda pendiente.
- Problema 2: la memoización debe comprobar `n < memo.len()` antes de
  indexar, porque la secuencia de Collatz puede superar el límite.
- Problema 4: la criba debe avanzar con `j += i`, no con `j += j`, y el
  conteo de Goldbach debe empezar en `p = 2` para incluir `2 + 2`.
- Problema 6: no basta con contar; hay que compactar el vector en el
  mismo lugar y recortarlo con `truncate`.
- Problema 8: `particiones_rec(n, K, ...)` es la versión restringida;
  `particiones(n)` es la versión total. La recursiva no sirve para 400.

Mi aporte fue el código inicial, la elección de la variante, las
decisiones de estilo, la aplicación de las correcciones y la
preparación de la entrega. La IA aportó diagnóstico, explicaciones,
verificación numérica y redacción de los apuntes.
```

---

## 11. Conclusión

El trabajo fue un **híbrido transparente**: el estudiante aportó el problema, el código inicial, las decisiones y la autoría; la IA aportó diagnóstico, correcciones puntuales, verificación y documentación.

La IA fue útil sobre todo para:

- encontrar errores que no se veían a simple vista;
- explicar el “por qué” y no solo el “cómo”;
- convertir el código en material de estudio y defensa;
- dejar registro de los errores y de los límites.

Lo más valioso del proceso no fue recibir código, sino **aprender a revisarlo, cuestionarlo y documentarlo**. El único pendiente importante es `raiz_entera`; el resto queda documentado y con pruebas sugeridas.

La IA no reemplaza la defensa ni la autoría. Este reporte existe precisamente para que el uso quede declarado y para que el estudiante pueda explicar qué hizo, qué corrigió y qué aprendió.

---

## Anexo: archivos relacionados

**Documentos:**

- `C4_invertir_arreglo.md`
- `C4_rectangulo.md`
- `C5_fib_rec.md`
- `C5_raiz_binaria.md`
- `Problema2.md`
- `Problema4.md`
- `Problema6.md`
- `Problema8.md`
- `logica_particiones_recursivas.md`
- `Reporte_Uso_IA.md` (este archivo)

**Código:**

- `C4.rs`
- `C4_pt2.rs`
- `Problema_2.rs`
- `Problema_4.rs`
- `Problema_6.rs`
- `Problema_8.rs`
- `C5_raiz_cuadrada.rs`
