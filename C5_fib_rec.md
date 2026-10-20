# C5 — `fib_rec`: lógica paso a paso

## Enunciado

```rust
pub fn fib_rec(n: u32, llamadas: &mut u64) -> u64
```

Versión **recursiva directa** de Fibonacci que, además, cuenta cuántas veces se llama a la función.

Ejemplo del taller: `fib_rec(10, ...)` devuelve **55** y hace **177 llamadas**.

---

## Código final

```rust
use std::io;

pub fn fib_rec(n: u32, llamadas: &mut u64) -> u64 {
    *llamadas += 1;
    if n <= 1 {
        n as u64
    } else {
        fib_rec(n - 1, llamadas) + fib_rec(n - 2, llamadas)
    }
}

fn main() {
    let mut llamadas: u64 = 0;
    let mut makanaky = String::new();
    println!("Ingresa el número papay: ");
    io::stdin().read_line(&mut makanaky).unwrap();
    let a: u32 = makanaky.trim().parse().unwrap();
    let resultado = fib_rec(a, &mut llamadas);
    println!("Resultado: {} con {} llamadas", resultado, llamadas);
}
```

---

## Lógica paso a paso

1. **Incrementar el contador al entrar**
   ```rust
   *llamadas += 1;
   ```
   `llamadas` es `&mut u64` (una referencia mutable). El `*` accede al valor apuntado, así que `*llamadas += 1` suma 1 al contador original de `main`. Se ejecuta **en cada llamada**, incluida la del caso base.

2. **Caso base**
   ```rust
   if n <= 1 {
       n as u64
   }
   ```
   - `fib(0) = 0` y `fib(1) = 1`.
   - Se devuelve `n as u64` porque `n` es `u32` y la función retorna `u64`.
   - Al ser la **última expresión** del bloque y no llevar `;`, ese valor es lo que retorna la función.

3. **Caso recursivo**
   ```rust
   } else {
       fib_rec(n - 1, llamadas) + fib_rec(n - 2, llamadas)
   }
   ```
   - Se calcula `fib(n-1)` y `fib(n-2)` y se suman.
   - Se pasa `llamadas` **tal cual** (no `&mut llamadas`), porque ya es una referencia `&mut u64`; volver a prestarla sería prestar un préstamo.
   - Las dos llamadas se ejecutan una después de la otra, así que sus préstamos mutables no se solapan y compila sin problema.
   - La suma es la última expresión del bloque → es el valor de retorno.

---

## Por qué `fib_rec(10)` da 55 y 177 llamadas

Con `n <= 1 → n`, la secuencia es la estándar:

`fib(0)=0, fib(1)=1, fib(2)=1, fib(3)=2, fib(4)=3, fib(5)=5, fib(6)=8, fib(7)=13, fib(8)=21, fib(9)=34, fib(10)=55`

Y las llamadas siguen `L(0)=1, L(1)=1, L(n)=1+L(n-1)+L(n-2)`:

| n | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| fib(n) | 0 | 1 | 1 | 2 | 3 | 5 | 8 | 13 | 21 | 34 | **55** |
| llamadas | 1 | 1 | 3 | 5 | 9 | 15 | 25 | 41 | 67 | 109 | **177** |

Fíjate que el número de llamadas **depende del caso base**. Si se corta antes (por ejemplo `n == 1 || n == 2 → 1`), el árbol cambia y el conteo baja (109 para n=10). Por eso el taller especifica **55 y 177**: corresponden a `n <= 1 → n`.

---

## Detalles de sintaxis de Rust importantes

- **`&mut u64`**: referencia mutable a un `u64`. Permite que la función modifique el contador que vive en `main`.
- **`*llamadas`**: el operador `*` **desreferencia** la referencia; accede al valor real. Por eso se escribe `*llamadas += 1`.
- **`*llamadas += 1`**: equivale a `*llamadas = *llamadas + 1;`.
- **`n as u64`**: conversión explícita de `u32` a `u64`. Rust **no** convierte tipos automáticamente; sin el `as`, el compilador da `mismatched types` (esperaba `u64`, encontró `u32`).
- **Última expresión sin `;`**: es el valor que devuelve el bloque/función. Si le pusieras `;`, devolvería `()` y no compilaría.
- **Recursión**: la función se llama a sí misma pasando `llamadas` (que es una referencia compartida de forma secuencial, no simultánea).
- **`if/else` como expresión**: en Rust, un `if/else` puede producir un valor; aquí ambos bloques producen `u64`.

---

## El error típico: `mismatched types`

Si en el caso base escribes solo `n`:

```rust
if n <= 1 {
    n          // ERROR: expected u64, found u32
}
```

El compilador se queja porque la función promete devolver `u64` y `n` es `u32`. Solución: `n as u64` (o `u64::from(n)`).

---

## Casos de prueba (con `#[cfg(test)]`, no tocan el resto del proyecto)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_0() {
        let mut llamadas = 0;
        assert_eq!(fib_rec(0, &mut llamadas), 0);
        assert_eq!(llamadas, 1);
    }

    #[test]
    fn fib_1() {
        let mut llamadas = 0;
        assert_eq!(fib_rec(1, &mut llamadas), 1);
        assert_eq!(llamadas, 1);
    }

    #[test]
    fn fib_2() {
        let mut llamadas = 0;
        assert_eq!(fib_rec(2, &mut llamadas), 1);
        assert_eq!(llamadas, 3);
    }

    #[test]
    fn fib_5() {
        let mut llamadas = 0;
        assert_eq!(fib_rec(5, &mut llamadas), 5);
        assert_eq!(llamadas, 15);
    }

    #[test]
    fn fib_10_del_enunciado() {
        let mut llamadas = 0;
        assert_eq!(fib_rec(10, &mut llamadas), 55);
        assert_eq!(llamadas, 177);
    }
}
```

- Cada test reinicia `llamadas` a `0` antes de llamar, porque el contador **no se reinicia solo**.
- Se pasa `&mut llamadas` en la llamada inicial desde el test.
- El test de `n = 10` verifica exactamente el ejemplo del taller.

---

## Resumen

`fib_rec` incrementa el contador en cada entrada, devuelve `n` cuando `n <= 1` (caso base) y, en otro caso, se llama recursivamente con `n-1` y `n-2` y suma los resultados. El contador se comparte con `main` mediante `&mut u64` y se modifica con `*llamadas += 1`. Para `n = 10` el resultado es **55** y el contador termina en **177**.
