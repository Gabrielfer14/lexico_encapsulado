use std::io::{self,Write};

struct Analisador<'a> {
    pos: usize, // Pos atual na str
    prox: &'a str, // Resto da str a se analisar
}
 
impl<'a> Analisador<'a> {
    fn novo(entrada:&'a str) -> Self {
        Self{
            pos: 1,
            prox: entrada
        }
    }
 
    fn próximo(&mut self) -> Result<(usize, &str), Option<usize>> {
        while let Some((posicao_relativa, c)) = self.prox.char_indices().next() {
            match c {
                '0'..='9' => {
                    // Caso encontre um número
                    let end = self.prox[posicao_relativa..]
                        .find(|ch: char| !ch.is_ascii_digit())
                        .map_or(self.prox.len(), |index| posicao_relativa + index);
                    let numero = &self.prox[posicao_relativa..end];
                    self.pos += end - posicao_relativa; // Atualizar pos atual
                    self.prox = &self.prox[end..]; // Atualizar a parte restante
                    return Ok((self.pos - numero.len(), numero));
                }
                '+' | '-' | '*' | '/' => {
                    // Caso encontre um operador
                    let operador = &self.prox[posicao_relativa..posicao_relativa + 1];
                    self.pos += 1; // Atualiza a pos atual
                    self.prox = &self.prox[posicao_relativa + 1..]; // Atualizar a parte restante
                    return Ok((self.pos - 1, operador));
                }
                ' ' => {
                    // Ignora espaços, mas continua iterando
                    self.prox = &self.prox[posicao_relativa + 1..];
                    self.pos += 1;
                    continue;
                }
                _ => return Err(Some(self.pos + posicao_relativa)), // Caso encontre um caractere inválido
            }
        }
        Err(None) // Caso não haja mais string
    }
 }

fn main() {
    println!("Digite a expressão para o analisador:");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

       let mut analisador = Analisador::novo(input.trim_end()); // Inicializa o analisador
        while !analisador.prox.is_empty() {
            match analisador.próximo() {
                Ok((posicao, elemento)) => {
                    print!("('{}', {})", elemento, posicao);
                }
                Err(Some(posicao)) => {
                    print!("(Erro na posição {})", posicao);
                    break;
                }
                Err(None) => {
                    break
                }
            }
        }
        break;
    }
}

 
 