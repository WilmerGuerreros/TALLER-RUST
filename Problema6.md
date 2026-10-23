# Problema 6 — Variante 2: `a_lo_sumo_dos`

## Enunciado

El Problema 6 tiene dos partes.

### 6.a) El programa que no compila

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        if *x % 2 == 1 {
            v.push(*x * 10);
        }
    }
    println!("{:?}", v);
}
```

Hay que explicar qué regla de préstamos se viola y corregirlo **sin usar `.clone()`**.

### 6.b) La función de la variante 2

```rust
pub fn a_lo_sumo_dos(v: &mut Vec<i32>) -> usize
```

- `v` está **ordenado**.
- Deja cada valor **como máximo dos veces**.
- Conserva el orden.
- **Recorta `v`**.
- Devuelve su nuevo largo.

Ejemplo:

```text
[1, 1, 1, 2, 2, 3]  →  [1, 1, 2, 2, 3]  y devuelve 5
```

Restricciones: **O(n)** en tiempo, **O(1)** de memoria adicional, sin crear otro vector y sin `.clone()`.

---

## Código de `a_lo_sumo_dos`

```rust
pub fn a_lo_sumo_dos(v: &mut Vec<i32>) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut escribir = 1;
    let mut actual = v[0];
    let mut contador = 1;

    for leer in 1..v.len() {
        if v[leer] == actual {
            contador += 1;
        } else {
            actual = v[leer];
            contador = 1;
        }

        if contador <= 2 {
            v[escribir] = v[leer];
            escribir += 1;
        }
    }

    v.truncate(escribir);
    escribir
}
```

Y el `main` de prueba:

```rust
use std::io;

