use std::env;
use std::sync::RwLock;

enum token_kind{
    TK_RESERVED,
    TK_NUM,
    TK_EOF
}

struct token{
    kind : token_kind,
    val : int32,
    str : Vec<char>
}

fn error(message : &str){
    eprintln!(message);
}

fn consume(token : token, op : char) -> bool {
    if(!matches!(token.kind, token_kind::TK_RESERVED) || token.str[0] != op){
        return false;
    }

    return true;
}

fn expect(token : token, op : char){
    if(!matches!(token.kind, token_kind::TK_NUM)){
        error("数ではありません");
    }
}

fn expect_number(token : token) -> i32{
    if(!matches!(token.kind, token_kind::TK_NUM)){
        error("数ではありません");
    }

    let val = token.val;
    return val;
}

fn at_eof(token : token) -> bool {
    return matches!(token.kind == token_kind::TK_EOF);
}

fn new_token(kind : token_kind, str : Vec<char>) -> token{
    let token = token{
        kind = kind,
        val = 0,
        str = str
    };

    return token;
}

fn tokenize(str : Vec<char>) -> Vec<token> {
    Vec<token> cur;
    for n in (str.len()){
        if(str[n].is_whitespace()){
            continue;
        }

        if(str[n] == '+' || str[n] == '-'){
            cur.push(new_token(token_kind::TK_RESERVED, str[n]));
            continue;
        }

        if(!str[n].parse::<i32>().is_err()){
            cur.push(new_token(token_kind::TK_NUM, str[n]));
            continue;
        }

        error("トークナイズできません");
    }

    cur.push(new_token(token_kind::TK_EOF, ""));

    return cur;
}

fn main(){
    let args : Vec<String> = env::args().collect();

    if(args.len() != 2){
        eprintln!("Application error");
    }

    let arg: Vec<char> = args[1].chars().collect::<Vec<char>>();
    let tokens = tokenize(arg);
    let idx = 0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", arg[0]);
    
    while(!at_eof(tokens[idx])){
        if(consume(tokens[idx], '+')){
            println!("  add rax, {}", expect_number(tokens[idx]));
            idx += 1;
            continue;
        }

        expect(tokens[idx], '-');
        println!("  sub rax, {}", expect_number(tokens[idx]));
        idx += 1;
        continue;
    }

    println!("  ret");
}