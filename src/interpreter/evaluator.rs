use crate::parser::ast::{Expression, Statement, Literal, Operator};

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
}

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Evaluator
    }

    pub fn evaluate_program(&mut self, statements: Vec<Statement>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.evaluate_statement(&statement)?;
        }
        Ok(())
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
            }

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
                        Ok(right_val) 
                    }
                    Operator::Almost => {
                        if let FddlValue::Number(n) = right_val {
                            Ok(FddlValue::Number(n.round())) 
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
            }

            Expression::Binary(left_expr, op, right_expr) => { // Add this new arm
                let left_val = self.evaluate_expression(left_expr)?;
                let right_val = self.evaluate_expression(right_expr)?;

                // Now, perform the operation based on 'op' and the types of left_val and right_val
                match op {
                    Operator::Plus => {
                        // Example for addition (assuming numbers for now)
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            Ok(FddlValue::Number(l + r))
                        } else {
                            // Later, you might allow string concatenation here
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '+' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Minus => { // Binary Minus
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
                                Err(RuntimeError::TypeMismatch("Division by zero.".to_string())) // Or a specific DivisionByZero error
                            } else {
                                Ok(FddlValue::Number(l / r))
                            }
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '/' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    Operator::Modulus => { // Assuming you have Operator::Modulus from earlier
                        if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                            if *r == 0.0 {
                                Err(RuntimeError::TypeMismatch("Modulus by zero.".to_string()))
                            } else {
                                Ok(FddlValue::Number(l % r))
                            }
                        } else {
                            Err(RuntimeError::TypeMismatch(
                                format!("Operands for '%' must be numbers. Got {:?} and {:?}", left_val, right_val)
                            ))
                        }
                    }
                    // TODO: Add cases for Operator::Greater, Less, EqualEqual, NotEqual, And, Or etc.
                    // These will typically operate on numbers or booleans and produce FddlValue::Boolean.
                    _ => Err(RuntimeError::TypeMismatch(format!(
                        "Unsupported binary operator {:?}.",
                        op
                    ))),
                }
            }

            Expression::Grouping(inner_expr) => {
                self.evaluate_expression(inner_expr)
            }

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