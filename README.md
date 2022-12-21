# Calculando a área de um Retângulo com Rust!

Do livro [A linguagem de Programação Rust](https://rust-br.github.io/rust-book-pt-br/ch05-02-example-structs.html) sobre Structs:

Para entender quando podemos querer usar structs, vamos escrever um programa que calcula a área de um retângulo. Usamos structs para dar significado aos dados usando rótulos.

```rs
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

```

Aqui temos definido um struct denominado Rectangle. Dentro das {} definimos os campos como comprimento e largura, ambas do tipo u32. Em main, criamos uma instância específica de um Rectangle que tem um comprimento de 50 e largura de 30.

A nossa função área agora é definida com um parâmetro, que chamamos rectangle, cujo tipo é um empréstimo de uma instância da struct imutável Rectangle. Como mencionado no capítulo 4, queremos usar a struct, em vez de tomar posse dela. Desta forma, main mantém-se a sua proprietaria e pode continuar a usar o rect1, que é a razão para usar o & na assinatura da função e onde chamamos a função.

A função área acessa os campos comprimento e largura da instância Rectangle. Nossa função assinatura para área agora indica exatamente o que queremos dizer: calcular a área de um Rectangle usando os campos lenght (comprimento) e width (largura). Transmite que o comprimento e a largura são relacionados uns aos outros, e dá nomes descritivos para os valores em vez de usar a tupla de valores de índice 0 e 1-uma vitória para uma maior clareza.

## Adicionando Funcionalidade com Util com Traits Derivadas

Rust inclui funcionalidades para imprimir informações de depuração, mas temos de inseri-la explicitamente para tornar essa funcionalidade disponível para nossa struct. Para isso, adicionamos a anotação ```#[derive(Debug)]```.

```rs
	#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
}
```

Agora, quando executamos o programa, vamos ver a seguinte informação:

```rect1 is Rectangle { length: 50, width: 30 }```