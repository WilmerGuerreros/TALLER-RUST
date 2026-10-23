
fn pasos_todos(limite: usize) -> Vec<u32> {
    let mut pasos = vec![0; limite + 1];

    for n in 2..=limite {
        let mut numero = n;
        let mut cantidad = 0;

        while numero > 1 {
            if numero % 2 == 0 {
                numero /= 2;
            } else {
                numero -= 1;
            }

            cantidad += 1;
        }

        pasos[n] = cantidad;
    }

    pasos
}

fn main() {
    let pasos = pasos_todos(10);

    for i in 0..=10 {
        println!("Número {}: {} pasos", i, pasos[i]);
    }
}
