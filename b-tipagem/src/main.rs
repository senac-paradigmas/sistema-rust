fn main() {
    println!("\n\t ---- Tipos Primitivos -----");

    // tipos primitivos
    // inteiros com sinal
    let _a: i32 = 10; // Rust o tipo pode ser anotado
    let _b = 3;        // se não for anotado, será inferido. Padrão i32
    let _c: i8 = 4;   // posso ter controle fino. Inteiro de 8 bits se o valor é pequeno

    //_c = 2; // este código irá gerar um erro. Variáveis são immutáveis por padrão

    let mut d: i8 = 3; // preciso definir que é mutável com mut
    // Mas constante é outra coisa
    // constante é um valor GLOBAL e variável é LOCAL
    const _THRESHOLD: i32 = 10;
    
    println!("Variável mutável d: {}", d);
    d = 10;
    println!("Variável mutável d: {}", d);

    // inteiros sem sinal
    let _a: u32 = 10; // Se não precisar de negativos, vc ganha o dobro de inteiros

    // ponto fluante
    let _e: f32 = 3.24; // por padrão, f64

    // booleano
    let _f: bool = true;

    // coleções
    // arrays
    println!("\n\t ---- Arrays -----");
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // arrays são de tamanho fixo. Posso assinar i32 e ter valor padrão 5 (opcional)
    println!("tamanho do array: {}", xs.len());
    println!("Segundo elemento: {}", xs[1]);

    // tuplas
    let par = (1, true);
    println!("\n\t ---- Tuplas -----");
    println!("par {:?}", par);
    println!("Primeiro Elemento: {}", par.0);
}
