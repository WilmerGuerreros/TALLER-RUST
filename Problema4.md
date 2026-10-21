# Problema 4 — Variante 2: `goldbach`

## Enunciado

La conjetura de Goldbach dice que todo número par mayor que 2 se puede escribir como suma de dos primos.

La variante V2 pide:

```rust
pub fn goldbach(n: u64) -> u64
```

Devuelve **de cuántas formas** se puede escribir `n` como `p + q`, donde:

- `p` y `q` son primos.
- `p ≤ q` (no se cuentan las parejas repetidas en distinto orden).

Ejemplo: `goldbach(100)` → **6**.

Tamaño máximo: `n ≤ 10⁸`.

---

## Código

```rust
use std::io;

pub fn goldbach(n: u64) -> u64 {
    if n < 4 {
        return 0;
    }

    let limite = n as usize;

    // true = primo, false = no primo
    let mut es_primo = vec![true; limite + 1];
    es_primo[0] = false;
    es_primo[1] = false;

    // Criba de Eratóstenes
    let mut i = 2;
    while i * i <= limite {
        if es_primo[i] {
            let mut j = i * i;
            while j <= limite {
                es_primo[j] = false;
                j += i; // múltiplos de i
            }
        }
        i += 1;
    }

    // Contar parejas p + q = n, con p <= q
    let mut contador = 0u64;
    let mut p: u64 = 2;
    while p <= n / 2 {
        let q = n - p;
        if es_primo[p as usize] && es_primo[q as usize] {
            contador += 1;
        }
        p += 1;
    }

    contador
}

fn main() {
    println!("Ingrese el número: ");
    let qqq: u64;
    let mut a: String = String::new();
    io::stdin().read_line(&mut a).unwrap();
    qqq = a.trim().parse().unwrap();
    println!(
        "El número {} se puede escribir de {} formas.",
        qqq,
        goldbach(qqq)
    );
}
```

---

## Idea general

1. Construir una **criba de Eratóstenes** hasta `n` para saber qué números son primos.
2. Recorrer los posibles `p` desde `2` hasta `n / 2`.
3. Para cada `p`, calcular `q = n - p`.
4. Si ambos son primos, contar la pareja.

Recorrer solo hasta `n / 2` garantiza que `p ≤ q`, así no se cuentan `3 + 7` y `7 + 3` como dos formas distintas.

---

## Lógica paso a paso

### 1. Caso borde

```rust
if n < 4 {
    return 0;
}
```

No hay formas de escribir 0, 2 o 3 como suma de dos primos:

- `2 = 1 + 1`, pero 1 no es primo.
- `3 = 1 + 2`, pero 1 no es primo.

El primer caso válido es `4 = 2 + 2`.

### 2. Crear la criba

```rust
let limite = n as usize;

let mut es_primo = vec![true; limite + 1];
es_primo[0] = false;
es_primo[1] = false;
```

- `vec![true; limite + 1]` crea un `Vec<bool>` con `limite + 1` posiciones.
- Al inicio todos se consideran primos.
- `0` y `1` no son primos, así que se marcan como `false`.

`Vec<bool>` guarda los valores de forma compacta (bits), no un byte por elemento, así que para `n = 10⁸` ocupa aproximadamente 12.5 MB.

### 3. Criba de Eratóstenes

```rust
let mut i = 2;
while i * i <= limite {
    if es_primo[i] {
        let mut j = i * i;
        while j <= limite {
            es_primo[j] = false;
            j += i;
        }
    }
    i += 1;
}
```

- Se recorren los candidatos `i` desde `2` hasta `√limite`.
- Si `i` sigue siendo primo, se marcan como no primos sus múltiplos.
- Se empieza en `j = i * i` porque los múltiplos más pequeños (`2i`, `3i`, ...) ya fueron marcados por primos anteriores.
- Se avanza con `j += i`, que es lo correcto para recorrer los múltiplos de `i`.

Ejemplo con `i = 3`:

```text
9, 12, 15, 18, 21, 24, 27, ...
```

### 4. Contar las parejas

```rust
let mut contador = 0u64;
let mut p: u64 = 2;
while p <= n / 2 {
    let q = n - p;
    if es_primo[p as usize] && es_primo[q as usize] {
        contador += 1;
    }
    p += 1;
}
```

- `p` empieza en `2` para incluir el caso `4 = 2 + 2`.
- `p` llega hasta `n / 2` para que `p ≤ q`.
- `q = n - p`.
- Solo se cuenta si `p` y `q` son primos según la criba.

### 5. Devolver el contador

```rust
contador
```

---

## Ejemplo: `goldbach(100)`

Las parejas de primos que suman 100 son:

