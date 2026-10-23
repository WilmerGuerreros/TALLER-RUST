# Problema 8 — Variante 2: particiones de un entero

## Enunciado

El Problema 8 pide dos versiones de la misma función:

1. Una **recursiva directa**, que suma 1 a `llamadas` en cada ejecución.
2. Una **eficiente con programación dinámica**.

Para la variante 2, las firmas exactas son:

```rust
pub fn particiones_rec(n: u32, maximo: u32, llamadas: &mut u64) -> u64
pub fn particiones(n: u32) -> u64
```

`particiones_rec(n, maximo, ...)` cuenta las formas de escribir `n` como suma de enteros positivos, **sin importar el orden**, usando partes de tamaño **como máximo `maximo`**.

`particiones(n)` cuenta lo mismo, pero sin restricción de tamaño: equivale a `particiones_rec(n, n, ...)`.

Ejemplo del enunciado:

```text
p(5) = 7
```

Las siete particiones de 5 son:

```text
5
4 + 1
3 + 2
3 + 1 + 1
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1
```

Para la versión eficiente, el taller pide:

```text
particiones(400)
```

---

## Código completo

```rust
use std::io;

pub fn particiones_rec(n: u32, maximo: u32, llamadas: &mut u64) -> u64 {
    *llamadas += 1;

    if n == 0 {
        return 1;
    }

    if maximo == 0 {
        return 0;
    }

    let maximo = maximo.min(n);

    let izquierda = particiones_rec(n, maximo - 1, llamadas);

    let derecha = particiones_rec(n - maximo, maximo, llamadas);

    izquierda + derecha
}

pub fn particiones(n: u32) -> u64 {
    let t = n as usize + 1;
    let mut dinamico: Vec<Vec<Option<u64>>> = vec![vec![None; t]; t];

    calcular_dp(n, n, &mut dinamico)
}

pub fn calcular_dp(n: u32, maximo: u32, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if n == 0 {
        return 1;
    }

    if maximo == 0 {
        return 0;
    }

    let maximo = maximo.min(n);

    let i = n as usize;
    let j = maximo as usize;

    if let Some(resultado) = dp[i][j] {
        return resultado;
    }

    let izquierda = calcular_dp(n, maximo - 1, dp);
    let derecha = calcular_dp(n - maximo, maximo, dp);

    dp[i][j] = Some(izquierda + derecha);

    dp[i][j].unwrap()
}

fn main() {
    println!("Inserte el número K [máximo sumando usable]: ");
    let mut a: String = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let k: u32 = a.trim().parse().unwrap();
    println!("Inserte el número n a ramificar: ");
    a = "".to_string();
    io::stdin().read_line(&mut a).unwrap();
    let lim: u32 = a.trim().parse().unwrap();
    let mut l: u64 = 0;
    println!(
        "Las particiones totales son {}",
        particiones_rec(lim, k, &mut l)
    );
    println!("Particiones con DP: {}", particiones(lim));
}
```

---

## 1. Versión recursiva: `particiones_rec`

### ¿Qué significa cada parámetro?

- `n`: el número que se quiere partir.
- `maximo`: el tamaño máximo que puede tener cada sumando.
- `llamadas`: contador compartido; se incrementa en cada ejecución de la función.

Por ejemplo:

```text
particiones_rec(5, 2, ...) = 3
```

Porque las únicas particiones de 5 con sumandos de tamaño ≤ 2 son:

```text
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1
```

En cambio:

```text
particiones_rec(5, 5, ...) = 7
```

que es `p(5)`.

### Casos base

```rust
if n == 0 {
    return 1;
}
```

Si `n` vale 0, hay **una** forma de representarlo: la partición vacía. Por eso se devuelve 1.

```rust
if maximo == 0 {
    return 0;
}
```

Si `n > 0` pero el tamaño máximo permitido es 0, no se puede usar ningún entero positivo. Por eso se devuelve 0.

El orden importa: primero se comprueba `n == 0`. Así, `particiones_rec(0, 0, ...)` devuelve 1, que es lo correcto, y no 0.

### ¿Por qué se hace `maximo.min(n)`?

```rust
let maximo = maximo.min(n);
```

Si `maximo` es mayor que `n`, ningún sumando puede superar a `n` de todos modos. Por ejemplo, en `particiones_rec(5, 10)` no tiene sentido permitir partes de tamaño 10: no se pueden usar.

