# C5 — `raiz_entera` con búsqueda binaria (mi versión)

## Enunciado del taller

> `pub fn raiz_entera(n: u64) -> u64`: la parte entera de √n, con búsqueda binaria y sin usar `f64`.
> Ejemplo: `raiz_entera(26)` es 5. Debe funcionar con n = 10¹⁸ sin desbordar.

---

## Mi código (tal como lo escribí)

```rust
use std::io;

pub fn raiz_entera(n: u64, lim_sup: &mut u64, lim_inf: &mut u64) -> u64 {
    if *lim_sup - *lim_inf > 1 {
        let media: u64 = (*lim_sup + *lim_inf) / 2;
        if (media as u128 * media as u128) > n as u128 {
            *lim_sup = media;
            raiz_entera(n, lim_sup, lim_inf)
        } else {
            if media.strict_pow(2) == n {
                media
            } else {
                *lim_inf = media;
                raiz_entera(n, lim_sup, lim_inf)
            }
        }
    } else {
        if *lim_sup - *lim_inf == 1 {
            *lim_sup + 1
        } else {
            *lim_inf
        }
    }
}

fn main() {
    println!("Ingrese el número: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut input: u64 = a.trim().parse().unwrap();
    println!("Raíz binaria: {}", raiz_entera(input, &mut input, &mut 0));
}
```

---

## Lógica paso a paso

1. **Condición del bucle (`if *lim_sup - *lim_inf > 1`)**
   Se sigue partiendo el rango mientras haya más de un número entero entre `lim_inf` y `lim_sup`. El `*` desreferencia las referencias `&mut u64` para operar con los valores.

2. **Punto medio (`let media: u64 = (*lim_sup + *lim_inf) / 2;`)**
   Se calcula la mitad del rango actual, que es el candidato a raíz.

3. **Comparación segura con u128**
   ```rust
   if (media as u128 * media as u128) > n as u128 {
   ```
   `media * media` podría desbordar `u64` (por ejemplo con n = 10¹⁸). Por eso ambos se convierten a `u128` **antes** de multiplicar: `u128` aguanta hasta ~3.4×10³⁸.

4. **Si `media² > n` → el candidato es muy grande**
   ```rust
   *lim_sup = media;
   ```
   Se baja el límite superior al punto medio y se repite la búsqueda en la mitad izquierda (recursión).

5. **Si `media² == n` → raíz exacta**
   ```rust
   if media.strict_pow(2) == n { media }
   ```
   Se devuelve `media`. Ojo: aquí ya se sabe que `media² <= n`, así que `strict_pow(2)` no desborda.

6. **Si `media² < n` → el candidato es muy chico**
   ```rust
   *lim_inf = media;
   ```
   Se sube el límite inferior y se busca en la mitad derecha (recursión).

7. **Caso base (`*lim_sup - *lim_inf <= 1`)**
   ```rust
   if *lim_sup - *lim_inf == 1 {
       *lim_sup + 1
   } else {
       *lim_inf
   }
   ```
   Cuando el rango se cierra, se devuelve un valor según esta regla.

---

## Detalles de sintaxis que usé

- **`&mut u64` + `*`**: las referencias no se operan directo; `*lim_sup` accede al valor apuntado.
- **Recursión con referencias**: se pasan `lim_sup` y `lim_inf` **tal cual** (ya son `&mut u64`), sin volver a poner `&`.
- **`media as u128`**: conversión explícita antes de multiplicar, para no desbordar `u64`.
- **`media.strict_pow(2)`**: eleva al cuadrado. Solo se llama en la rama donde `media² <= n`, así que no hay riesgo de pánico por desbordamiento.
- **`&mut 0`**: referencia mutable a un temporal; vive lo que dura la llamada. Sirve como límite inferior inicial (0).
- **`main` reutilizando `input`**: se pasa `input` por valor como `n` (es `Copy`, se copia) y `&mut input` como `lim_sup`; y `&mut 0` como `lim_inf`.

---

## Traza de ejemplo (n = 26, con el código de arriba)

| paso | lim_inf | lim_sup | media | media² | ¿media² > 26? | acción |
|---|---|---|---|---|---|---|
| 1 | 0 | 26 | 13 | 169 | sí | `sup = 13` |
| 2 | 0 | 13 | 6 | 36 | sí | `sup = 6` |
| 3 | 0 | 6 | 3 | 9 | no | `9 == 26`? no → `inf = 3` |
| 4 | 3 | 6 | 4 | 16 | no | `16 == 26`? no → `inf = 4` |
| 5 | 4 | 6 | 5 | 25 | no | `25 == 26`? no → `inf = 5` |
| 6 | 5 | 6 | — | — | — | `sup - inf == 1` → devuelve `sup + 1 = 7` |

**Resultado con esta versión: 7.** El enunciado espera 5, así que queda como pendiente revisar el caso base.

---

## Observaciones / pendientes

- **Firma**: el taller pide exactamente `pub fn raiz_entera(n: u64) -> u64`. Esta versión usa tres parámetros (`n`, `lim_sup`, `lim_inf`), así que sirve como función auxiliar, pero para la entrega faltaría una envoltura con la firma exacta.
- **Caso base**: devuelve `*lim_sup + 1` cuando la diferencia es 1; en la traza de `n = 26` eso da 7 en lugar de 5. Queda anotado para revisar.
- **Casos chicos**: `n = 1` devuelve 2 con esta versión (se espera 1); los cuadrados perfectos (`4 → 2`, `9 → 3`, `10¹⁸ → 10⁹`) sí salen bien porque toman la rama de igualdad.
- **`main`**: la etiqueta dice "Raíz binaria"; para la entrega conviene que diga "Raíz entera".

---

## Resumen

La función parte el rango `[lim_inf, lim_sup]` por la mitad en cada llamada, compara `media²` con `n` usando `u128` para evitar desbordamientos y ajusta el límite correspondiente. Si encuentra un cuadrado exacto, devuelve `media`. El código del .md queda **tal cual lo escribí**, como registro de mi propio proceso (con sus detalles pendientes).
