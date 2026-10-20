# C4 — `Rectangulo`: lógica paso a paso

## Enunciado

```rust
pub struct Rectangulo { pub ancho: u32, pub alto: u32 }
```

Con los métodos:

- `pub fn area(&self) -> u32`
- `pub fn escalar(&mut self, k: u32)`, que multiplica ambos lados por `k`.

Ejemplo: un rectángulo de 3 × 4 tiene área 12; después de `escalar(2)`, 48.

---

## Código completo

```rust
use std::io;

pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

impl Rectangulo {
    pub fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    pub fn escalar(&mut self, k: u32) {
        self.ancho *= k;
        self.alto *= k;
    }
}

fn main() {
    println!("Ingrese el ancho del rectángulo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let ancho: u32 = linea.trim().parse().unwrap();

    println!("Ingrese la altura del rectángulo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let alto: u32 = linea.trim().parse().unwrap();

    let mut r = Rectangulo { ancho, alto };
    println!("Área original: {}", r.area());

    println!("Ingrese un número k a escalar, o ponga 1 si quiere cambios:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let k: u32 = linea.trim().parse().unwrap();

    r.escalar(k);
    println!("Nueva área del rectángulo escalado: {}", r.area());
}
```

---

## Lógica paso a paso

1. **Definir la estructura `Rectangulo`**
   ```rust
   pub struct Rectangulo {
       pub ancho: u32,
       pub alto: u32,
   }
   ```
   Una `struct` agrupa datos relacionados. Aquí tiene dos **campos** (`ancho`, `alto`) de tipo `u32` (entero sin signo de 32 bits). `pub` significa que son visibles desde fuera del módulo.

2. **Implementar los métodos con `impl`**
   ```rust
   impl Rectangulo { ... }
   ```
   El bloque `impl` asocia funciones a la estructura. Estas funciones se llaman **métodos** y se invocan con la notación de punto: `r.area()`, `r.escalar(k)`.

3. **`area`: solo lee → `&self`**
   ```rust
   pub fn area(&self) -> u32 {
       self.ancho * self.alto
   }
   ```
   - `&self` es una **referencia inmutable** al propio rectángulo: permite **leer** los campos, no modificarlos.
   - `self.ancho` y `self.alto` acceden a los campos desde dentro del método.
   - `-> u32` indica que devuelve un número. Al no llevar `;` al final, el valor del producto **es** lo que retorna la función.

4. **`escalar`: modifica → `&mut self`**
   ```rust
   pub fn escalar(&mut self, k: u32) {
       self.ancho *= k;
       self.alto *= k;
   }
   ```
   - `&mut self` es una **referencia mutable**: permite **escribir** en los campos.
   - `self.ancho *= k;` es equivalente a `self.ancho = self.ancho * k;` (operador compuesto, sin repetir el nombre).
   - No devuelve nada (`->` ausente = retorna `()`), porque su efecto es modificar el propio rectángulo.

5. **Leer los datos del usuario en `main`**
   - Cada valor se lee con `io::stdin().read_line(&mut linea)`, que **siempre** entrega un `String` (texto), aunque el usuario escriba `3`.
   - `.trim()` quita el salto de línea `\n` que deja `read_line`.
   - `.parse()` convierte texto → número; el tipo lo indica `: u32`.
   - `.unwrap()` "abre" el `Result` que devuelve `parse` (si es `Ok` saca el valor; si es `Err`, el programa se cae).

6. **Crear el rectángulo**
   ```rust
   let mut r = Rectangulo { ancho, alto };
   ```
   - `ancho, alto` es el **atajo de campos**: como la variable local se llama igual que el campo, equivale a `ancho: ancho, alto: alto`.
   - `let mut r` es obligatorio porque después se llamará a `escalar`, que necesita `&mut self`.

7. **Mostrar el área original**
   ```rust
   println!("Área original: {}", r.area());
   ```
   `r.area()` calcula `ancho * alto`. En el ejemplo 3 × 4 → `12`.

8. **Leer `k`, escalar y mostrar la nueva área**
   ```rust
   r.escalar(k);
   println!("Nueva área del rectángulo escalado: {}", r.area());
   ```
   `escalar(k)` multiplica ambos lados por `k`. Con 3 × 4 y `k = 2`: pasa a 6 × 8 → área `48`.

---

## Salida esperada de un ejemplo

Entrada: ancho = `3`, alto = `4`, k = `2`

```
Ingrese el ancho del rectángulo:
3
Ingrese la altura del rectángulo:
4
Área original: 12
Ingrese un número k a escalar, o ponga 1 si quiere cambios:
2
Nueva área del rectángulo escalado: 48
```

---

## Detalles de sintaxis de Rust importantes

