use std::io;
pub fn particiones_rec(n: u32, maximo: u32, llamadas: &mut u64) -> u64 {
    *llamadas += 1;

    if n == 0 {
        return 1;
    }

    if maximo == 0 {
        return 0;
    }

    let maximo = maximo.min(n);

    let izquierda = particiones_rec(n, maximo - 1, llamadas);

    let derecha = particiones_rec(n - maximo, maximo, llamadas);

    izquierda + derecha
}
pub fn particiones(n: u32) -> u64 {
    let t = n as usize + 1;
    let mut dinamico: Vec<Vec<Option<u64>>> = vec![vec![None; t]; t];

    calcular_dp(n, n, &mut dinamico)
}

pub fn calcular_dp(n: u32, maximo: u32, dp: &mut Vec<Vec<Option<u64>>>) -> u64 {
    if n == 0 {
        return 1;
    }

    if maximo == 0 {
        return 0;
    }

    let maximo = maximo.min(n);

    let i = n as usize;
    let j = maximo as usize;

    if let Some(resultado) = dp[i][j] {
        return resultado;
    }

    let izquierda = calcular_dp(n, maximo - 1, dp);
    let derecha = calcular_dp(n - maximo, maximo, dp);

    dp[i][j] = Some(izquierda + derecha);

    dp[i][j].unwrap()
}

fn main() {
    println!("Inserte el número K [máximo sumando usable]: ");
    let mut a: String = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let k: u32 = a.trim().parse().unwrap();
    println!("Inserte el número n a ramificar: ");
    a = "".to_string();
    io::stdin().read_line(&mut a).unwrap();
    let lim: u32 = a.trim().parse().unwrap();
    let mut l: u64 = 0;
    println!(
        "Las particiones totales son {}",
        particiones_rec(lim, k, &mut l)
    );
    println!("Particiones con DP: {}", particiones(lim));
}
