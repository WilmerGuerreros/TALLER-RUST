use std::io;

pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}

impl Rectangulo {
    pub fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    pub fn escalar(&mut self, k: u32) {
        self.ancho *= k;
        self.alto *= k;
    }
}

fn main() {
    println!("Ingrese el ancho del rectángulo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let ancho: u32 = linea.trim().parse().unwrap();

    println!("Ingrese la altura del rectángulo:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let alto: u32 = linea.trim().parse().unwrap();

    let mut r = Rectangulo { ancho, alto };
    println!("Área original: {}", r.area());

    println!("Ingrese un número k a escalar, o ponga 1 si quiere cambios:");
    let mut linea = String::new();
    io::stdin().read_line(&mut linea).unwrap();
    let k: u32 = linea.trim().parse().unwrap();

    r.escalar(k);
    println!("Nueva área del rectángulo escalado: {}", r.area());
}
