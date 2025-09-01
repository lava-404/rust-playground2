
use std::fmt;
use thiserror::Error;


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Ident(String),
    Number(i64),
    Symbol(char),        
    Operator(String),   
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("unexpected character: '{0}' at position {1}")]
    UnexpectedChar(char, usize),
    #[error("unterminated token at end of input")]
    Unterminated,   
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let keywords = [
        "rule","if","then","in","and","or","not",
        "delete","mask","notify","encrypt",
    ];

    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut idx = 0usize;

    while let Some(&c) = chars.peek() {
        // skip whitespace
        if c.is_whitespace() {
            chars.next(); idx += 1; continue;
        }

        // identifier / keyword
        if is_ident_start(c) {
            let mut s = String::new();
            while let Some(&ch) = chars.peek() {
                if is_ident_continue(ch) { s.push(ch); chars.next(); idx += 1; }
                else { break; }
            }
            if keywords.contains(&s.as_str()) {
                tokens.push(Token::Keyword(s));
            } else {
                tokens.push(Token::Ident(s));
            }
            continue;
        }

        // number
        if c.is_ascii_digit() {
            let mut s = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() { s.push(ch); chars.next(); idx += 1; }
                else { break; }
            }
            let n: i64 = s.parse().unwrap();
            tokens.push(Token::Number(n));
            continue;
        }

      
        let two = {
            let mut it = chars.clone();
            let first = it.next();
            let second = it.next();
            match (first, second) {
                (Some(a), Some(b)) => Some(format!("{a}{b}")),
                _ => None,
            }
        };

        if let Some(op2) = two {
            if ["==","!=" ,">=","<="].contains(&op2.as_str()) {
                // consume 2
                chars.next(); chars.next(); idx += 2;
                tokens.push(Token::Operator(op2));
                continue;
            }
        }
        if ['=','>','<'].contains(&c) {
            chars.next(); idx += 1;
            tokens.push(Token::Operator(c.to_string()));
            continue;
        }

        // symbols
        if "{}()[],.;".contains(c) {
            chars.next(); idx += 1;
            tokens.push(Token::Symbol(c));
            continue;
        }

        // unknown
        return Err(LexError::UnexpectedChar(c, idx));
    }

    Ok(tokens)
}



#[derive(Debug)]
pub struct Program { pub rules: Vec<Rule> }

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Statement {
    pub condition: Expr,
    pub action: Action,
}

#[derive(Debug, Clone)]
pub enum Action { Delete, Mask, Notify, Encrypt }

#[derive(Debug, Clone)]
pub enum Expr {
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Group(Box<Expr>),
    Compare { left: Operand, op: CompOp, right: Operand },
    In { field: Field, set: Vec<String> },
}

#[derive(Debug, Clone, Copy)]
pub enum CompOp { Eq, Ne, Gt, Lt, Ge, Le }

#[derive(Debug, Clone)]
pub enum Operand {
    Number(i64),
    Duration { value: i64, unit: String }, 
    Field(Field),
}

#[derive(Debug, Clone)]
pub struct Field { pub segments: Vec<String> } 

impl fmt::Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CompOp::*;
        write!(f, "{}", match self { Eq=>"==", Ne=>"!=", Gt=>">", Lt=>"<", Ge=>">=", Le=>"<=" })
    }
}

