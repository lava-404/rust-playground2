#[derive(Debug)]
enum Token {
    Number(i32),
    Keyword(String),
    Symbol(char),
    Identifier(String),
}

fn tokenize(code: &str) -> Vec<Token> {
    let keywords = vec![
        "rule", "if", "then", "in", "delete", "mask",
        "notify", "encrypt", "and", "or", "not", "let"
    ];
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut is_number = false;

    for c in code.chars() {
        if c.is_alphabetic() {
            buffer.push(c);
        } else if c.is_numeric() {
            buffer.push(c);
            is_number = true;
        } else if "(){};:=+-".contains(c) {
            // Flush buffer BEFORE pushing symbol
            if !buffer.is_empty() {
                if is_number {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                } else if keywords.contains(&buffer.as_str()) {
                    tokens.push(Token::Keyword(buffer.clone()));
                } else {
                    tokens.push(Token::Identifier(buffer.clone()));
                }
                buffer.clear();
                is_number = false;
            }
            tokens.push(Token::Symbol(c));
        } else if c.is_whitespace() {
            if !buffer.is_empty() {
                if is_number {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                } else if keywords.contains(&buffer.as_str()) {
                    tokens.push(Token::Keyword(buffer.clone()));
                } else {
                    tokens.push(Token::Identifier(buffer.clone()));
                }
                buffer.clear();
                is_number = false;
            }
        }
    }

    // Final flush
    if !buffer.is_empty() {
        if is_number {
            tokens.push(Token::Number(buffer.parse().unwrap()));
        } else if keywords.contains(&buffer.as_str()) {
            tokens.push(Token::Keyword(buffer.clone()));
        } else {
            tokens.push(Token::Identifier(buffer.clone()));
        }
    }

    tokens
}

fn main() {
    let code = "let x = 42 + y;";
    let tokens = tokenize(code);
    println!("{:?}", tokens);
}
