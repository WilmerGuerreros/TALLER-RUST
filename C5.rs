use std::io;

pub fn fib_rec(n: u32, llamadas: &mut u64) -> u64 {
    *llamadas += 1;
    if n <= 1 {
        n as u64
    } else {
        fib_rec(n - 1, llamadas) + fib_rec(n - 2, llamadas)
    }
}

fn main() {
    let mut llamadas: u64 = 0;
    let mut makanaky = String::new();
    println!("Ingresa el número papay: ");
    io::stdin().read_line(&mut makanaky).unwrap();
    let a: u32 = makanaky.trim().parse().unwrap();
    let resultado = fib_rec(a, &mut llamadas);
    println!("Resultado: { } con { } llamadas", resultado, llamadas);
}
