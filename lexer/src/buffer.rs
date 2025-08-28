#[derive(Debug)]
enum Token {
    Identifier(String),
    Number(i32),
    Symbol(char),
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();        // a vector to store tokens
    let mut buffer = String::new();     // collects characters for identifiers/numbers
    let mut is_number = false;          // flag to track if buffer is number

    for c in code.chars() {
        if c.is_alphabetic() {          // letters → identifiers
            buffer.push(c);
            is_number = false;
        } else if c.is_numeric() {      // digits → numbers
            buffer.push(c);
            if buffer.chars().all(|ch| ch.is_numeric()) {
                is_number = true;
            }
        } else if "(){};=+".contains(c) { // symbols → push directly
            if !buffer.is_empty() {
                if is_number {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                } else {
                    tokens.push(Token::Identifier(buffer.clone()));
                }
                buffer.clear();
            }
            tokens.push(Token::Symbol(c));
        } else if c.is_whitespace() {    // space → finalize buffer
            if !buffer.is_empty() {
                if is_number {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                } else {
                    tokens.push(Token::Identifier(buffer.clone()));
                }
                buffer.clear();
            }
        }
    }

    // after loop ends, push last token if buffer not empty
    if !buffer.is_empty() {
        if is_number {
            tokens.push(Token::Number(buffer.parse().unwrap()));
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
