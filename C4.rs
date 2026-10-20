use std::io;

pub fn invertir_arreglo(v: &mut [i32]) {
    let n = v.len();
    for i in 0..n / 2 {
        let j = n - (i + 1);
        let temp = v[i];
        v[i] = v[j];
        v[j] = temp;
    }
}

fn main() {
    let mut qqq: Vec<i32> = Vec::new();

    println!("Ingrese el tamaño del arreglo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let n: usize = linea.trim().parse().unwrap();

    for i in 0..n {
        println!("Ingrese el {} número:", i);
        let mut linea = String::new();
        io::stdin().read_line(&mut linea).unwrap();
        let aux: i32 = linea.trim().parse().unwrap();
        qqq.push(aux);
    }

    println!("Arreglo: {:?}", qqq);
    invertir_arreglo(&mut qqq);
    println!("Invertido: {:?}", qqq);
}
