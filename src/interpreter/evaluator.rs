use crate::parser::ast::{Expression, Statement, Literal, Operator};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FddlValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl std::fmt::Display for FddlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FddlValue::Number(n) => write!(f, "{}", n),
            FddlValue::Boolean(b) => write!(f, "{}", b),
            FddlValue::String(s) => write!(f, "{}", s),
            FddlValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    TypeMismatch(String),
    UndefinedVariable(String),
    DivisionByZero,
}

pub struct Environment {
    values: HashMap<String, FddlValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: FddlValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<FddlValue, RuntimeError> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()), 
            None => Err(RuntimeError::UndefinedVariable(format!(
                "Undefined variable '{}'.",
                name
            ))),
        }
    }

    pub fn assign(&mut self, name: &str, value: FddlValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedVariable(format!(
                "Cannot assign to undefined variable '{}'.",
                name
            )))
        }
    }
}

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub fn evaluate_program(&mut self, statements: Vec<Statement>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.evaluate_statement(&statement)?;
        }
        Ok(())
    }

    fn is_truthy(value: &FddlValue) -> bool {
        match value {
            FddlValue::Boolean(false) => false,
            FddlValue::Nil => false,
            _ => true,
        }
    }

    fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), RuntimeError> {
        match statement {
            Statement::PrintStatement(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
            }

            Statement::ExpressionStatement(expr) => {
                self.evaluate_expression(expr)?; 
            }

            Statement::VariableDeclaration(name, initializer) => {
                let value = match initializer {
                    Some(init_expr) => self.evaluate_expression(init_expr)?,
                    None => FddlValue::Nil,
                };
                self.environment.define(name.clone(), value);
            }

            Statement::Assignment { target_name, value } => {
                let val_to_assign = self.evaluate_expression(value)?;
                self.environment.assign(target_name, val_to_assign)?;
            }

            Statement::Block(statements) => {
                for stmt_in_block in statements {
                    self.evaluate_statement(stmt_in_block)?;
                }
            }

            // TODO: IfStatement, WhileStatement, ForStatement, FunctionDeclaration, ReturnStatement

            _ => {
                println!("Interpreter: Skipping unimplemented statement: {:?}", statement);
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Result<FddlValue, RuntimeError> {
        match expression {
            Expression::Literal(literal) => {
                match literal {
                    Literal::Number(n) => Ok(FddlValue::Number(*n)),
                    Literal::Boolean(b) => Ok(FddlValue::Boolean(*b)),
                    Literal::String(s) => Ok(FddlValue::String(s.clone())),
                    Literal::Nil => Ok(FddlValue::Nil),
                }
            }, 

            Expression::Variable(name) => {
                self.environment.get(name)
            }, 

            Expression::Unary(op, right_expr) => { 
                let right_val = self.evaluate_expression(right_expr)?;
                match op {
                    Operator::Minus => {
                        if let FddlValue::Number(n) = right_val {
                            Ok(FddlValue::Number(-n))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operand for unary '-' must be a number.".to_string(),
                            ))
                        }
                    }

                    Operator::Not => { 
                        if let FddlValue::Boolean(b) = right_val {
                            Ok(FddlValue::Boolean(!b))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operand for 'not' must be a boolean.".to_string(),
                            ))
                        }
                    }

                    Operator::Some => { 
                        Ok(FddlValue::Boolean(!matches!(right_val, FddlValue::Nil))) 
                    }

                    Operator::Almost => {
                        if let FddlValue::Number(n) = right_val {
                            Ok(FddlValue::Number(n.floor())) 
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operand for unary '~' (Almost) must be a number for this example.".to_string(),
                            ))
                        }
                    }
                    _ => Err(RuntimeError::TypeMismatch(format!(
                        "Unsupported unary operator {:?}.",
                        op
                    ))),
                }
            }, 

            Expression::Binary(left_expr, op, right_expr) => {
                let left_val = self.evaluate_expression(left_expr)?;
                let right_val = self.evaluate_expression(right_expr)?;

                match op {
                    Operator::EqualEqual => { // For ==
                        Ok(FddlValue::Boolean(left_val == right_val))
                    }
                    Operator::NotEqual => { // For !=
                        Ok(FddlValue::Boolean(left_val != right_val))
                    }
                    Operator::Plus => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Number(l + r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '+' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Minus => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Number(l - r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '-' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Multiply => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Number(l * r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '*' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Divide => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            if *r == 0.0 {
                                Err(RuntimeError::DivisionByZero) 
                            } else {
                                Ok(FddlValue::Number(l / r))
                            }
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '/' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Modulus => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            if *r == 0.0 {
                                Err(RuntimeError::TypeMismatch("Modulus by zero.".to_string())) // Or DivisionByZero
                            } else {
                                Ok(FddlValue::Number(l % r))
                            }
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '%' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Greater => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Boolean(l > r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operands for '>' must be numbers.".to_string()
                            ))
                        }
                    }
                    Operator::GreaterEqual => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Boolean(l >= r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operands for '>=' must be numbers.".to_string()
                            ))
                        }
                    }
                    Operator::Less => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Boolean(l < r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operands for '<' must be numbers.".to_string()
                            ))
                        }
                    }
                    Operator::LessEqual => {
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Boolean(l <= r))
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                "Operands for '<=' must be numbers.".to_string()
                            ))
                        }
                    }
                    _ => Err(RuntimeError::TypeMismatch(format!(
                        "Unsupported binary operator {:?}.",
                        op
                    ))),
                }
            },

            Expression::Grouping(inner_expr) => { 
                self.evaluate_expression(inner_expr)
            },

            _ => {
                println!("Interpreter: Unimplemented expression: {:?}", expression);
                Err(RuntimeError::TypeMismatch(format!(
                    "Unimplemented expression type: {:?}",
                    expression
                )))
            }
        }
    }

}