fn main() {
    let mut vector: Vec<i32> = Vec::new();

    let mut a: String = String::new();
    println!("Inserte la longitud del vector: ");
    io::stdin().read_line(&mut a).unwrap();
    let longitud: i32 = a.trim().parse().unwrap();

    for i in 1..longitud + 1 {
        println!("Ingrese el {}° valor del vector", i);
        let mut b: String = String::new();
        io::stdin().read_line(&mut b).unwrap();

        let input: i32 = b.trim().parse().unwrap();
        vector.push(input);
    }

    println!(
        "La nueva longitud del vector es {}",
        a_lo_sumo_dos(&mut vector)
    );
}
```

---

## Idea general: compactar y luego recortar

No se puede eliminar con `remove` porque cada `remove` mueve todos los elementos de la derecha y el costo total sería **O(n²)**.

La solución es usar **dos índices**:

- `leer`: recorre todo el vector.
- `escribir`: marca dónde se guarda el siguiente elemento permitido.

Se van copiando los elementos válidos hacia el inicio del mismo vector. Al terminar:

- `v[0..escribir]` contiene el resultado.
- `v[escribir..]` contiene basura/duplicados que ya no deben formar parte del vector.
- `v.truncate(escribir)` elimina esa cola.

---

## Lógica paso a paso

1. **Vector vacío**
   ```rust
   if v.is_empty() {
       return 0;
   }
   ```
   Sin esto, `v[0]` entraría en pánico.

2. **Inicializar variables**
   ```rust
   let mut escribir = 1;
   let mut actual = v[0];
   let mut contador = 1;
   ```
   - El primer elemento siempre se conserva, por eso `escribir = 1`.
   - `actual` es el valor del grupo que se está leyendo.
   - `contador` indica cuántas veces ha aparecido `actual`.

3. **Recorrer desde el segundo elemento**
   ```rust
   for leer in 1..v.len() {
   ```
   `leer` empieza en 1 porque `v[0]` ya está conservado.

4. **Actualizar el grupo**
   ```rust
   if v[leer] == actual {
       contador += 1;
   } else {
       actual = v[leer];
       contador = 1;
   }
   ```
   - Si el valor se repite, aumenta el contador.
   - Si cambia, empieza un grupo nuevo con contador 1.

5. **Copiar solo si va como máximo dos veces**
   ```rust
   if contador <= 2 {
       v[escribir] = v[leer];
       escribir += 1;
   }
   ```
   - La primera y la segunda aparición se copian.
   - La tercera en adelante se saltan.

6. **Recortar el vector**
   ```rust
   v.truncate(escribir);
   ```
   Elimina la cola que ya no pertenece al resultado.

7. **Devolver el nuevo largo**
   ```rust
   escribir
   ```
   Después de `truncate`, coincide con `v.len()`.

---

## El proceso de `truncate` en detalle

Esta es la parte que más conviene entender.

### ¿Qué hace `v.truncate(n)`?

`truncate(n)` reduce la longitud del `Vec` a **como máximo `n`**:

- Si `n` es menor que la longitud actual, elimina los elementos **del final** hasta que queden `n`.
- Si `n` es mayor o igual que la longitud actual, **no hace nada**.

Ejemplo:

```rust
let mut v = vec![1, 1, 2, 2, 3, 3];
v.truncate(5);
// v queda como [1, 1, 2, 2, 3]
```

El último `3` desaparece de la parte válida del vector.

### ¿Qué pasa con la memoria?

`truncate` **no crea un vector nuevo** y **no mueve los elementos que se quedan**. Solamente cambia la longitud lógica del `Vec`.

Antes:

```text
v = [1, 1, 2, 2, 3, 3]
len = 6
capacidad = 6 (por ejemplo)
```

Después de `v.truncate(5)`:

```text
v = [1, 1, 2, 2, 3]
len = 5
capacidad = 6 (puede seguir igual)
```

La capacidad **no necesariamente baja**. Lo que cambia es `len`: los elementos que están después de `len` ya no forman parte del vector y no se pueden acceder con `v[i]` de forma segura.

Si se quisiera liberar la memoria sobrante, habría que usar `shrink_to_fit`, pero **no es necesario** para este problema. De hecho, `shrink_to_fit` podría reasignar memoria, y aquí solo queremos recortar.

### ¿Por qué es barato para `i32`?

`i32` es un tipo `Copy` y no tiene destructor. Por eso, al hacer `truncate` con `i32`, no hay que ejecutar código de limpieza por cada elemento eliminado: en la práctica solo se ajusta la longitud del `Vec`.

En cambio, si el vector fuera de `String` o de otro tipo con destructor, `truncate` sí tendría que liberar cada elemento eliminado; el costo sería proporcional a la cantidad de elementos recortados. Aquí no ocurre eso.

### ¿Por qué se hace al final y no durante el recorrido?

Porque primero hay que **compactar** los elementos válidos al inicio:

```text
antes:   [1, 1, 1, 2, 2, 3]
                 ^
                 el tercer 1 no se copia

después de compactar:
         [1, 1, 2, 2, 3, 3]
          ^        ^
          válidos  cola sobrante

después de truncate(5):
         [1, 1, 2, 2, 3]
```

Si se truncara antes de compactar, se perderían elementos válidos que todavía no se habían copiado.

### Diferencia con otras operaciones

| Operación | Qué hace | ¿Crea otro vector? | Costo |
|---|---|---|---|
| `v.remove(i)` | Elimina el elemento `i` y mueve todo lo de la derecha | No | O(n) |
| `v.truncate(n)` | Corta la cola hasta dejar longitud `n` | No | O(1) para `i32` |
| `v.drain(..)` | Extrae un rango y desplaza el resto | No | O(n) |
| `v.split_off(n)` | Devuelve un vector nuevo con la cola | **Sí** | O(n) |

Por eso `truncate` es la opción adecuada después de compactar: no crea otro vector, no desplaza elementos y deja el `Vec` con la longitud correcta.

---

## Traza con `[1, 1, 1, 2, 2, 3]`

| `leer` | valor | `contador` | ¿se copia? | `escribir` | vector |
|---|---|---|---|---|---|
| 1 | 1 | 2 | sí | 2 | `[1, 1, 1, 2, 2, 3]` |
| 2 | 1 | 3 | no | 2 | `[1, 1, 1, 2, 2, 3]` |
| 3 | 2 | 1 | sí | 3 | `[1, 1, 2, 2, 2, 3]` |
| 4 | 2 | 2 | sí | 4 | `[1, 1, 2, 2, 2, 3]` |
| 5 | 3 | 1 | sí | 5 | `[1, 1, 2, 2, 3, 3]` |

Al final:

```rust
v.truncate(5);
// v = [1, 1, 2, 2, 3]
// devuelve 5
```

---

## ¿Por qué no se pisa información sin leer?

En cada paso se cumple que `escribir <= leer`:

- `escribir` empieza en 1 y `leer` también.
- `escribir` avanza como máximo una vez por cada avance de `leer`.

Por eso, cuando se hace:

```rust
v[escribir] = v[leer];
```

nunca se sobrescribe un elemento que todavía no se haya leído. Si `escribir == leer`, la asignación copia el valor sobre sí mismo, lo cual no causa problemas.

---

## Complejidad

- **Tiempo: O(n)**. Se recorre el vector una sola vez; cada elemento se procesa con operaciones constantes. `truncate` sobre `i32` no recorre la cola con destructores.
- **Memoria adicional: O(1)**. Solo se usan `escribir`, `actual`, `contador` y `leer`; no se crea otro vector.
- **Modifica el vector original**: los elementos válidos se copian al inicio y luego se recorta.

---

## 6.a) Por qué el programa original no compila

El programa:

```rust
let mut v = vec![1, 2, 3];
for x in &v {
    if *x % 2 == 1 {
        v.push(*x * 10);
    }
}
```

falla porque:

1. **`for x in &v` pide un préstamo inmutable de `v`** y lo mantiene durante todo el bucle.
2. **`v.push(...)` necesita un préstamo mutable de `v`**.
3. Rust no permite tener un préstamo inmutable y uno mutable al mismo tiempo sobre el mismo valor. El error es del tipo `E0502`: *cannot borrow `v` as mutable because it is also borrowed as immutable*.

Además, `push` puede **reasignar** el vector si se queda sin capacidad. Si eso ocurre, las referencias o el iterador que apuntaban a la memoria anterior quedarían inválidos. Rust prohíbe esa situación para evitar accesos a memoria liberada.

### Corrección sin `.clone()`

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let n = v.len();

    for i in 0..n {
        let x = v[i];
        if x % 2 == 1 {
            v.push(x * 10);
        }
    }

    println!("{:?}", v);
}
```

Por qué funciona:

- `v[i]` devuelve una **copia** del `i32` y el préstamo termina en esa línea.
- Luego `v.push(...)` puede pedir el préstamo mutable sin conflicto.
- `n` se guarda antes del bucle, así que el rango no cambia aunque el vector crezca.

`.clone()` también compilaría, pero copiaría todo el vector; el enunciado pide evitarlo.

---

## Pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_del_enunciado() {
        let mut v = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(a_lo_sumo_dos(&mut v), 5);
        assert_eq!(v, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn todos_repetidos() {
        let mut v = vec![1, 1, 1, 1, 1];
        assert_eq!(a_lo_sumo_dos(&mut v), 2);
        assert_eq!(v, vec![1, 1]);
    }

    #[test]
    fn vacio() {
        let mut v: Vec<i32> = vec![];
        assert_eq!(a_lo_sumo_dos(&mut v), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn sin_duplicados() {
        let mut v = vec![1, 2, 3];
        assert_eq!(a_lo_sumo_dos(&mut v), 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn un_solo_elemento() {
        let mut v = vec![7];
        assert_eq!(a_lo_sumo_dos(&mut v), 1);
        assert_eq!(v, vec![7]);
    }

    #[test]
    fn varios_grupos() {
        let mut v = vec![1, 1, 1, 2, 2, 2, 2, 3];
        assert_eq!(a_lo_sumo_dos(&mut v), 5);
        assert_eq!(v, vec![1, 1, 2, 2, 3]);
    }
}
```

---

## Nota para la entrega

- La función va en `src/lib.rs` con la firma exacta:
  ```rust
  pub fn a_lo_sumo_dos(v: &mut Vec<i32>) -> usize
  ```
- `main` es solo para probar por consola.
- No se usa `.clone()`, no se crea otro vector y no se usa `remove`.
- `truncate` no crea un vector nuevo: solo recorta la longitud lógica del original.

---

## Resumen

`a_lo_sumo_dos` recorre el vector con un índice de lectura y otro de escritura. Conserva la primera y la segunda aparición de cada valor, copiándolas al inicio del mismo `Vec`. Al terminar, `v.truncate(escribir)` elimina la cola sobrante y `escribir` es el nuevo largo. Todo en **O(n)** tiempo y **O(1)** memoria adicional.
