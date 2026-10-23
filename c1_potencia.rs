
fn potencia(base: u64, exp: u32) -> Option<u64> {
    let mut resultado = 1u64;

    for _ in 0..exp {
        resultado = resultado.checked_mul(base)?;
    }

    Some(resultado)
}

fn main() {
    println!("{:?}", potencia(2, 63));
    println!("{:?}", potencia(2, 64));
}
