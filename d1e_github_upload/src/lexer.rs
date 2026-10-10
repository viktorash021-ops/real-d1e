#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Start,
    Finish,
    Print,
    Hash,
    As,
    Identifier(String),
    StringLiteral(String),
    Equals,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current = self.input[self.position];

            if current.is_whitespace() {
                self.position += 1;
                continue;
            }

            if current == '"' {
                self.position += 1;
                let start_pos = self.position;
                while self.position < self.input.len() && self.input[self.position] != '"' {
                    self.position += 1;
                }
                if self.position >= self.input.len() {
                    return Err("Unterminated string literal".into());
                }
                let s: String = self.input[start_pos..self.position].iter().collect();
                self.position += 1;
                tokens.push(Token::StringLiteral(s));
                continue;
            }

            if current.is_alphabetic() {
                let start_pos = self.position;
                while self.position < self.input.len() 
                    && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') 
                {
                    self.position += 1;
                }
                let word: String = self.input[start_pos..self.position].iter().collect();
                match word.as_str() {
                    "start" => tokens.push(Token::Start),
                    "finish" => tokens.push(Token::Finish),
                    "print" => tokens.push(Token::Print),
                    "hash" => tokens.push(Token::Hash),
                    "as" => tokens.push(Token::As),
                    _ => tokens.push(Token::Identifier(word)),
                }
                continue;
            }

            if current == '=' {
                tokens.push(Token::Equals);
                self.position += 1;
                continue;
            }

            return Err(format!("Unexpected character: '{}'", current));
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