Esta línea:

- evita llamadas y estados inútiles;
- garantiza que `maximo <= n`;
- en la versión DP, garantiza que el índice `j` no se salga de la tabla.

### Las dos ramas de la recursión

```rust
let izquierda = particiones_rec(n, maximo - 1, llamadas);
let derecha = particiones_rec(n - maximo, maximo, llamadas);
```

Toda partición de `n` con partes ≤ `maximo` cae en uno de estos dos grupos:

1. **No usa el valor `maximo`**.
   Entonces todas sus partes son ≤ `maximo - 1`.
   Eso es `izquierda`.

2. **Usa al menos un `maximo`**.
   Se quita uno de esos `maximo` y queda una partición de `n - maximo` con partes ≤ `maximo`.
   Eso es `derecha`.

Los dos grupos no se solapan y juntos cubren todas las particiones. Por eso se suman:

```rust
izquierda + derecha
```

Esta forma de elegir evita contar permutaciones distintas del mismo conjunto de sumandos. Por ejemplo, `3 + 2` y `2 + 3` son la misma partición y se cuentan una sola vez.

### Traza de valores para `p(5)`

```text
rec(5, 5) = rec(5, 4) + rec(0, 5) = 6 + 1 = 7

rec(5, 4) = rec(5, 3) + rec(1, 4) = 5 + 1 = 6

rec(5, 3) = rec(5, 2) + rec(2, 3) = 3 + 2 = 5

rec(5, 2) = rec(5, 1) + rec(3, 2) = 1 + 2 = 3

rec(5, 1) = rec(5, 0) + rec(4, 1) = 0 + 1 = 1
```

Interpretación:

- `rec(0, 5) = 1`: la partición `[5]`.
- `rec(1, 4) = 1`: la partición `[4 + 1]`.
- `rec(2, 3) = 2`: `[3 + 2]` y `[3 + 1 + 1]`.
- `rec(3, 2) = 2`: `[2 + 2 + 1]` y `[2 + 1 + 1 + 1]`.
- `rec(4, 1) = 1`: `[1 + 1 + 1 + 1]`.

La traza anterior muestra los **valores**, no todas las llamadas. El conteo total de llamadas para `particiones_rec(5, 5, ...)` es **37**.

### ¿Qué cuenta exactamente `llamadas`?

El incremento está al inicio:

```rust
*llamadas += 1;
```

Por eso se cuenta **cada invocación**, incluyendo:

- las llamadas que llegan a `n == 0`;
- las llamadas que llegan a `maximo == 0`.

Si se moviera el incremento después de los casos base, los números serían distintos. Los valores de este documento usan el contador tal como está en el código.

### Tabla de llamadas

Para `particiones_rec(n, n, &mut llamadas)` empezando con `llamadas = 0`:

| `n` | `p(n)` | Llamadas |
|---:|---:|---:|
| 5 | 7 | 37 |
| 10 | 42 | 277 |
| 20 | 627 | 5 427 |
| 25 | 1 958 | 18 591 |
| 30 | 5 604 | 57 257 |

Los valores pedidos en `respuestas.md` para la variante 2 son:

```text
n = 10  →  277 llamadas
n = 20  →  5 427 llamadas
n = 25  →  18 591 llamadas
```

### Complejidad de la versión recursiva

La versión recursiva **no tiene memoización**: si un estado `(n, maximo)` aparece varias veces, se vuelve a calcular completo.

Su costo es el número de nodos del árbol de llamadas. Ese número crece mucho más rápido que `n`:

- de `n = 10` a `n = 20`, `n` se duplica, pero las llamadas pasan de 277 a 5 427: se multiplican por **19,6**;
- de `n = 20` a `n = 25`, `n` sube 25 %, pero las llamadas pasan de 5 427 a 18 591: se multiplican por **3,4**.

Ese crecimiento es mucho más rápido que cualquier polinomio pequeño y es incompatible con `n = 400`. La profundidad de la recursión sí es manejable, pero la cantidad de llamadas no.

> **Importante:** no ejecutes `particiones_rec(400, 400, ...)`. El programa parecerá colgarse porque el árbol de llamadas es gigantesco. Para medir llamadas usa `n = 10`, `20` o `25`. Para `n = 400` usa solo `particiones(400)`.

