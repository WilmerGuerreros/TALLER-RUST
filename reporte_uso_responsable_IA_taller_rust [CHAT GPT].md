# Reporte de uso responsable de IA — Taller de Rust (ADA)

**Fecha:** 23 de octubre de 2026  
**Tema:** Conteo de particiones mediante recursión y programación dinámica.

> **Nota:** este reporte resume el proceso observado en la conversación. Antes de entregarlo, verifica que refleje fielmente tu experiencia y ajústalo si el taller exige un formato, enlace o declaración específica.

## 1. Objetivo del trabajo

Comprender e implementar dos soluciones para contar las particiones de un entero:

1. **Versión recursiva:** `particiones_rec(n, maximo, llamadas)`, que cuenta particiones respetando un sumando máximo y registra el número de invocaciones.
2. **Versión con memoización:** `particiones(n)`, que reutiliza resultados previamente calculados en una matriz para evitar repetir subproblemas.

El objetivo del uso de IA fue recibir explicaciones, detectar errores conceptuales y de Rust, y orientar la depuración sin delegar deliberadamente todo el ejercicio desde el inicio.

## 2. Aportes del estudiante

Durante el proceso, el estudiante realizó aportes sustanciales:

- **Propuso la lógica inicial de la recursión.** Planteó separar el problema entre usar un sumando máximo y no usarlo, y preguntó cómo continuar con el resto de la suma.
- **Construyó y modificó su propio código.** Compartió varias versiones, cambió condiciones, reorganizó casos base y adaptó la función auxiliar según los errores que iba comprendiendo.
- **Identificó problemas de Rust.** Se encontró con dificultades relacionadas con préstamos mutables (`&mut`), tipos de datos, índices y la necesidad de pasar una tabla a una función auxiliar.
- **Propuso almacenar resultados.** Dedujo que la versión dinámica necesitaría conservar una tabla y sugirió separar el cálculo en una función auxiliar para mantener la firma pública exigida.
- **Trabajó con `Option<u64>`.** Adaptó la tabla para distinguir entre un estado no calculado (`None`) y un resultado ya calculado (`Some(valor)`).
- **Solicitó validación y aclaraciones en lugar de limitarse a copiar una solución.** Preguntó reiteradamente si sus cambios eran correctos y presentó nuevas versiones para recibir retroalimentación.
- **Mantuvo el control del aprendizaje.** Indicó que quería ir programando por su cuenta y utilizó las explicaciones para seguir modificando el código.

## 3. Aportes de ChatGPT

La IA actuó principalmente como tutor y apoyo de depuración. Sus contribuciones incluyeron:

- Explicar la recurrencia como dos grupos disjuntos:
  - No utilizar el sumando `maximo`: `P(n, maximo - 1)`.
  - Utilizar al menos un `maximo`: `P(n - maximo, maximo)`.
- Explicar por qué `P(0, maximo) = 1` y por qué `P(n, 0) = 0` cuando `n > 0`.
- Crear una explicación visual del árbol recursivo y un diagrama de flujo.
- Aclarar que `maximo` es un límite superior permitido, no un sumando obligatorio, y que puede limitarse a `n`.
- Explicar la diferencia entre recursión simple y memoización, y por qué la tabla debe distinguir los estados `(n, maximo)`, no solo `n`.
- Orientar sobre `Vec<Vec<Option<u64>>>`, el significado de `None` y `Some(valor)`, y el uso de `&mut` para compartir una tabla con una función auxiliar.
- Señalar errores potenciales: restas inválidas con `u32`, casos base en un orden incorrecto, `.unwrap()` sobre `None`, discrepancias entre firmas y tipos, y variables no utilizadas.
- Sugerir casos de prueba para comparar las dos implementaciones.

La IA también proporcionó fragmentos de código correctivo y una estructura de referencia para la función recursiva. Por ello, su contribución fue más allá de una explicación puramente conceptual: incluyó orientación concreta de implementación y depuración.

## 4. Evolución del proceso de aprendizaje

### Etapa 1 — Comprensión de la recurrencia

Al comienzo, el estudiante entendía que el problema requería bifurcarse, pero le costaba visualizar cómo continuar por las ramas. Mediante ejemplos y árboles, logró interpretar que cada llamada separa las particiones según si utiliza o no el límite actual.

### Etapa 2 — Construcción de la versión recursiva

El estudiante escribió su propia función y se encontró con dificultades en los casos base, las condiciones de bifurcación, las restas con enteros sin signo y el contador de llamadas. A través de iteraciones y correcciones, llegó a una estructura más sencilla: contar la llamada, resolver los casos base, limitar `maximo` a `n`, evaluar las dos ramas y sumar sus resultados.

### Etapa 3 — Diseño de la versión dinámica

El estudiante reconoció que la firma pública `particiones(n)` no recibía la tabla y planteó usar una función auxiliar para crearla y compartirla. También identificó que el resultado depende de dos parámetros, por lo que una tabla unidimensional no sería suficiente para la estrategia de memoización recursiva planteada.

