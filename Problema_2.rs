use std::io;

pub fn contar_largas(limite: u64, k: u32) -> u64 {
    if limite == 0 {
        return 0;
    }

    // memo[n] = longitud de Collatz de n; 0 = todavía no calculado
    let mut memo = vec![0; (limite + 1) as usize];
    memo[1] = 1;

    let mut contador: u64 = 0;
    for n in 1..=limite {
        if longitud(n, &mut memo) > k {
            contador += 1;
        }
    }
    contador
}

fn longitud(n: u64, memo: &mut [u32]) -> u32 {
    // Si ya está calculado, lo devolvemos
    if n < memo.len() as u64 && memo[n as usize] != 0 {
        return memo[n as usize];
    }

    // Siguiente término de Collatz
    let siguiente = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };

    // Longitud = 1 (este término) + la del siguiente
    let l = 1 + longitud(siguiente, memo);

    // Guardamos solo si cabe en el Vec
    if n < memo.len() as u64 {
        memo[n as usize] = l;
    }

    l
}

fn main() {
    println!("Ingrese el número k:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let k: u32 = a.trim().parse().unwrap();

    println!("Ingrese el límite:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let limite: u64 = b.trim().parse().unwrap();

    println!("Cantidad: {}", contar_largas(limite, k));
}