---

## 2. Versión eficiente: `particiones` con programación dinámica

### Idea general

La versión DP hace exactamente la misma recursión, pero guarda los resultados ya calculados en una tabla.

```rust
pub fn particiones(n: u32) -> u64 {
    let t = n as usize + 1;
    let mut dinamico: Vec<Vec<Option<u64>>> = vec![vec![None; t]; t];

    calcular_dp(n, n, &mut dinamico)
}
```

La tabla es de tamaño `(n + 1) × (n + 1)`:

- fila `i`: valor de `n`;
- columna `j`: valor de `maximo`;
- `None`: todavía no se calculó;
- `Some(resultado)`: ya está calculado.

Se llama con `maximo = n` porque cualquier partición de `n` usa partes de tamaño ≤ `n`.

### `calcular_dp`

```rust
pub fn calcular_dp(n: u32, maximo: u32, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if n == 0 {
        return 1;
    }

    if maximo == 0 {
        return 0;
    }

    let maximo = maximo.min(n);

    let i = n as usize;
    let j = maximo as usize;

    if let Some(resultado) = dp[i][j] {
        return resultado;
    }

    let izquierda = calcular_dp(n, maximo - 1, dp);
    let derecha = calcular_dp(n - maximo, maximo, dp);

    dp[i][j] = Some(izquierda + derecha);

    dp[i][j].unwrap()
}
```

Los casos base son los mismos que en la versión recursiva.

Después se normaliza `maximo` y se calculan los índices:

```rust
let i = n as usize;
let j = maximo as usize;
```

Como `maximo <= n`, se cumple `j <= i <= n`, así que `dp[i][j]` siempre está dentro de la tabla.

La memoización es esta parte:

```rust
if let Some(resultado) = dp[i][j] {
    return resultado;
}
```

Si el estado ya fue calculado, se devuelve directamente y no se vuelve a ramificar.

Si no estaba calculado, se calculan las dos ramas y se guarda el resultado:

```rust
dp[i][j] = Some(izquierda + derecha);
```

Finalmente:

```rust
dp[i][j].unwrap()
```

es seguro porque en la línea anterior se acaba de asignar `Some(...)`. También se podría devolver directamente `izquierda + derecha`; el `unwrap` no es incorrecto, solo es un paso extra.

### ¿Por qué no se desborda `u64` con `n = 400`?

El resultado es:

```text
particiones(400) = 6727090051741041926
```

o, con separadores:

```text
6 727 090 051 741 041 926
```

El máximo de `u64` es:

```text
18 446 744 073 709 551 615
```

Como `6 727 090 051 741 041 926 < 18 446 744 073 709 551 615`, el resultado cabe en `u64`.

Además, todos los valores intermedios `p(i, j)` con `i <= 400` son menores o iguales que `p(400)`, así que las sumas intermedias tampoco se salen de `u64`.

> Para `n` mucho mayor, `u64` podría no alcanzar. Por ejemplo, `p(500)` ya supera el máximo de `u64`. Para el taller basta con `n = 400`.

### Complejidad de la versión DP

La tabla tiene como máximo:

```text
(n + 1) × (n + 1) = (n + 1)²
```

estados. Cada estado **no base** se calcula una sola vez y hace trabajo constante; los casos base son `O(1)`. Por eso:

- **Tiempo: O(n²)**.
- **Memoria: O(n²)**.

Para `n = 400`:

```text
401² = 160 801 estados
```

es una tabla pequeña; la versión DP responde sin problema en menos de un segundo.

La profundidad de la recursión del DP también es manejable: cada llamada reduce `n` o `maximo`, así que la pila no crece de forma descontrolada.

---

## 3. Comparación de las dos versiones

| `n` | `p(n)` | Llamadas recursivas | Estados DP (cota) |
|---:|---:|---:|---:|
| 10 | 42 | 277 | 121 |
| 20 | 627 | 5 427 | 441 |
| 25 | 1 958 | 18 591 | 676 |
| 30 | 5 604 | 57 257 | 961 |
| 400 | 6 727 090 051 741 041 926 | no ejecutar | 160 801 |

La diferencia clave:

- La recursiva **recalcula** los mismos estados muchas veces; su árbol de llamadas crece muy rápido.
- La DP **calcula cada estado no base una sola vez**; por eso pasa de un crecimiento explosivo a `O(n²)`.

