use std::env;
use std::sync::RwLock;

#[derive(Debug, Copy, Clone)]
enum TokenKind{
    TkReserved,
    TkNum,
    TkEof
}

#[derive(Debug, Copy, Clone)]
struct Token{
    kind : TokenKind,
    val : u32,
    str : char
}

fn error(message : &str){
    eprintln!("{}", message);
}

fn consume(token : &Token, op : char) -> bool {
    if !matches!(token.kind, TokenKind::TkReserved) || token.str != op {
        return false;
    }

    return true;
}

fn expect(token : &Token, op : char){
    if !matches!(token.kind, TokenKind::TkNum) {
        error("数ではありません");
    }
}

fn expect_number(token : &Token) -> u32{
    if !matches!(token.kind, TokenKind::TkNum) {
        error("数ではありません");
    }

    let val = token.val;
    return val;
}

fn at_eof(token : &Token) -> bool {
    return matches!(token.kind, TokenKind::TkEof);
}

fn new_token(kind : &TokenKind, val : u32, str : char) -> Token{
    let token = Token{
        kind : *kind,
        val : val,
        str : str
    };

    return token;
}

fn tokenize(str : Vec<char>) -> Vec<Token> {
    let mut cur: Vec<Token> = Vec::new();
    for n in 0..str.len() {
        if str[n].is_whitespace(){
            continue;
        }

        if str[n] == '+' || str[n] == '-' {
            cur.push(new_token(&TokenKind::TkReserved, 0, str[n]));
            continue;
        }

        let result = str[n].to_digit(10);
        match result {
            Some(i) => cur.push(new_token(&TokenKind::TkNum, i, str[n])),
            None => error("トークナイズできません"),
        }
    }

    cur.push(new_token(&TokenKind::TkEof, 0, '\0'));

    return cur;
}

fn main(){
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Application error");
    }

    let arg: Vec<char> = args[1].chars().collect::<Vec<char>>();
    let tokens = tokenize(arg);
    let mut idx = 0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", tokens[idx].val);
    idx += 1;
    
    while !at_eof(&tokens[idx]) {
        if consume(&tokens[idx], '+') {
            println!("  add rax, {}", expect_number(&tokens[idx + 1]));
            idx += 2;
            continue;
        }

        expect(&tokens[idx], '-');
        println!("  sub rax, {}", expect_number(&tokens[idx + 1]));
        idx += 2;
        continue;
    }

    println!("  ret");
}