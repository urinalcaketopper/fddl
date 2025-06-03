use crate::parser::ast::{Expression, Statement, Literal, Operator};
use std::collections::HashMap;

// --- Runtime Values ---
#[derive(Debug, Clone, PartialEq)]
pub enum FddlValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
    // Later, you might add: Function, Array, Object/Struct, etc.
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

// --- Runtime Errors ---
#[derive(Debug)]
pub enum RuntimeError {
    TypeMismatch(String),
    UndefinedVariable(String),
    DivisionByZero,
    // You could add more specific errors, e.g., IncorrectArgumentCount, etc.
}

// --- Environment for Variables ---
pub struct Environment {
    values: HashMap<String, FddlValue>,
    parent: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }
}

impl Environment {
    // Creates a new global/base environment
    pub fn new() -> Self {
        Environment::default()
    }

    // Creates a new environment that encloses a parent environment (for new scopes)
    pub fn new_enclosed(parent_environment: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(Box::new(parent_environment)),
        }
    }

    // Defines a new variable in the current scope. Allows shadowing.
    pub fn define(&mut self, name: String, value: FddlValue) {
        self.values.insert(name, value);
    }

    // Gets a variable's value, looking up through parent scopes if necessary.
    pub fn get(&self, name: &str) -> Result<FddlValue, RuntimeError> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()), // Clone to return an owned value
            None => { // Not found in current scope, try parent
                if let Some(parent_env_box) = &self.parent {
                    parent_env_box.get(name) // Recursive call
                } else {
                    Err(RuntimeError::UndefinedVariable(format!(
                        "Undefined variable '{}'.",
                        name
                    )))
                }
            }
        }
    }

    // Assigns a new value to an existing variable.
    // It must exist in the current or an enclosing scope.
    pub fn assign(&mut self, name: &str, value: FddlValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            // Variable exists in the current scope, assign here.
            self.values.insert(name.to_string(), value);
            Ok(())
        } else {
            // Not in current scope, try to assign in parent scope.
            if let Some(parent_env_box) = &mut self.parent {
                parent_env_box.assign(name, value) // Recursive call
            } else {
                Err(RuntimeError::UndefinedVariable(format!(
                    "Cannot assign to undefined variable '{}' (not found in any scope).",
                    name
                )))
            }
        }
    }
}

// --- Evaluator ---
pub struct Evaluator {
    environment: Environment,
}

