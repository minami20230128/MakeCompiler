use std::env;

fn main(){
    let args : Vec<String> = env::args().collect();

    if(args.len() != 2){
        eprintln!("Application error");
    }

    let arg: Vec<char> = args[1].chars().collect::<Vec<char>>();
    let mut itr = arg.iter();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", itr.next().unwrap().to_string());
    
    while(itr.next() != None){
        if(itr.next().unwrap().to_string() == "+"){
            println!("  add rax, {}", itr.next().unwrap().to_string());
            continue;
        }

        if(itr.next().unwrap().to_string() == "-"){
            println!("  sub rax, {}", itr.next().unwrap().to_string());
            continue;
        }

        eprintln!("Application error");
    }

    println!("  ret");
}