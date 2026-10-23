use std::io;

pub fn a_lo_sumo_dos(v: &mut Vec<i32>) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut escribir = 1;
    let mut actual = v[0];
    let mut contador = 1;

    for leer in 1..v.len() {
        if v[leer] == actual {
            contador += 1;
        } else {
            actual = v[leer];
            contador = 1;
        }

        if contador <= 2 {
            v[escribir] = v[leer];
            escribir += 1;
        }
    }

    v.truncate(escribir);
    escribir
}

fn main() {
    let mut vector: Vec<i32> = Vec::new();

    let mut a: String = String::new();
    println!("Inserte la longitud del vector: ");
    io::stdin().read_line(&mut a).unwrap();
    let longitud: i32 = a.trim().parse().unwrap();
    for i in 1..longitud + 1 {
        println!("Ingrese el {}° valor del vector", i);
        let mut b: String = String::new();
        io::stdin().read_line(&mut b).unwrap();

        let input: i32 = b.trim().parse().unwrap();
        vector.push(input);
    }

    println!(
        "La nueva longitud del vector es {}",
        a_lo_sumo_dos(&mut vector)
    );
    /*
    REGLAS VIOLADAS EN ESTE APARTADO 6.a:
    1) El bucle:
    El préstamo de "v" YA ESTÁ siendo utilizada por el bucle for, en forma de condicional...
    Esto significa que ningún otro proceso se reserva el derecho de editarlo a menos que el bucle finalize,
    o simple y llanamente creo otra variable "copia" para que los permisos de esta copia no dependa del "v"
    ya prestado

    2) El v.push:
    Este comando le está pidiendo el permiso a "v", pero YA ESTÁ TOMADO. Por ende, no puede ejecutar su orden,
    y arroja error.

    3) ¿Por qué ocurre esto?:
    Rust previene el hecho de acceder a "vacíos de memoria" o "memoria no correspondiente". ¿Y cómo sucede esto,
    si solamente se está tomando al vector? Lo que pasa es que ese "vector" solo consiste en un conjunto de
    direcciones/posiciones de memoria CONTIGÜAS: o todas van juntas, o buscan un lugar para estarlo. Y eso es
    el problema del v.push: si el vector NO RESERVÓ un espacio de memoria contigüo para almacenar ese nuevo valor
    (en otras palabras, que almacene más valores de "lo esperado"/"lo definido"), Rust llevará a todos los espacios
    de memoria del vector a un espacio contigüo MÁS amplio para almacenar el nuevo valor ingresado. Sin embargo,
    ¿qué pasa con los espacios de memoria pre-mudanza? Se limpian, quedan vacías y otros datos pueden tomar su lugar.
    Pero el iterador x está apuntando a ESE ESPACIO PRE-MUDANZA: por ende, estaría tomando datos QUE NO LE CORRESPONDE.
    ¿Y qué significa esto? Prácticamente un problema fuerte de ciberseguridad: puedo desplazar los datos originales y
    forzar al programa a poner otros datos (sensibles) en ese espacio de memoria, pudiendo acceder fácilmente a ellos
    al solamente poder acceder a un conjunto espacio de memoria pública.

    4) ¿Qué hace .clone()?
    Copia el CONTENIDO del vector original en ese momento, y almacena ese nuevo contenido en espacios contigüos de memoria
    completamentes independientes del original.

    5) Y sin usar .clone(), ¿cuál sería la solución?
    Simple: usamos una lectura que no implique referenciar más de una vez a "v". Por ejemplo, podemos crear una variable
    n que adopte la longitud de "v" (len n = v.len();), y que "i" itere desde 0 hasta dicho n, que sería el valor de la longitud
    de "v". De este modo, nos ahorramos el uso de punteros que puedan derivar en error de préstamo de permisos.

    let n = v.len();

    for i in 0..n {
        let x = v[i];

        if x % 2 == 1 {
            v.push(x * 10);
        }
    }
    */
}
