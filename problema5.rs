fn euclides_ext(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        if a >= 0 {
            return (a, 1, 0);
        } else {
            return (-a, -1, 0);
        }
    }

    let (g, x1, y1) = euclides_ext(b, a % b);

    (g, y1, x1 - (a / b) * y1)
}

fn inverso(a: i64, m: i64) -> Option<i64> {
    if m <= 0 {
        return None;
    }

    let a = a.rem_euclid(m);
    let (g, x, _) = euclides_ext(a, m);

    if g != 1 {
        return None;
    }

    Some(x.rem_euclid(m))
}

fn main() {
    println!("Inverso de 3 mod 26: {:?}", inverso(3, 26));
    println!("Inverso de 4 mod 26: {:?}", inverso(4, 26));

    let (g, x, y) = euclides_ext(99, 78);
    println!("MCD: {}", g);
    println!("x: {}, y: {}", x, y);
    println!("Comprobación: {}", 99 * x + 78 * y);
}