| p | q | p + q |
|---|---|---|
| 3 | 97 | 100 |
| 11 | 89 | 100 |
| 17 | 83 | 100 |
| 29 | 71 | 100 |
| 41 | 59 | 100 |
| 47 | 53 | 100 |

Resultado: **6**.

Otro ejemplo pequeño: `goldbach(4)` → `2 + 2` → **1**.

---

## Detalles de sintaxis de Rust importantes

- **`n as usize`**: los índices y tamaños de un `Vec` usan `usize`, no `u64`.
- **`vec![true; limite + 1]`**: crea un `Vec<bool>` con `limite + 1` elementos, todos `true`.
- **`Vec<bool>`**: está optimizado para ocupar bits, no bytes, lo que ayuda mucho con `n = 10⁸`.
- **`i * i <= limite`**: evita usar `sqrt` y `f64`; además, con `n ≤ 10⁸`, `i` es como máximo `10⁴`, así que no hay desbordamiento.
- **`j = i * i`**: se empieza a marcar desde el cuadrado de `i`; los múltiplos menores ya fueron marcados antes.
- **`j += i`**: recorre los múltiplos de `i`. Si se usara `j += j`, se duplicaría `j` y se saltarían múltiplos.
- **`p as usize` y `q as usize`**: conversión de `u64` a `usize` para indexar la criba.
- **`p <= n / 2`**: garantiza que `p` sea el menor de la pareja y evita contar el mismo par dos veces.
- **`p += 1`**: se revisan todos los candidatos; no hace falta saltar de dos en dos porque la criba ya descarta los pares no primos.

---

## Qué se corrigió respecto a la versión anterior

La primera versión tenía varios problemas:

1. **El bucle de Goldbach empezaba en 3.**
   Para `n = 4`, la única pareja es `2 + 2`, pero `3 <= 4 / 2` es falso y el bucle no se ejecutaba. Por eso `goldbach(4)` daba 0.

2. **La criba usaba `j += j` en lugar de `j += i`.**
   Eso duplicaba el índice y no marcaba todos los múltiplos. Por ejemplo, para `i = 3` se marcaban `9, 18, 36, ...` y se saltaban `15, 21, 27, ...`, que quedaban como primos por error.

3. **`criba[1] = 1` marcaba el 1 como primo.**
   1 no es primo; debe ser `false` desde el inicio.

4. **La criba tenía tamaño fijo `10_000_001`.**
   El problema permite `n ≤ 10⁸`, así que el vector debe llegar hasta `n`; de lo contrario se indexa fuera de rango.

5. **`Vec<u64>` ocupaba demasiado.**
   Para `10⁸` elementos, un `Vec<u64>` usaría aproximadamente 800 MB. `Vec<bool>` reduce el uso a unos 12.5 MB.

---

## Complejidad

- **Tiempo:** `O(n log log n)` por la criba de Eratóstenes, más `O(n)` por el conteo de parejas.
- **Memoria:** `O(n)` bits por el `Vec<bool>` (aproximadamente 12.5 MB para `n = 10⁸`).
- Para el tamaño máximo conviene compilar y probar con `cargo test --release`.

---

## Pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casos_pequenos() {
        assert_eq!(goldbach(0), 0);
        assert_eq!(goldbach(2), 0);
        assert_eq!(goldbach(3), 0);
        assert_eq!(goldbach(4), 1);
        assert_eq!(goldbach(6), 1);
        assert_eq!(goldbach(8), 1);
        assert_eq!(goldbach(10), 2);
    }

    #[test]
    fn ejemplo_del_enunciado() {
        assert_eq!(goldbach(100), 6);
    }

    #[test]
    fn otros_pares() {
        assert_eq!(goldbach(14), 2); // 3 + 11, 7 + 7
        assert_eq!(goldbach(16), 2); // 3 + 13, 5 + 11
    }
}
```

Notas:

- `goldbach(4)` debe dar **1** por la pareja `2 + 2`.
- `goldbach(10)` debe dar **2** por `3 + 7` y `5 + 5`.
- `goldbach(100)` debe dar **6**, como indica el taller.

---

## Nota para la entrega

- La función que se entrega va en `src/lib.rs` y debe ser exactamente:
  ```rust
  pub fn goldbach(n: u64) -> u64
  ```
- `main` es solo para probar por consola; no es necesario para las pruebas del taller.
- El algoritmo no usa `f64` ni pruebas número por número: usa una criba y luego cuenta las parejas.

---

## Resumen

`goldbach` construye una criba de primos hasta `n` con `Vec<bool>`, marca los múltiplos con `j += i` y luego recorre `p` desde `2` hasta `n / 2`. Para cada `p`, calcula `q = n - p` y cuenta la pareja si ambos son primos. Así se obtienen todas las formas de escribir `n` como suma de dos primos, sin repetir parejas y sin probar cada número con divisiones.
