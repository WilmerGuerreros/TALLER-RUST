fn velocidad_minima(pilas: &[u64], h: u64) -> u64 {
    if pilas.is_empty() {
        return 0;
    }

    let cantidad = pilas.iter().filter(|&&x| x > 0).count() as u64;

    if h < cantidad {
        return 0;
    }

    let mut izquierda = 1u64;
    let mut derecha = *pilas.iter().max().unwrap();

    while izquierda < derecha {
        let medio = izquierda + (derecha - izquierda) / 2;
        let mut horas = 0u64;

        for &pila in pilas {
            horas += pila / medio;

            if pila % medio != 0 {
                horas += 1;
            }

            if horas > h {
                break;
            }
        }

        if horas <= h {
            derecha = medio;
        } else {
            izquierda = medio + 1;
        }
    }

    izquierda
}

fn main() {
    println!(
        "Velocidad mínima: {}",
        velocidad_minima(&[3, 6, 7, 11], 8)
    );

    println!(
        "Velocidad mínima: {}",
        velocidad_minima(&[10], 2)
    );
}
