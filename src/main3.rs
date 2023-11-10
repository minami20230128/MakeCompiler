use std::env;

fn main(){
    let args : Vec<String> = env::args().collect();

    if(args.len() != 2){
        eprintln!("Application error");
    }

    let arg: Vec<char> = args[1].chars().collect::<Vec<char>>();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", arg[0]);
    
    for n in (1..arg.len()).step_by(2) {
        if(arg[n].to_string() == "+".to_string()){
            println!("  add rax, {}", arg[n + 1].to_string());
            continue;
        }
        else if(arg[n].to_string() == "-".to_string()){
            println!("  sub rax, {}", arg[n + 1].to_string());
            continue;
        }

        eprintln!("Application error");
    }

    println!("  ret");
}