use std::convert::From;

// Parecida com Structs do C
struct Coordenada {
    x: f32,
    y: f32,
}

// estruturas podem estar em outras estrutuas
struct Campus {
    nome: String,
    coordenada: Coordenada,
}

// alias - Apelidos de tipos
type Segundo = u32; // no contexto, tempo não pode ser negativo
type _Hora = u32; // no contexto, tempo não pode ser negativo

fn main() {
    let cas_xy = Coordenada {x: -23.6190872, y: -46.6703841};
    let cas = Campus {nome: String::from("Santo Amaro"), coordenada: cas_xy};

    println!("SENAC {0} está em ({1},{2})", cas.nome, cas.coordenada.x, cas.coordenada.y);

    let segundos: Segundo = 1000;
    println!("Segundos: {}", segundos)
}
