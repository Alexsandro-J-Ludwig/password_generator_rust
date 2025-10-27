use rand::Rng;
use std::io;

fn main() {
    user_mananger();
}

fn user_mananger(){
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let especial = "!@#$%&*()><:=+-,./?;";

    let mut config = String::new();
    
    println!("Selecione suas opções de senha\nA)Apenas letras\nB)Letras e números\nC)Letras, números e símbolos\n");

    io::stdin()
        .read_line(&mut config)
        .expect("Failed to read line");
    
    let pass_final = match config.trim() {
        "A" => render_pass(alpha),
        "B" => render_pass(&(alpha.to_string() + numbers)),
        "C" => render_pass(&(alpha.to_string() + numbers + especial)),
        _ => {
            eprintln!("Opção inválida, selecione A, B ou C");
            std::process::exit(1);
        }
    };
    
    println!("Sua senha é {}", pass_final);
}

fn render_pass(elemento:&str) -> String{
    let mut rng = rand::thread_rng();
    let qtd:usize = rng.gen_range(8..51);
    let chars: Vec<char> = elemento.chars().collect();
    let mut pass = String::with_capacity(qtd);

    while pass.len() < qtd {
        let idx = rng.gen_range(0..chars.len());
        pass.push(chars[idx]);
    }

    pass
}