// Helper for truthiness (nil and false are falsey, everything else is truthy)
// Defined as an associated function because it doesn't need `self`.
impl Evaluator {
    fn is_truthy(value: &FddlValue) -> bool {
        match value {
            FddlValue::Boolean(false) => false,
            FddlValue::Nil => false,
            _ => true,
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(), // Start with a global environment
        }
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
                self.evaluate_expression(expr)?; // Evaluate for side effects, discard result
            }
            Statement::VariableDeclaration(name, initializer) => {
                let value = match initializer {
                    Some(init_expr) => self.evaluate_expression(init_expr)?,
                    None => FddlValue::Nil, // Default to nil if no initializer
                };
                self.environment.define(name.clone(), value);
            }
            Statement::Assignment { target_name, value } => {
                let val_to_assign = self.evaluate_expression(value)?;
                self.environment.assign(target_name, val_to_assign)?;
            }
            Statement::Block(statements) => {
                // Create a new scope for the block
                let outer_environment = std::mem::take(&mut self.environment); // Takes current, leaves Default in self.env
                self.environment = Environment::new_enclosed(outer_environment); // New current env, old one is parent

                let mut block_execution_result: Result<(), RuntimeError> = Ok(());
                for stmt_in_block in statements {
                    block_execution_result = self.evaluate_statement(stmt_in_block);
                    if block_execution_result.is_err() {
                        break; // Stop executing statements in block if one errors
                    }
                }

                // Restore the outer environment
                if let Some(parent_env_box) = self.environment.parent.take() { // .take() to get ownership
                    self.environment = *parent_env_box; // Move parent back to be the current environment
                } else {
                    // This should ideally not happen if scopes are managed correctly.
                    eprintln!("Warning: Exited a block scope that had no parent environment. Resetting to global.");
                    self.environment = Environment::new(); // Fallback
                }
                return block_execution_result; // Return the result from the block (e.g., if an error occurred)
            }
            // TODO: Implement IfStatement, WhileStatement, ForStatement evaluation
            // TODO: Implement FunctionDeclaration (store function object in environment)
            // TODO: Implement ReturnStatement (special handling for unwinding and returning value)
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
                        Ok(FddlValue::Boolean(!Self::is_truthy(&right_val)))
                    }
                    Operator::Some => {
                        Ok(FddlValue::Boolean(!matches!(right_val, FddlValue::Nil)))
                    }
                    Operator::Almost => { // '~' operator
                        match right_val {
                            FddlValue::Number(n) => {
                                if n == 0.0 {
                                    Ok(FddlValue::Number(0.1337)) // Arbitrary small chaotic number
                                } else {
                                    let bits = n.to_bits();
                                    let offset_seed = (bits >> 16) & 0xFFF;
                                    let scale_seed = bits & 0xFFF;
                                    let chaotic_offset = (offset_seed as f64 / 4095.0 - 0.5) * n.abs() * 0.2;
                                    let chaotic_scale = 1.0 + (scale_seed as f64 / 4095.0 - 0.5) * 0.1;
                                    Ok(FddlValue::Number((n + chaotic_offset) * chaotic_scale))
                                }
                            }
                            FddlValue::Boolean(b) => Ok(FddlValue::Boolean(!b)),
                            FddlValue::String(s) => {
                                if s.is_empty() {
                                    Ok(FddlValue::String("?!~".to_string()))
                                } else {
                                    let mut new_s: String = s.chars().rev().collect();
                                    new_s.push('~');
                                    Ok(FddlValue::String(new_s))
                                }
                            }
                            FddlValue::Nil => Ok(FddlValue::String("almost nil?".to_string())),
                        }
                    }
                    // Add other unary operators if you have them in your Operator enum
                    _ => Err(RuntimeError::TypeMismatch(format!(
                        "Unsupported unary operator {:?}.",
                        op
                    ))),
                }
            },

            Expression::Binary(left_expr, op, right_expr) => {
                // Handle logical AND and OR first for short-circuiting
                match op {
                    Operator::And => {
                        let left_val = self.evaluate_expression(left_expr)?;
                        if !Self::is_truthy(&left_val) {
                            return Ok(FddlValue::Boolean(false)); // Short-circuit
                        }
                        let right_val = self.evaluate_expression(right_expr)?;
                        return Ok(FddlValue::Boolean(Self::is_truthy(&right_val)));
                    }
                    Operator::Or => {
                        let left_val = self.evaluate_expression(left_expr)?;
                        if Self::is_truthy(&left_val) {
                            return Ok(FddlValue::Boolean(true)); // Short-circuit
                        }
                        let right_val = self.evaluate_expression(right_expr)?;
                        return Ok(FddlValue::Boolean(Self::is_truthy(&right_val)));
                    }
                    _ => { // For all other binary operators, evaluate both operands first
                        let left_val = self.evaluate_expression(left_expr)?;
                        let right_val = self.evaluate_expression(right_expr)?;

                        match op {
                            // Arithmetic
                            Operator::Plus => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                                    Ok(FddlValue::Number(l + r))
                                } else { // TODO: String concatenation?
                                    Err(RuntimeError::TypeMismatch(format!("Operands for '+' must be numbers. Got {:?} and {:?}", left_val, right_val)))
                                }
                            }
                            Operator::Minus => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                                    Ok(FddlValue::Number(l - r))
                                } else {
                                    Err(RuntimeError::TypeMismatch(format!("Operands for '-' must be numbers. Got {:?} and {:?}", left_val, right_val)))
                                }
                            }
                            Operator::Multiply => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                                    Ok(FddlValue::Number(l * r))
                                } else {
                                    Err(RuntimeError::TypeMismatch(format!("Operands for '*' must be numbers. Got {:?} and {:?}", left_val, right_val)))
                                }
                            }
                            Operator::Divide => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                                    if *r == 0.0 { Err(RuntimeError::DivisionByZero) } else { Ok(FddlValue::Number(l / r)) }
                                } else {
                                    Err(RuntimeError::TypeMismatch(format!("Operands for '/' must be numbers. Got {:?} and {:?}", left_val, right_val)))
                                }
                            }
                            Operator::Modulus => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) {
                                    if *r == 0.0 { Err(RuntimeError::TypeMismatch("Modulus by zero.".to_string())) } else { Ok(FddlValue::Number(l % r)) }
                                } else {
                                    Err(RuntimeError::TypeMismatch(format!("Operands for '%' must be numbers. Got {:?} and {:?}", left_val, right_val)))
                                }
                            }

                            // Comparison
                            Operator::Greater => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) { Ok(FddlValue::Boolean(l > r)) } else { Err(RuntimeError::TypeMismatch("Operands for '>' must be numbers.".to_string())) }
                            }
                            Operator::GreaterEqual => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) { Ok(FddlValue::Boolean(l >= r)) } else { Err(RuntimeError::TypeMismatch("Operands for '>=' must be numbers.".to_string())) }
                            }
                            Operator::Less => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) { Ok(FddlValue::Boolean(l < r)) } else { Err(RuntimeError::TypeMismatch("Operands for '<' must be numbers.".to_string())) }
                            }
                            Operator::LessEqual => {
                                if let (FddlValue::Number(l), FddlValue::Number(r)) = (&left_val, &right_val) { Ok(FddlValue::Boolean(l <= r)) } else { Err(RuntimeError::TypeMismatch("Operands for '<=' must be numbers.".to_string())) }
                            }

                            // Equality
                            Operator::EqualEqual => Ok(FddlValue::Boolean(left_val == right_val)),
                            Operator::NotEqual => Ok(FddlValue::Boolean(left_val != right_val)),
                            
                            // And & Or are handled above due to short-circuiting.
                            // This _ should catch any other Operator variants not explicitly handled here.
                            _ => Err(RuntimeError::TypeMismatch(format!(
                                "Unsupported binary operator '{:?}' after operand evaluation.", op
                            ))),
                        }
                    }
                }
            },

            Expression::Grouping(inner_expr) => {
                self.evaluate_expression(inner_expr)
            },

            // TODO: Implement Expression::FunctionCall evaluation

            _ => { // Fallback for unimplemented expression types
                println!("Interpreter: Unimplemented expression: {:?}", expression);
                Err(RuntimeError::TypeMismatch(format!(
                    "Unimplemented expression type: {:?}",
                    expression
                )))
            }
        }
    }
}
