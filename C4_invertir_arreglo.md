# C4 — `invertir_arreglo`: lógica paso a paso

## Enunciado

```
pub fn invertir_arreglo(v: &mut [i32])
```

Invierte `v` **en el mismo lugar**, intercambiando elementos, **sin crear otro arreglo** y **sin usar `reverse()`**.

Restricciones:
- No usar `reverse()` ni métodos que resuelvan el problema directamente.
- No crear otro arreglo ni copiar todos los elementos a una estructura auxiliar.
- Usar solamente una variable temporal por cada intercambio.
- Modificar el arreglo original directamente.
- Debe funcionar con arreglos vacíos, de un elemento, de longitud par y de longitud impar.
- Mantener exactamente la firma pedida.

---

## Código de la función

```rust
pub fn invertir_arreglo(v: &mut [i32]) {
    let n = v.len();
    for i in 0..n / 2 {
        let j = n - (i + 1);
        let temp = v[i];
        v[i] = v[j];
        v[j] = temp;
    }
}
```

---

## Lógica paso a paso

1. **`let n = v.len();`**
   Se obtiene la longitud del arreglo `n`. Esta es la cantidad de elementos.

2. **`for i in 0..n / 2`**
   Se recorre la primera mitad de los índices: `0, 1, 2, ...` hasta `n/2` **sin incluir** `n/2`.
   Solo hace falta recorrer la mitad porque en cada vuelta se acomodan **dos** elementos (uno de la izquierda y su opuesto de la derecha).

3. **`let j = n - (i + 1);`**
   Se calcula el índice opuesto. Si `i` apunta a la izquierda, `j` apunta a la posición espejo desde la derecha.
   Ejemplo con `n = 5`:
   - `i = 0` → `j = 4`
   - `i = 1` → `j = 3`

4. **Intercambio con variable temporal:**
   ```rust
   let temp = v[i];   // 1) guarda el valor de la izquierda
   v[i] = v[j];       // 2) mueve el de la derecha a la izquierda
   v[j] = temp;       // 3) coloca el valor guardado a la derecha
   ```
   Se usa **una sola** variable auxiliar (`temp`) por cada intercambio, tal como pide el enunciado.

---

## Por qué funciona en cada caso

| Longitud `n` | `n / 2` | Qué ocurre |
|---|---|---|
| `0` (vacío) | `0` | El rango `0..0` está vacío; no se ejecuta nada. |
| `1` (un elemento) | `0` | El rango `0..0` está vacío; el único elemento queda igual. |
| `4` (par) | `2` | `i = 0,1` → se intercambian `(0,3)` y `(1,2)`. |
| `5` (impar) | `2` | `i = 0,1` → se intercambian `(0,4)` y `(1,3)`; el del centro (índice 2) no se toca. |

En los casos vacío y de un elemento, el rango queda vacío y no se requiere código especial.

---

## Detalles de sintaxis de Rust importantes

- **`&mut [i32]`**: es un *slice* mutable, una vista a datos que ya existen. Por eso lo que se modifica se refleja en el arreglo original.
- **`v.len()`** devuelve `usize` (entero sin signo). Por eso `n`, `i` y `j` son `usize`.
- **`n - (i + 1)`** nunca es negativo: `i` va de `0` a `n/2 - 1`, así que `i + 1 <= n/2 <= n`.
- **`0..n / 2`** es un rango "inicio incluido, fin excluido".
- **`let temp = v[i];`**: `v[i]` devuelve una **copia** del `i32` (los enteros son tipo `Copy`), así que el acceso no "bloquea" el arreglo.
- **Verificación de índices**: Rust comprueba los índices en tiempo de ejecución; aquí siempre son válidos, así que no hay pánico.
- **Sin `reverse()` ni arreglo auxiliar**: solo se lee y se escribe dentro del mismo `v`.

---

## Compilación / prueba manual (`main`)

```rust
use std::io;

pub fn invertir_arreglo(v: &mut [i32]) {
    let n = v.len();
    for i in 0..n / 2 {
        let j = n - (i + 1);
        let temp = v[i];
        v[i] = v[j];
        v[j] = temp;
    }
}

fn main() {
    let mut qqq: Vec<i32> = Vec::new();

    println!("Ingrese el tamaño del arreglo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let n: usize = linea.trim().parse().unwrap();

    for i in 0..n {
        println!("Ingrese el {} número:", i);
        let mut linea = String::new();
        io::stdin().read_line(&mut linea).unwrap();
        let aux: i32 = linea.trim().parse().unwrap();
        qqq.push(aux);
    }

    println!("Arreglo: {:?}", qqq);
    invertir_arreglo(&mut qqq);   // &mut Vec<i32> se coacciona a &mut [i32]
    println!("Invertido: {:?}", qqq);
}
```

### Por qué `&mut qqq` es válido
`qqq` es `Vec<i32>`, y `invertir_arreglo` espera `&mut [i32]`. Rust coacciona automáticamente `&mut Vec<i32>` a `&mut [i32]`, así que no hay que hacer nada especial.

### Por qué se necesita `.trim().parse().unwrap()`
- `read_line` **siempre** entrega un `String` (texto), aunque el usuario escriba `42`.
- `.trim()` quita el salto de línea `\n` que deja `read_line`.
- `.parse()` convierte texto → número (el tipo lo indica el `: usize` o `: i32`).
- `.parse()` devuelve un `Result`, no el número directamente.
- `.unwrap()` "abre" ese `Result`: si es `Ok` saca el valor, si es `Err` el programa se cae con pánico.

---

## Casos de prueba (con `#[cfg(test)]`, no tocan el resto del proyecto)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vacio() {
        let mut v: [i32; 0] = [];
        invertir_arreglo(&mut v);
        assert_eq!(v, []);
    }

    #[test]
    fn un_elemento() {
        let mut v = [7];
        invertir_arreglo(&mut v);
        assert_eq!(v, [7]);
    }

    #[test]
    fn longitud_par() {
        let mut v = [1, 2, 3, 4];
        invertir_arreglo(&mut v);
        assert_eq!(v, [4, 3, 2, 1]);
    }

    #[test]
    fn longitud_impar() {
        let mut v = [1, 2, 3, 4, 5];
        invertir_arreglo(&mut v);
        assert_eq!(v, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn con_repetidos_y_negativos() {
        let mut v = [-3, 0, 5, 5, 9];
        invertir_arreglo(&mut v);
        assert_eq!(v, [9, 5, 5, 0, -3]);
    }

    #[test]
    fn dos_veces_devuelve_el_original() {
        let mut v = [1, 2, 3, 4, 5, 6];
        let original = v;
        invertir_arreglo(&mut v);
        invertir_arreglo(&mut v);
        assert_eq!(v, original);
    }
}
```

- Se pasan arreglos de tamaño fijo (`[i32; 4]`) que se coaccionan a `&mut [i32]`.
- `assert_eq!` compara el arreglo completo; si no coincide, el test falla.
- El caso "dos veces" verifica la propiedad de involución: invertir dos veces vuelve al original.

---

## Resumen

La función recorre solo la **primera mitad** del arreglo y, en cada paso, intercambia el elemento de la izquierda (`v[i]`) con su opuesto (`v[j]`), usando una única variable temporal. Así invierte todo el arreglo en el mismo lugar, sin `reverse()` y sin memoria extra.
