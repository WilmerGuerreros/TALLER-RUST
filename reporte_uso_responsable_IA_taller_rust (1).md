# Reporte de uso responsable de inteligencia artificial

## Taller de Rust --- Recursión y programación dinámica

### 1. Propósito del uso de IA

Durante el desarrollo del taller se utilizó ChatGPT como apoyo para
comprender e implementar el problema de contar particiones de un número
entero. El objetivo fue aclarar la lógica recursiva y trasladarla a una
versión de programación dinámica mediante memoización, respetando las
firmas de las funciones solicitadas.

La IA se empleó principalmente como tutor técnico: explicó conceptos,
señaló errores de razonamiento y de Rust, propuso estructuras de
solución y ayudó a interpretar problemas relacionados con referencias
mutables, tipos y almacenamiento de resultados.

### 2. Aportes realizados por el estudiante

El estudiante mantuvo un papel activo durante el proceso y realizó
aportes concretos:

-   Planteó la idea de dividir el problema en dos ramas: considerar el
    uso del sumando máximo o continuar con un máximo menor.
-   Escribió y compartió sus propias versiones de `particiones_rec`,
    `particiones` y `calcular_dp`, en lugar de limitarse a solicitar una
    solución terminada.
-   Identificó dificultades de compilación relacionadas con el préstamo
    mutable (`&mut`) y probó alternativas para organizar las llamadas
    recursivas.
-   Propuso utilizar una tabla `dp` para conservar resultados y evitar
    repetir cálculos.
-   Reconoció que la firma pública `particiones(n: u32) -> u64` no
    permite recibir directamente una tabla como argumento y exploró la
    opción de una función auxiliar.
-   Adaptó progresivamente su código incorporando casos base,
    `maximo.min(n)`, una matriz bidimensional y `Option<u64>` para
    distinguir estados no calculados de resultados almacenados.
-   Solicitó aclaraciones cuando una explicación todavía no le resultaba
    clara y fue verificando sus cambios mediante nuevas versiones del
    código.

Estos aportes muestran participación activa, revisión iterativa y una
intención de comprender la lógica, no solamente obtener una respuesta
para copiar.

### 3. Aportes realizados por ChatGPT

ChatGPT contribuyó de forma sustancial en la orientación conceptual y
técnica. En concreto:

-   Explicó la recurrencia para contar particiones sin duplicar
    combinaciones: separar las particiones que no utilizan `maximo` de
    aquellas que sí lo utilizan.
-   Desarrolló ejemplos y árboles de llamadas para ilustrar la
    bifurcación, los casos base y cómo los resultados regresan y se
    suman.
-   Aclaró los casos base: `n == 0` devuelve `1`, mientras que `n > 0` y
    `maximo == 0` devuelve `0`.
-   Señaló riesgos de restas con `u32`, accesos fuera de rango y el
    orden necesario de las comprobaciones.
-   Explicó cómo compartir un contador mediante una referencia mutable y
    cómo una función auxiliar puede recibir una tabla mutable.
-   Introdujo `Option<u64>` (`None` para un estado no calculado y
    `Some(valor)` para un resultado calculado) y la técnica de
    memoización.
-   Proporcionó fragmentos de código, estructuras de referencia y una
    versión simplificada de la función recursiva, además de sugerencias
    de pruebas.

Por tanto, la IA no se limitó a corregir sintaxis: también influyó
directamente en la formulación de la solución y proporcionó ejemplos de
implementación. Es importante declarar esta contribución de manera
transparente.

### 4. Evolución del estudiante durante el proceso

La interacción avanzó por etapas:

1.  **Comprensión de la recurrencia.** Al principio, el reto era
    visualizar qué significaba fijar un sumando máximo, descontarlo del
    total pendiente y continuar con el resto. Los diagramas de árbol
    ayudaron a comprender la bifurcación y el retorno de resultados.
2.  **Implementación recursiva.** El estudiante escribió su función y
    revisó el contador de llamadas, los casos base, las restas y las
    condiciones de bifurcación. La revisión permitió identificar que el
    límite debía normalizarse antes de realizar la resta.
3.  **Diseño de la versión dinámica.** El estudiante propuso conservar
    resultados en una tabla y detectó la necesidad de separar la
    interfaz pública de la función auxiliar.
4.  **Memoización.** Se pasó de un vector unidimensional a una tabla
    bidimensional porque el estado depende tanto de `n` como de
    `maximo`. El uso de `Option<u64>` permitió distinguir un resultado
    igual a cero de un estado que todavía no se había calculado.
5.  **Revisión y validación pendiente.** La última versión compartida
    tenía la estructura lógica prevista para ambas funciones. Se
    sugirieron pruebas de resultados conocidos, casos límite y
    compilación; en la conversación no se presentó una salida de
    compilación o ejecución que certifique formalmente que el programa
    pasó las pruebas.

Esta progresión muestra que el estudiante fue refinando su comprensión,
detectando cuestiones de diseño y adaptando su implementación con base
en la retroalimentación.

### 5. Evaluación del uso responsable

El uso fue interactivo e iterativo: el estudiante compartió intentos
propios, pidió explicaciones de los errores, cuestionó las propuestas y
realizó modificaciones entre revisiones. Esto favoreció el aprendizaje
conceptual y la depuración.

Sin embargo, dado que ChatGPT proporcionó explicaciones detalladas y
fragmentos de código directamente aplicables ---incluida una estructura
completa para la función recursiva---, no sería correcto describir la
solución como desarrollada sin ayuda de IA. La contribución de la IA fue
**sustancial en la orientación y parcial en la implementación**; el
estudiante conservó un papel activo en el planteamiento, la adaptación
del código y la toma de decisiones.

La conversación permite valorar la participación y la evolución del
estudiante, pero no permite asignar porcentajes exactos de autoría ni
certificar cuánto del código final se escribió independientemente.
Tampoco sustituye la comprobación del código mediante compilación y
pruebas.

### 6. Conclusión

En este taller, ChatGPT se utilizó como apoyo para aprender recursión,
casos base, referencias mutables y memoización en Rust. El estudiante
aportó ideas iniciales, escribió varias versiones del código, detectó
problemas prácticos y refinó progresivamente su solución. La IA aportó
explicaciones, diagramas, correcciones y ejemplos de implementación que
tuvieron una influencia real en el desarrollo.

La valoración final es que hubo una participación activa y una evolución
visible en la comprensión del problema, junto con una asistencia de IA
significativa. Para mantener la transparencia académica, este reporte
declara tanto los aportes del estudiante como los de ChatGPT, sin
atribuir todo el desarrollo a una sola parte. Antes de entregar el
taller, se recomienda compilar y ejecutar las funciones, comparar sus
resultados y registrar las pruebas realizadas.

------------------------------------------------------------------------

**Herramienta utilizada:** ChatGPT\
**Tipo de asistencia:** explicación conceptual, diagramas, revisión de
lógica, depuración orientativa y ejemplos de código.\
**Estado de validación:** revisión conceptual realizada durante la
conversación; la compilación y las pruebas finales deben confirmarse en
el entorno del taller.
