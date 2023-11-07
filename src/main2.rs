use std::env;

enum token_kind{
    TK_RESERVED,
    TK_NUM,
    TK_EOF
}

struct token{

}

struct token{
    token_kind kind,
    int val,
    Vec<char> str
}

fn error(Vec<char> str){

}

fn consume(char op) -> bool {
    if(token.kind != TK_RESERVED || token->str[0] != op){
        return false;
    }

    token = token.next;
    return true;
}

fn expect(char op){
    if(token.kind != TK_NUM){
        error("数ではありません");
    }

    token = token.next;
}

fn expect_number(){
    if(token->kind != TK_NUM){
        error("数ではありません");
    }

    let val = token.val;
    token = token.next;
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

token tokenize(Vec<char> str){
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

    cur.push(new_token(TK_EOF, str[n]));
}