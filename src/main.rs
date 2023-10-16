use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if(args.len() != 2){
        eprintln!("Application error");
    }

    println!(".intel_syntax noprefix\n");
    println!(".globl main\n");
    println!("main:\n");
    println!("  mov rax, %d\n", args[1].parse().unwrap());
    println!("  ret\n");
}