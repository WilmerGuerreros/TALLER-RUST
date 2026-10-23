
fn criba(n: usize) -> Vec<bool> {
    let mut primos = vec![true; n + 1];

    if n >= 0 {
        primos[0] = false;
    }
    if n >= 1 {
        primos[1] = false;
    }

    for i in 2..=n {
        if primos[i] {
            let mut j = i * 2;

            while j <= n {
                primos[j] = false;
                j += i;
            }
        }
    }

    primos
}

fn main() {
    let primos = criba(20);

    println!("Números primos del 0 al 20:");

    for i in 0..=20 {
        if primos[i] {
            println!("{}", i);
        }
    }
}
