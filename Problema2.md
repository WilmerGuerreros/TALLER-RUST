# Problema 2 — Variante 2: `contar_largas` (Collatz con memoización)

## Enunciado

La **secuencia de Collatz** de un número `n` se construye así:

- Si `n` es par: `n / 2`.
- Si `n` es impar: `3n + 1`.
- Se repite hasta llegar a `1`.

La **longitud** cuenta todos los términos, incluidos `n` y el `1`. Por ejemplo, la secuencia de `6` es:

`6, 3, 10, 5, 16, 8, 4, 2, 1` → longitud **9**.

La variante V2 pide:

```rust
pub fn contar_largas(limite: u64, k: u32) -> u64
```

Devuelve **cuántos números `n ≤ limite`** tienen una longitud de Collatz **mayor que `k`**.

Ejemplo: `contar_largas(10, 10)` → **2** (los números 7 y 9).

Tamaño máximo: `limite = 10⁶`. A ese tamaño la fuerza bruta no entra en el tiempo, así que hay que **memoizar** las longitudes ya calculadas en un `Vec`.

---

## Código

```rust
use std::io;

pub fn contar_largas(limite: u64, k: u32) -> u64 {
    if limite == 0 {
        return 0;
    }

    // memo[n] = longitud de Collatz de n; 0 = todavía no calculado
    let mut memo = vec![0; (limite + 1) as usize];
    memo[1] = 1;

    let mut contador: u64 = 0;
    for n in 1..=limite {
        if longitud(n, &mut memo) > k {
            contador += 1;
        }
    }
    contador
}

fn longitud(n: u64, memo: &mut [u32]) -> u32 {
    // Si ya está calculado, lo devolvemos
    if n < memo.len() as u64 && memo[n as usize] != 0 {
        return memo[n as usize];
    }

    // Siguiente término de Collatz
    let siguiente = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

    // Longitud = 1 (este término) + la del siguiente
    let l = 1 + longitud(siguiente, memo);

    // Guardamos solo si cabe en el Vec
    if n < memo.len() as u64 {
        memo[n as usize] = l;
    }

    l
}

fn main() {
    println!("Ingrese el número k:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let k: u32 = a.trim().parse().unwrap();

    println!("Ingrese el límite:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let limite: u64 = b.trim().parse().unwrap();

    println!("Cantidad: {}", contar_largas(limite, k));
}
```

---

## Idea general

El `Vec` `memo` guarda la longitud de cada número ya calculado:

- `memo[n] = 0` → todavía no se calculó.
- `memo[n] > 0` → ya conocemos su longitud.

La regla recursiva es:

```text
longitud(1) = 1
longitud(n) = 1 + longitud(siguiente(n))
```

El `1 +` es importante: cuenta el término actual antes de pasar al siguiente.

---

## Lógica paso a paso

1. **Caso borde `limite == 0`**
   ```rust
   if limite == 0 {
       return 0;
   }
   ```
   No hay números que contar (y además `memo[1]` no existiría si el vector midiera 1).

2. **Crear el memo**
   ```rust
   let mut memo = vec![0; (limite + 1) as usize];
   memo[1] = 1;
   ```
   - Se necesitan índices de `0` a `limite`, por eso el tamaño es `limite + 1`.
   - `memo[1] = 1` porque la longitud de `1` es 1 (solo el término `1`).

3. **Recorrer todos los números del rango**
   ```rust
   for n in 1..=limite {
   ```
   El rango `1..=limite` es **inclusivo**: incluye `limite`, tal como pide el enunciado (`n ≤ limite`).

4. **Calcular la longitud con el helper**
   ```rust
   if longitud(n, &mut memo) > k {
       contador += 1;
   }
   ```
   Si la longitud supera `k`, se incrementa el contador.

5. **Dentro de `longitud`**
   - Si `n` ya está memoizado, se devuelve directamente:
     ```rust
     if n < memo.len() as u64 && memo[n as usize] != 0 {
         return memo[n as usize];
     }
     ```
   - Si no, se calcula el siguiente término:
     ```rust
     let siguiente = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
     ```
   - Se suma 1 y se llama recursivamente:
     ```rust
     let l = 1 + longitud(siguiente, memo);
     ```
   - Se guarda el resultado **solo si `n` cabe en el `Vec`**:
     ```rust
     if n < memo.len() as u64 {
         memo[n as usize] = l;
     }
     ```
   - Se devuelve `l`.

6. **Devolver el contador**
   ```rust
   contador
   ```
   La función pública retorna cuántos números cumplieron la condición.

---

## Ejemplo: `contar_largas(10, 10)`

Longitudes de los números del 1 al 10:

| n | Longitud | ¿Mayor que 10? |
|---|---|---|
| 1 | 1 | no |
| 2 | 2 | no |
| 3 | 8 | no |
| 4 | 3 | no |
| 5 | 6 | no |
| 6 | 9 | no |
| 7 | 17 | **sí** |
| 8 | 4 | no |
| 9 | 20 | **sí** |
| 10 | 7 | no |