Para `n = 25`, la recursiva todavía es rápida (18 591 llamadas). Para `n = 400`, la recursiva es inviable; la DP es la que se usa.

---

## 4. Diferencia entre `particiones_rec(n, k, ...)` y `particiones(n)`

Esto es importante para entender el `main`.

- `particiones_rec(n, k, ...)` cuenta las particiones de `n` usando partes de tamaño **≤ k**.
- `particiones(n)` cuenta todas las particiones de `n`, sin restricción. Es equivalente a `particiones_rec(n, n, ...)`.

Por eso, si `k < n`, los resultados son distintos:

```text
particiones_rec(5, 2, ...) = 3
particiones(5) = 7
```

Las 3 particiones de 5 con partes ≤ 2 son:

```text
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1
```

En cambio, `particiones(5)` incluye también:

```text
5
4 + 1
3 + 2
3 + 1 + 1
```

Por eso el `main` pide `K` y `n` por separado:

```rust
particiones_rec(lim, k, &mut l)
particiones(lim)
```

Si quieres que ambos resultados coincidan, ingresa `K = n`. Por ejemplo:

```text
K = 5
n = 5
```

Ambos deben dar 7.

Si ingresas `K = 2` y `n = 5`, el primero dará 3 y el segundo 7. **No es un error**: son funciones con restricciones distintas.

---

## 5. Pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_p5() {
        let mut llamadas = 0;
        assert_eq!(particiones_rec(5, 5, &mut llamadas), 7);
        assert_eq!(llamadas, 37);
        assert_eq!(particiones(5), 7);
    }

    #[test]
    fn casos_base() {
        let mut llamadas = 0;
        assert_eq!(particiones_rec(0, 0, &mut llamadas), 1);
        assert_eq!(llamadas, 1);

        let mut llamadas = 0;
        assert_eq!(particiones_rec(5, 0, &mut llamadas), 0);
        assert_eq!(llamadas, 1);

        assert_eq!(particiones(0), 1);
        assert_eq!(particiones(1), 1);
    }

    #[test]
    fn maximo_restrictivo() {
        let mut llamadas = 0;
        assert_eq!(particiones_rec(5, 2, &mut llamadas), 3);
    }

    #[test]
    fn tabla_10_20_25() {
        let casos = [
            (10, 42, 277),
            (20, 627, 5_427),
            (25, 1_958, 18_591),
        ];

        for (n, valor, llamadas_esperadas) in casos {
            let mut llamadas = 0;
            assert_eq!(particiones_rec(n, n, &mut llamadas), valor);
            assert_eq!(llamadas, llamadas_esperadas);
            assert_eq!(particiones(n), valor);
        }
    }

    #[test]
    fn p400() {
        assert_eq!(particiones(400), 6_727_090_051_741_041_926);
    }
}
```

Para ejecutar:

```bash
cargo test
cargo test --release
```

---

## 6. Qué escribir en `respuestas.md`

Una forma directa de responder lo que pide el taller:

```markdown
## Problema 8 — V2

Contador de `particiones_rec` incrementado al inicio de cada llamada,
incluyendo los casos base.

| n | p(n) | llamadas |
|---:|---:|---:|
| 10 | 42 | 277 |
| 20 | 627 | 5 427 |
| 25 | 1 958 | 18 591 |

`particiones(400) = 6727090051741041926`
(6 727 090 051 741 041 926).

Complejidad:

- Versión recursiva: el costo es el número de llamadas del árbol.
  Crece mucho más rápido que n²: de n = 10 a n = 20, n se duplica
  y las llamadas se multiplican por aproximadamente 19,6. No es viable
  para n = 400.
- Versión DP: O(n²) en tiempo y O(n²) en memoria. Cada estado no base
  (n, maximo) se calcula una sola vez; los casos base son O(1). Para
  n = 400 hay como máximo 401² = 160 801 estados.
