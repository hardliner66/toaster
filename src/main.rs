use std::io::{BufRead, Write};

#[allow(dead_code)]
enum Token {
    Char(char),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Identifier(String),
    LeftParen,
    RightParen,
    Quote(Vec<Token>),
}

fn tokenize(input: &str) -> Vec<Token> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut c: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    while c < chars.len() {
        tokens.push(match chars[c] {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '"' => {
                let mut buf = Vec::new();
                c += 1;
                while (chars[c]) != '"' || (chars[c - 1] == '\\') {
                    buf.push(chars[c] as u8);
                    c += 1;
                    if c >= chars.len() {
                        panic!("Un terminated string");
                    }
                }
                Token::String(String::from_utf8_lossy(&buf).to_string())
            }
            _ => {
                let mut buf = Vec::new();
                while c < chars.len()
                    && chars[c] != ' '
                    && chars[c] != '\t'
                    && chars[c] != '\n'
                    && chars[c] != '\r'
                {
                    buf.push(chars[c]);
                    c += 1;
                }
                Token::Identifier(String::from_iter(buf))
            }
        });
        c += 1;
    }
    return tokens;
}

fn eval(input: &[Token]) {
    for token in input {
        match token {
            Token::Char(c) => println!("{}", c),
            Token::String(s) => println!("{}", s),
            Token::Int(i) => println!("{}", i),
            Token::Float(f) => println!("{}", f),
            Token::Bool(b) => println!("{}", b),
            Token::Identifier(s) => println!("{}", s),
            Token::LeftParen => println!("("),
            Token::RightParen => println!(")"),
            Token::Quote(v) => {
                println!("[");
                eval(&v);
                println!("]");
            }
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut running = true;
    while running {
        std::io::stdout().flush().unwrap();
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            running = false;
        } else if input.trim().is_empty() {
            continue;
        } else {
            let tokens = tokenize(input.trim());
            eval(&tokens);
        }
    }
}
