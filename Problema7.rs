struct ColaDosPilas {
    entrada: Vec<i64>,
    salida: Vec<i64>,
}

impl ColaDosPilas {
    fn nueva() -> ColaDosPilas {
        ColaDosPilas {
            entrada: Vec::new(),
            salida: Vec::new(),
        }
    }

    fn encolar(&mut self, x: i64) {
        self.entrada.push(x);
    }

    fn pasar_elementos(&mut self) {
        if self.salida.is_empty() {
            while let Some(valor) = self.entrada.pop() {
                self.salida.push(valor);
            }
        }
    }

    fn desencolar(&mut self) -> Option<i64> {
        self.pasar_elementos();
        self.salida.pop()
    }

    fn frente(&mut self) -> Option<i64> {
        self.pasar_elementos();
        self.salida.last().copied()
    }
}

fn main() {
    let mut cola = ColaDosPilas::nueva();

    cola.encolar(10);
    cola.encolar(20);
    cola.encolar(30);

    println!("Primero: {:?}", cola.frente());
    println!("Sale: {:?}", cola.desencolar());
    println!("Sale: {:?}", cola.desencolar());

    cola.encolar(40);

    println!("Primero ahora: {:?}", cola.frente());
    println!("Sale: {:?}", cola.desencolar());
    println!("Sale: {:?}", cola.desencolar());
    println!("Cola vacía: {:?}", cola.desencolar());
}
