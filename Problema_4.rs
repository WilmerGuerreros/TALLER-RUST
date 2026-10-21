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