//
// ===== PARSER =====
//

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected end of input")]
    Eof,
    #[error("unexpected token: {0:?}")]
    Unexpected(Token),
    #[error("expected {expected}, found {found:?}")]
    Expected { expected: String, found: Token },
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn advance(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() { self.pos += 1; }
        t
    }
    fn expect_symbol(&mut self, ch: char) -> Result<(), ParseError> {
        match self.advance() {
            Some(Token::Symbol(c)) if c == ch => Ok(()),
            Some(t) => Err(ParseError::Expected { expected: format!("'{}'", ch), found: t }),
            None => Err(ParseError::Eof),
        }
    }
    fn expect_keyword(&mut self, kw: &str) -> Result<(), ParseError> {
        match self.advance() {
            Some(Token::Keyword(s)) if s == kw => Ok(()),
            Some(t) => Err(ParseError::Expected { expected: kw.to_string(), found: t }),
            None => Err(ParseError::Eof),
        }
    }
    fn match_keyword(&mut self, kw: &str) -> bool {
        if matches!(self.peek(), Some(Token::Keyword(s)) if s == kw) {
            self.pos += 1; true
        } else { false }
    }
    fn match_symbol(&mut self, ch: char) -> bool {
        if matches!(self.peek(), Some(Token::Symbol(c)) if *c == ch) {
            self.pos += 1; true
        } else { false }
    }
    fn match_op(&mut self, s: &str) -> bool {
        if matches!(self.peek(), Some(Token::Operator(op)) if op == s) {
            self.pos += 1; true
        } else { false }
    }
    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Some(Token::Ident(s)) => Ok(s),
            Some(t) => Err(ParseError::Expected { expected: "identifier".to_string(), found: t }),
            None => Err(ParseError::Eof),
        }
    }
    fn expect_number(&mut self) -> Result<i64, ParseError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(n),
            Some(t) => Err(ParseError::Expected { expected: "number".to_string(), found: t }),
            None => Err(ParseError::Eof),
        }
    }

    // program := { rule }
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut rules = Vec::new();
        while let Some(Token::Keyword(k)) = self.peek() {
            if k == "rule" {
                rules.push(self.parse_rule()?);
            } else {
                return Err(ParseError::Unexpected(self.peek().unwrap().clone()));
            }
        }
        Ok(Program { rules })
    }

    fn parse_rule(&mut self) -> Result<Rule, ParseError> {
        self.expect_keyword("rule")?;
        let name = self.expect_ident()?;
        self.expect_symbol('{')?;
        let mut statements = Vec::new();
        while !self.match_symbol('}') {
            statements.push(self.parse_statement()?);
            // optional semicolon
            let _ = self.match_symbol(';');
        }
        Ok(Rule { name, statements })
    }


    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect_keyword("if")?;
        let condition = self.parse_condition()?;
        self.expect_keyword("then")?;
        let action = self.parse_action()?;
        Ok(Statement { condition, action })
    }


    fn parse_condition(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }


    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.match_keyword("or") {
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }


    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_not()?;
        while self.match_keyword("and") {
            let right = self.parse_not()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, ParseError> {
        if self.match_keyword("not") {
            let inner = self.parse_not()?;
            Ok(Expr::Not(Box::new(inner)))
        } else {
            self.parse_predicate()
        }
    }


    fn parse_predicate(&mut self) -> Result<Expr, ParseError> {
        if self.match_symbol('(') {
            let inner = self.parse_condition()?;
            self.expect_symbol(')')?;
            return Ok(Expr::Group(Box::new(inner)));
        }


        let save = self.pos;
        if let Ok(field) = self.parse_field() {
            if self.match_keyword("in") {
                self.expect_symbol('[')?;
                let mut list = Vec::new();
                loop {
                    let ident = self.expect_ident()?;
                    list.push(ident);
                    if self.match_symbol(']') { break; }
                    self.expect_symbol(',')?;
                }
                return Ok(Expr::In { field, set: list });
            } else {
                
                self.pos = save;
            }
        }

        let left = self.parse_operand()?;
        let op = self.parse_comp_op()?;
        let right = self.parse_operand()?;
        Ok(Expr::Compare { left, op, right })
    }

    fn parse_comp_op(&mut self) -> Result<CompOp, ParseError> {
        use CompOp::*;
        if self.match_op("==") { return Ok(Eq); }
        if self.match_op("!=") { return Ok(Ne); }
        if self.match_op(">=") { return Ok(Ge); }
        if self.match_op("<=") { return Ok(Le); }
        if self.match_op(">")  { return Ok(Gt); }
        if self.match_op("<")  { return Ok(Lt); }
        Err(self.unexpected("comparison operator"))
    }

    fn unexpected(&mut self, expected: &str) -> ParseError {
        match self.peek().cloned() {
            Some(t) => ParseError::Expected { expected: expected.to_string(), found: t },
            None => ParseError::Eof,
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, ParseError> {
        match self.peek().cloned() {
            Some(Token::Number(_)) => {
                let n = self.expect_number()?;
                if let Some(Token::Ident(unit)) = self.peek().cloned() {
                    // duration
                    self.advance();
                    Ok(Operand::Duration { value: n, unit })
                } else {
                    Ok(Operand::Number(n))
                }
            }
            _ => {
                let field = self.parse_field()?;
                Ok(Operand::Field(field))
            }
        }
    }


    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let first = self.expect_ident()?;
        let mut segs = vec![first];
        while self.match_symbol('.') {
            segs.push(self.expect_ident()?);
        }
        Ok(Field { segments: segs })
    }

    fn parse_action(&mut self) -> Result<Action, ParseError> {
        match self.advance() {
            Some(Token::Keyword(k)) => match k.as_str() {
                "delete" => Ok(Action::Delete),
                "mask"   => Ok(Action::Mask),
                "notify" => Ok(Action::Notify),
                "encrypt"=> Ok(Action::Encrypt),
                _ => Err(ParseError::Expected { expected: "action keyword".to_string(), found: Token::Keyword(k) })
            },
            Some(t) => Err(ParseError::Expected { expected: "action keyword".to_string(), found: t }),
            None => Err(ParseError::Eof),
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example DSL program
    let src = r#"
        rule delete_old_data {
            if record.age_in_days > 30 days then delete;
            if field in [ssn, credit_card] and not user.is_admin then mask
        }

        rule alert_weird {
            if (user.country == blocked_country) or user.failed_logins >= 5 then notify
        }
    "#;

    let tokens = lex(src)?;
    println!("TOKENS:");
    for t in &tokens { println!("  {:?}", t); }

    let mut p = Parser::new(tokens);
    let program = p.parse_program()?;

    println!("\nAST:");
    for r in &program.rules {
        println!("Rule: {}", r.name);
        for (i, st) in r.statements.iter().enumerate() {
            println!("  Statement {i}:");
            println!("    Condition: {:?}", st.condition);
            println!("    Action:    {:?}", st.action);
        }
    }

    Ok(())
}