- **`struct`**: agrupa varios valores (campos) bajo un mismo tipo con nombre.
- **Campo público `pub`**: visible fuera del módulo donde se declaró. Sin `pub`, solo el propio módulo podría acceder.
- **`impl`**: bloque donde se definen los métodos de un tipo.
- **`self`**: el propio valor sobre el que se llama el método (`r` al hacer `r.area()`).
- **`&self` vs `&mut self`**:
  - `&self` → préstamo inmutable: **solo lectura** (basta para `area`).
  - `&mut self` → préstamo mutable: **lectura y escritura** (necesario para `escalar`).
- **`*=`**: operador compuesto. `x *= k` equivale a `x = x * k`.
- **`-> u32`**: declara el tipo de retorno. Sin `->`, el método devuelve `()` (nada).
- **Última expresión sin `;`**: el valor de esa expresión es lo que devuelve la función.
- **Atajo `Rectangulo { ancho, alto }`**: cuando la variable local tiene el mismo nombre que el campo.
- **`let mut r`**: el método `escalar` modifica `r`, por lo que la variable debe ser `mut` y el préstamo `&mut`.

---

## Por qué `area` usa `&self` y `escalar` usa `&mut self`

| Método | ¿Qué hace? | Receptor | ¿Obliga a `let mut`? |
|---|---|---|---|
| `area` | Solo **lee** `ancho` y `alto` | `&self` | No |
| `escalar` | **Modifica** `ancho` y `alto` | `&mut self` | Sí |

Esta distinción es justo el punto del C4 ("préstamos y estructuras"): un método que no cambia nada debe pedir el préstamo más débil posible (`&self`); solo el que modifica pide `&mut self`.

---

## `mut` vs `&mut` (aclaración clave)

- **`mut`** (en `let mut x`) → marca que **la variable** puede cambiar de valor.
- **`&mut`** (en parámetros o receptores) → indica que se **presta** el valor para que otro lo modifique.
- Para modificar algo a través de un préstamo se necesitan **ambos**: la variable `mut` **y** el `&mut`.
- Si solo se lee, basta `&` (o `&self`).

```rust
let mut r = Rectangulo { ancho: 3, alto: 4 }; // mut: r va a cambiar
println!("{}", r.area());   // &self por detrás: solo lectura
r.escalar(2);               // &mut self por detrás: escritura, r debe ser mut
```

---

## Por qué hay DOS `unwrap` en cada lectura

`unwrap` no está ligado a `parse`; sirve para "abrir" un `Result`. Y hay **dos** operaciones que pueden fallar, cada una con su propio `Result`:

| Operación | Qué devuelve | ¿Por qué puede fallar? | `unwrap` |
|---|---|---|---|
| `read_line(&mut linea)` | `Result<usize, io::Error>` | falla la entrada (teclado, encoding, etc.) | #1 |
| `linea.trim().parse()` | `Result<u32, ParseIntError>` | el texto podría no ser un número | #2 |

No tienen relación entre sí: el primero es por si **la lectura** falla; el segundo, por si **la conversión** falla.

---

## Casos de prueba (con `#[cfg(test)]`, no tocan el resto del proyecto)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_basica() {
        let r = Rectangulo { ancho: 3, alto: 4 };
        assert_eq!(r.area(), 12);
    }

    #[test]
    fn escalar_por_2() {
        let mut r = Rectangulo { ancho: 3, alto: 4 };
        r.escalar(2);
        assert_eq!(r.ancho, 6);
        assert_eq!(r.alto, 8);
        assert_eq!(r.area(), 48);
    }

    #[test]
    fn escalar_por_1_no_cambia() {
        let mut r = Rectangulo { ancho: 5, alto: 7 };
        r.escalar(1);
        assert_eq!(r.area(), 35);
    }

    #[test]
    fn escalar_por_0() {
        let mut r = Rectangulo { ancho: 4, alto: 9 };
        r.escalar(0);
        assert_eq!(r.area(), 0);
    }

    #[test]
    fn escalar_dos_veces() {
        let mut r = Rectangulo { ancho: 1, alto: 2 };
        r.escalar(2); // 2 x 4
        r.escalar(3); // 6 x 12
        assert_eq!(r.area(), 72);
    }
}
```

- `escalar` recibe `&mut self`, así que en los tests la variable del rectángulo debe ser `let mut r`.
- `escalar` **no** devuelve nada: se llama como sentencia y luego se comprueba con `r.area()` o con los campos.
- El caso `k = 0` lleva el área a 0 (caso borde, dimensiones válidas pero colapsadas).

---

## Resumen

`Rectangulo` guarda `ancho` y `alto`. El método `area` **lee** los campos (por eso `&self`) y devuelve `ancho * alto`. El método `escalar` **modifica** los campos multiplicándolos por `k` (por eso `&mut self`, y por eso la variable debe ser `let mut`). El `main` solo se encarga de leer los datos con `read_line` + `trim().parse().unwrap()` y de mostrar las áreas antes y después de escalar.