```

---

## 7. Preguntas probables de defensa

### ¿Por qué `n == 0` devuelve 1?

Porque hay exactamente una forma de escribir 0 como suma de enteros positivos: la **partición vacía**. Es una convención necesaria para que la recursión funcione.

### ¿Por qué `maximo == 0` devuelve 0?

Porque si `n > 0` y no se permite ningún sumando positivo, no hay ninguna forma de formar `n`.

### ¿Por qué el orden de los casos base importa?

Porque `particiones_rec(0, 0, ...)` debe devolver 1. Si se comprobara primero `maximo == 0`, devolvería 0. El caso `n == 0` tiene prioridad.

### ¿Por qué se usa `maximo.min(n)`?

Porque ninguna parte puede ser mayor que `n`. Además, en la DP evita que el índice `j` se salga de la tabla.

### ¿Por qué las dos ramas no se superponen?

Una partición o **no usa** el valor `maximo`, o **usa al menos uno**. No hay una tercera posibilidad. Si lo usa, al quitar una copia queda una partición de `n - maximo` con partes ≤ `maximo`. Por eso se pueden sumar sin miedo a contar dos veces.

### ¿Por qué `particiones(n)` llama con `maximo = n`?

Porque cualquier sumando de una partición de `n` es como máximo `n`. Usar `maximo = n` permite todas las partes posibles.

### ¿Por qué la recursiva es lenta y la DP no?

La recursiva recalcula el mismo estado muchas veces. La DP guarda cada resultado en `dp[i][j]` y lo reutiliza. La tabla tiene `(n + 1)²` estados; cada estado no base se calcula una sola vez y los casos base son `O(1)`.

### ¿Por qué la DP es `O(n²)`?

Porque hay como máximo `(n + 1)²` pares `(n, maximo)` y cada uno se resuelve con dos llamadas y una suma, es decir, trabajo constante.

### ¿Por qué se usa `u64`?

Porque `p(400)` es un número grande:

```text
6 727 090 051 741 041 926
```

que no cabe en `u32`, pero sí en `u64`.

### ¿Qué hace `Option<u64>` en la tabla?

`None` significa “este estado todavía no se calculó”. `Some(x)` significa “ya se calculó y vale `x`”. Es una forma de distinguir un resultado pendiente de un resultado que vale 0.

### ¿Qué cuenta `llamadas`?

Cada invocación de `particiones_rec`, incluyendo las que caen en los casos base. El incremento está antes de los `if`, así que también se cuentan las llamadas a `(0, m)` y `(n, 0)`.

### ¿Se puede usar `unwrap` en este problema?

Sí. La prohibición de `unwrap` y `expect` es del Problema 10, no del Problema 8. Además, en `dp[i][j].unwrap()` es seguro porque se acaba de asignar `Some(...)`. Aun así, se puede devolver directamente `izquierda + derecha`.

---

## 8. Observaciones sobre el código

- `calcular_dp` no es una función pedida por el enunciado; puede quedarse `pub` o hacerse privada. Si se hace privada, `particiones` sigue funcionando.
- `dp[i][j].unwrap()` es seguro, pero se puede reemplazar por `izquierda + derecha` para evitar el `unwrap`.
- `a = "".to_string()` funciona para limpiar el buffer, aunque `a.clear()` es más directo.
- El `main` calcula `l` (las llamadas) pero no lo imprime. Si quieres verlo, agrega:

  ```rust
  println!("Llamadas recursivas: {}", l);
  ```

- El `main` llama a la versión recursiva antes que a la DP. Si ingresas `n = 400`, se quedará atascado en la recursiva y nunca llegará a la DP. Para probar `n = 400`, usa solo `particiones(400)` o comenta la llamada recursiva.
- Para la entrega, las funciones pedidas deben estar en `src/lib.rs` con sus firmas exactas y `pub`. El `main` puede ir en `src/main.rs` o usarse solo para pruebas locales.

---

## Resumen

- `particiones_rec(n, maximo, ...)` cuenta las particiones de `n` con partes ≤ `maximo`.
- La recursión divide el problema en “no usar `maximo`” y “usar al menos un `maximo`”.
- `llamadas` se incrementa al inicio de cada ejecución, incluyendo los casos base.
- Para la variante 2, las llamadas son 277, 5 427 y 18 591 para `n = 10, 20, 25`.
- `particiones(n)` usa memoización en una tabla `(n + 1) × (n + 1)`.
- La versión DP es `O(n²)` en tiempo y memoria.
- `particiones(400) = 6727090051741041926`, que cabe en `u64`.
- La versión recursiva sirve para entender y contar llamadas, pero no para `n = 400`.
