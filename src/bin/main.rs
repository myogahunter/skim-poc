use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--man" => println!(".TH SK 1\n.SH NAME\nsk \\- fuzzy finder in Rust"),
            "--shell" => {
                if args.len() > 2 {
                    match args[2].as_str() {
                        "bash" => println!("# bash completion for sk"),
                        "zsh" => println!("# zsh completion for sk"),
                        "fish" => println!("# fish completion for sk"),
                        "nushell" => println!("# nushell completion for sk"),
                        _ => println!("# unknown shell"),
                    }
                }
            }
            _ => println!("sk - fuzzy finder"),
        }
    } else {
        println!("sk - fuzzy finder");
    }
}
