
// 1 - Definindo uma estrutura da linha de comando
struct Cli{
  padrao: String,
  arquivo: std::path::PathBuf,
}

fn main() {
  // 2 - Padrões de entrada
  let padrao = std::env::args().nth(1).expect("no pattern given");
  let caminho = std::env::args().nth(2).expect("no path given");
  let args = Cli {
    padrao: padrao,
    arquivo: std::path::PathBuf::from(caminho),
  };

  let content = std::fs::read_to_string(&args.arquivo)
    .expect("could not read file");

  for line in content.lines() {
    if line.contains(&args.padrao) {
      println!("{}", line);
    }
  }

}
