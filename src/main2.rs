use std::env;
use std::sync::RwLock;

enum token_kind{
    TK_RESERVED,
    TK_NUM,
    TK_EOF
}

struct token{
    token_kind kind,
    int val,
    Vec<char> str
}

static tokens: RwLock<token> = RwLock::new(token:Vec::new());
static mut idx i32 = 0;

fn error(str& message){
    eprintln!(message);
}

fn consume(char op) -> bool {
    if(token.kind != TK_RESERVED || token->str[0] != op){
        return false;
    }

    unsafe{
        idx++;
    }

    return true;
}

fn expect(char op){
    if(token.kind != TK_NUM){
        error("数ではありません");
    }

    unsafe{
        idx++;
    }
}

fn expect_number(){
    if(token->kind != TK_NUM){
        error("数ではありません");
    }

    let val = token.val;
    unsafe{
        idx++;
    }
    return val;
}

fn at_eof -> bool (){
    return token.kind == TK_EOF;
}

fn new_token(token_kind kind, Vec<char> str){
    let token = Token{
        token.kind = kind;
        token.str = str[n];
    }
}

token tokenize(Vec<char> str) -> Vec<token> cur{
    Vec<token> cur;
    for n in (str.len()){
        if(str[n].is_whitespace()){
            continue;
        }

        if(str[n] == '+' || str[n] == '-'){
            cur.push(new_token(TK_RESERVED, str[n]));
            continue;
        }

        if(!str[n].parse::<i32>().is_err()){
            cur.push(new_token(TK_NUM, str[n]));
            continue;
        }

        error("トークナイズできません");
    }

    cur.push(new_token(TK_EOF, ""));
}

fn main(){
    let args : Vec<String> = env::args().collect();

    if(args.len() != 2){
        eprintln!("Application error");
    }

    let arg: Vec<char> = args[1].chars().collect::<Vec<char>>();
    let tokens = tokenize(arg);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", arg[0]);
    
    while(!at_eof()){
        if(consume('+')){
            println!("  add rax, {}", expect_number());
            continue;
        }

        expect('-');
        println!("  sub rax, {}", expect_number());
        continue;
    }

    println!("  ret");
}