Resultado: **2**.

La secuencia de 7 completa:

`7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1` → 17 términos.

---

## Por qué se comprueba `n < memo.len()`

La secuencia puede pasar por números **mayores que `limite`**. Por ejemplo, con `limite = 10`:

- `7 → 22`, y `22` no cabe en un `Vec` de tamaño 11 (índices `0..10`).

Si hiciéramos `memo[22]` directamente, el programa entraría en pánico por índice fuera de rango. Por eso:

- Solo se **lee** `memo[n]` si `n < memo.len()`.
- Solo se **guarda** `memo[n]` si `n < memo.len()`.
- Los valores que superan `limite` se calculan igual, pero no se memorizan.

---

## Detalles de sintaxis de Rust importantes

- **`memo: &mut [u32]`**: es un *slice* mutable. Permite leer y escribir el `Vec` sin transferir su propiedad.
- **`&mut memo`**: al pasar un `Vec<u32>` a una función que espera `&mut [u32]`, Rust hace la coerción automáticamente.
- **`(limite + 1) as usize`**: el tamaño de un `Vec` se expresa en `usize`, no en `u64`.
- **`memo[n as usize]`**: los índices también son `usize`; por eso el cast.
- **`n < memo.len() as u64`**: se comparan dos `u64`; `memo.len()` devuelve `usize`.
- **`vec![0; N]`**: crea un vector de `N` ceros. El tipo se infiere como `u32` por el uso en `longitud`.
- **`0` como centinela**: las longitudes reales son ≥ 1, así que `0` sirve para indicar “no calculado”.
- **`1 + longitud(...)`**: cuenta el término actual; sin ese `+1`, todas las longitudes valdrían 1.
- **`for n in 1..=limite`**: rango inclusivo; `1..limite` dejaría fuera al límite.
- **`if/else` como expresión**: `let siguiente = if ... { ... } else { ... };` produce un valor.
- **Recursión**: la profundidad máxima para `limite = 10⁶` es 525, así que no hay problema de pila.

---

## Complejidad

- **Memoria:** `O(limite)` por el `Vec` de `limite + 1` elementos `u32`.
- **Tiempo:** cada número `n ≤ limite` se calcula una sola vez gracias al memo. Los valores que superan `limite` no se memorizan, pero el recorrido total para `limite = 10⁶` es de unos pocos millones de pasos, muy por debajo de la fuerza bruta.
- Para `limite = 10⁶`, la longitud máxima es **525**, que cabe perfectamente en `u32`.

---

## Pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_del_enunciado() {
        assert_eq!(contar_largas(10, 10), 2);
    }

    #[test]
    fn limite_cero() {
        assert_eq!(contar_largas(0, 10), 0);
    }

    #[test]
    fn un_solo_numero() {
        assert_eq!(contar_largas(1, 0), 1);
        assert_eq!(contar_largas(1, 1), 0);
    }

    #[test]
    fn longitudes_conocidas() {
        assert_eq!(contar_largas(10, 8), 3);  // 6 (9), 7 (17), 9 (20)
        assert_eq!(contar_largas(10, 16), 2); // 7 (17), 9 (20)
        assert_eq!(contar_largas(10, 17), 1); // solo 9 (20)
    }

    #[test]
    fn caso_grande() {
        assert_eq!(contar_largas(1_000_000, 100), 655_970);
    }
}
```

Notas sobre las pruebas:

- `contar_largas(10, 8)` da **3** porque las longitudes mayores que 8 son las de 6 (9), 7 (17) y 9 (20).
- `contar_largas(10, 16)` da **2** porque solo 7 y 9 superan 16.
- `contar_largas(10, 17)` da **1** porque solo 9 (20) supera 17.
- El caso de `10⁶` verifica que no se sale del `Vec` y que el resultado es el esperado; conviene ejecutarlo con `cargo test --release`.

---

## Nota para la entrega

- La función que se entrega va en `src/lib.rs` y debe ser exactamente:
  ```rust
  pub fn contar_largas(limite: u64, k: u32) -> u64
  ```
- El helper `longitud` puede quedar **privado** (sin `pub`) en el mismo archivo.
- `main` es solo para probar por consola; no es necesario para las pruebas del taller.

---

## Resumen

`contar_largas` crea un `Vec` donde guarda la longitud de Collatz de cada número ya calculado. Para cada `n` del rango `1..=limite`, llama a `longitud`, que usa la regla `longitud(n) = 1 + longitud(siguiente(n))` y memoriza el resultado si `n` cabe en el vector. Si la longitud supera `k`, incrementa el contador. Así se evita recalcular secuencias y se cumple el límite de tiempo para `10⁶`.
