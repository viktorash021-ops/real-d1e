use crate::ast::{Expr, Program, Stmt};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        match self.advance() {
            Token::Start => {}
            other => return Err(format!(
                "Syntax Error: Every .d1e program must begin with 'start'. Found '{:?}'",
                other
            )),
        }

        let mut statements = Vec::new();

        while self.peek() != &Token::Finish && self.peek() != &Token::Eof {
            statements.push(self.parse_statement()?);
        }

        match self.advance() {
            Token::Finish => {}
            Token::Eof => return Err("Syntax Error: Missing 'finish' keyword at the end of the program.".into()),
            other => return Err(format!(
                "Syntax Error: Expected 'finish' keyword, but found '{:?}'",
                other
            )),
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Print => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Stmt::Print(expr))
            }
            Token::Hash => {
                self.advance();
                let algo = match self.advance() {
                    Token::Identifier(a) => a,
                    other => return Err(format!("Expected hashing algorithm name, found '{:?}'", other)),
                };
                let input = self.parse_expr()?;
                
                match self.advance() {
                    Token::As => {}
                    other => return Err(format!("Expected 'as' keyword in hash statement, found '{:?}'", other)),
                }

                let out_var = match self.advance() {
                    Token::Identifier(v) => v,
                    other => return Err(format!("Expected variable name after 'as', found '{:?}'", other)),
                };

                Ok(Stmt::Hash { algo, input, out_var })
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                match self.advance() {
                    Token::Equals => {
                        let expr = self.parse_expr()?;
                        Ok(Stmt::Assign(name, expr))
                    }
                    other => Err(format!("Expected '=' after identifier, found '{:?}'", other)),
                }
            }
            other => Err(format!("Unexpected token in block statement: '{:?}'", other)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::StringLiteral(s) => Ok(Expr::StringLit(s)),
            Token::Identifier(id) => Ok(Expr::Identifier(id)),
            other => Err(format!("Expected expression, found '{:?}'", other)),
        }
    }
}
