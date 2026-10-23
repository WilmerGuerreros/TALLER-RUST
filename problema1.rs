fn potencia_mod(base: u64, exp: u64, m: u64) -> u64 {
    if m == 0 {
        return 0;
    }

    let mut base = base % m;
    let mut exp = exp;
    let mut resultado = 1 % m;

    while exp > 0 {
        if exp % 2 == 1 {
            resultado =
                ((resultado as u128 * base as u128) % m as u128) as u64;
        }

        base = ((base as u128 * base as u128) % m as u128) as u64;
        exp /= 2;
    }

    resultado
}

fn potencia_mod_grande(base: u64, exp: &str, m: u64) -> u64 {
    if m == 0 || exp.is_empty() {
        return 0;
    }

    let mut resultado = 1 % m;

    for c in exp.chars() {
        if !c.is_ascii_digit() {
            return 0;
        }

        resultado = potencia_mod(resultado, 10, m);
        let digito = c.to_digit(10).unwrap() as u64;
        let parte = potencia_mod(base, digito, m);

        resultado =
            ((resultado as u128 * parte as u128) % m as u128) as u64;
    }

    resultado
}

fn main() {
    println!("2^10 mod 1000 = {}", potencia_mod(2, 10, 1000));
    println!("2^100 mod 1000000007 = {}",potencia_mod_grande(2, "100", 1_000_000_007)
    );
}