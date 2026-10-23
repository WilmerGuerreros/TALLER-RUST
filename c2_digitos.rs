
fn digitos_distintos(mut n: u64) -> u32 {
    let mut vistos = [false; 10];
    let mut cantidad = 0;

    if n == 0 {
        return 1;
    }

    while n > 0 {
        let digito = (n % 10) as usize;

        if !vistos[digito] {
            vistos[digito] = true;
            cantidad += 1;
        }

        n /= 10;
    }

    cantidad
}

fn main() {
    println!("112233 = {}", digitos_distintos(112233));
    println!("777777 = {}", digitos_distintos(777777));
    println!("12345 = {}", digitos_distintos(12345));
    println!("0 = {}", digitos_distintos(0));
}