### Etapa 4 — Uso de memoización

El estudiante adaptó el diseño a una matriz de opciones y trabajó con la idea de reutilizar resultados ya calculados. La orientación de la IA ayudó a distinguir entre estados aún no calculados (`None`) y resultados válidos, incluido cero (`Some(0)`).

### Etapa 5 — Revisión y validación

En las últimas iteraciones, el estudiante presentó versiones cada vez más cercanas a la estructura esperada y solicitó revisiones puntuales. La conversación terminó con una versión cuya lógica principal parecía consistente, aunque la compilación y las pruebas efectivas todavía debían comprobarse en el entorno del estudiante.

## 5. Distribución cualitativa de responsabilidades

No es posible asignar porcentajes objetivos y verificables de autoría a partir de una conversación. Una descripción más fiel es:

| Área | Aporte del estudiante | Aporte de la IA |
|---|---|---|
| Idea de separar el problema en ramas | La planteó y la fue refinando mediante preguntas. | La formalizó y explicó por qué las ramas no se solapan. |
| Escritura del código | Escribió y compartió sucesivas versiones propias. | Señaló fallos y propuso fragmentos correctivos. |
| Diseño de la tabla dinámica | Propuso almacenar resultados y usar una auxiliar. | Explicó por qué se necesitan dos dimensiones y cómo representar estados no calculados. |
| Depuración de Rust | Probó adaptar el código y reportó sus dificultades. | Orientó sobre `&mut`, `Option`, casos base, índices y restas seguras. |
| Comprensión y toma de decisiones | Preguntó, contrastó explicaciones y decidió cómo modificar su código. | Funcionó como tutor, explicador y revisor. |
| Verificación final | Debe ejecutar las pruebas y comprobar los resultados localmente. | Sugirió casos de prueba y criterios para comparar ambas versiones. |

**No se recomienda inventar una proporción numérica** (por ejemplo, “70 % estudiante y 30 % IA”), porque no existe una medida fiable que respalde esos porcentajes. La declaración debe describir las contribuciones reales y reconocer que la IA ofreció también fragmentos de implementación.

## 6. Limitaciones y verificaciones pendientes

La conversación documenta la evolución del código, pero no demuestra por sí sola que la última versión haya compilado o superado todas las pruebas. Antes de entregar el taller, conviene:

- Compilar el proyecto y corregir cualquier error restante.
- Comparar ambas funciones para entradas pequeñas, usando el mismo límite máximo (`maximo = n`) cuando se comparen las particiones completas.
- Comprobar los casos base `n = 0` y `maximo = 0`.
- Verificar resultados conocidos, como `p(4) = 5`, `p(5) = 7` y `p(10) = 42`.
- Ejecutar la versión dinámica para `n = 400` y registrar el resultado real.
- Medir las llamadas recursivas para los valores exigidos por el taller y justificar la complejidad con los datos observados, no con cifras inventadas.
- Confirmar las instrucciones oficiales sobre cómo declarar el uso de IA y añadir el enlace a la conversación si el curso lo exige.

## 7. Conclusión final

El uso de ChatGPT se orientó principalmente a **comprender la lógica, revisar decisiones de implementación y depurar progresivamente el código**. El estudiante mantuvo una participación activa: propuso el enfoque recursivo, escribió y modificó las funciones, planteó la necesidad de almacenar resultados, preguntó por las firmas y los préstamos de Rust, y utilizó la retroalimentación para corregir su implementación.

La IA contribuyó con explicaciones, diagramas, análisis de errores y fragmentos de código que ayudaron a concretar la solución. Por transparencia, debe reconocerse que el apoyo incluyó tanto orientación conceptual como sugerencias específicas de implementación; no sería exacto afirmar que la IA solo ofreció pistas generales.

El resultado educativo más importante fue la progresión desde una comprensión parcial de la bifurcación recursiva hacia un diseño que distingue casos base, estados `(n, maximo)` y resultados memorizados. La autoría final y la validez técnica deben respaldarse con la revisión personal del código, su ejecución y las pruebas realizadas por el estudiante.

## 8. Declaración breve sugerida

> Utilicé ChatGPT como apoyo para comprender la recurrencia de particiones, visualizar el árbol de llamadas, analizar casos base y depurar las implementaciones recursiva y dinámica en Rust. Escribí y modifiqué las versiones del código de manera iterativa, planteé la estructura de la memoización y utilicé las explicaciones para revisar errores relacionados con los parámetros, las referencias mutables, los tipos y la tabla de resultados. ChatGPT proporcionó explicaciones, sugerencias de depuración y algunos fragmentos de código correctivo. Antes de entregar, verificaré la compilación y los resultados con pruebas propias, y adjuntaré el enlace a la conversación si lo exige la pauta del taller.

---

**Antes de entregar:** ajusta esta declaración a las normas exactas del curso, añade el enlace de la conversación si es obligatorio y no afirmes que las pruebas fueron ejecutadas hasta haberlas realizado.
