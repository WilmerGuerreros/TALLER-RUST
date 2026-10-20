use std::io;

pub fn raiz_entera(n: u64, lim_sup: &mut u64, lim_inf: &mut u64) -> u64 {
    if *lim_sup - *lim_inf > 1 {
        let media: u64 = (*lim_sup + *lim_inf) / 2;
        if (media as u128 * media as u128) > n as u128 {
            *lim_sup = media;
            raiz_entera(n, lim_sup, lim_inf)
        } else {
            if media.strict_pow(2) == n {
                media
            } else {
                *lim_inf = media;
                raiz_entera(n, lim_sup, lim_inf)
            }
        }
    } else {
        if *lim_sup - *lim_inf == 1 {
            *lim_sup + 1
        } else {
            *lim_inf
        }
    }
}

fn main() {
    println!("Ingrese el número: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut input: u64 = a.trim().parse().unwrap();
    println!("Raíz binaria: {}", raiz_entera(input, &mut input, &mut 0));
}
