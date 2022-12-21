#[derive(Debug)]
struct Retangulo {
    comprimento: u32,
    largura: u32,
}

fn main() {
    let ret = Retangulo {
           comprimento: 30,
           largura: 60
    };
    println!("\n<{:?}>", ret); // deixei o :? ao invés do :#? porque achei
    println!(                  // mais bonito de se ler 
        "<#> a área do retângulo é igual a {}\n",
        area(&ret)
    );   
}
fn area(retangulo: &Retangulo) -> u32 {
    retangulo.comprimento * retangulo.largura
}



