fn anterior_menor(n: u64) -> Option<u64> {
    let mut digitos: Vec<u8> =
        n.to_string().bytes().map(|b| b - b'0').collect();

    if digitos.len() < 2 {
        return None;
    }

    let mut i = digitos.len() - 2;

    loop {
        if digitos[i] > digitos[i + 1] {
            break;
        }

        if i == 0 {
            return None;
        }

        i -= 1;
    }

    let mut j = digitos.len() - 1;

    while digitos[j] >= digitos[i] {
        j -= 1;
    }

    digitos.swap(i, j);

    let mut izquierda = i + 1;
    let mut derecha = digitos.len() - 1;

    while izquierda < derecha {
        digitos.swap(izquierda, derecha);
        izquierda += 1;
        derecha -= 1;
    }

    if digitos[0] == 0 {
        return None;
    }

    let mut resultado = 0u64;

    for d in digitos {
        resultado = resultado * 10 + d as u64;
    }

    Some(resultado)
}

fn main() {
    println!("1324 -> {:?}", anterior_menor(1324));
    println!("1023 -> {:?}", anterior_menor(1023));
    println!("1234 -> {:?}", anterior_menor(1234));
}