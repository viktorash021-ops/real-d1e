use crate::ast::{Expr, Program, Stmt};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

pub struct Interpreter {
    env: HashMap<String, String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: HashMap::new() }
    }

    pub fn eval(&mut self, program: Program) -> Result<(), String> {
        for stmt in program.statements {
            match stmt {
                Stmt::Print(expr) => {
                    let val = self.eval_expr(&expr)?;
                    println!("{}", val);
                }
                Stmt::Assign(name, expr) => {
                    let val = self.eval_expr(&expr)?;
                    self.env.insert(name, val);
                }
                Stmt::Hash { algo, input, out_var } => {
                    let val = self.eval_expr(&input)?;
                    let hashed_result = match algo.to_lowercase().as_str() {
                        "sha256" => {
                            let mut hasher = Sha256::new();
                            hasher.update(val.as_bytes());
                            hex::encode(hasher.finalize())
                        }
                        _ => return Err(format!("Unsupported hashing algorithm: '{}'", algo)),
                    };
                    self.env.insert(out_var, hashed_result);
                }
            }
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Result<String, String> {
        match expr {
            Expr::StringLit(s) => Ok(s.clone()),
            Expr::Identifier(id) => {
                self.env.get(id)
                    .cloned()
                    .ok_or_else(|| format!("Runtime Error: Undefined variable '{}'", id))
            }
        }
    }
}
