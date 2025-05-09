// Add Literal to the use statement if it's not already implicitly included
use crate::lexer::token::Token;
use crate::parser::ast::{Expression, Statement, Literal, Operator}; // Added Literal here
use crate::lexer::Lexer;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// mostly all written months ago
#[allow(dead_code)]
impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        // ... (existing statement parsing code) ...
        // Make sure these call the updated parse_expression eventually
        if self.match_token(Token::Print) {
            self.parse_print_statement()
        } else if self.match_token(Token::Let) {
            self.parse_variable_declaration()
        } else if self.match_token(Token::If) {
            self.parse_if_statement()
        } else if self.match_token(Token::While) {
            self.parse_while_statement()
        } else if self.match_token(Token::For) {
            self.parse_for_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        let value = self.parse_expression()?;  
        if self.match_token(Token::Semicolon) {
            Some(Statement::PrintStatement(value))
        } else {
            // Error: Missing semicolon
            eprintln!("Error: Expected ';' after print value.");
            None
        }
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        // Assuming 'let' token is already consumed by match_token
        // peek_and_advance() returns Option<Token>, which is what we want.
        let token_option = self.peek_and_advance();

        if let Some(Token::Identifier(name)) = token_option {
            let initializer = if self.match_token(Token::Equal) {
                self.parse_expression()
            } else {
                None
            };

            if self.match_token(Token::Semicolon) {
                Some(Statement::VariableDeclaration(name, initializer))
            } else {
                eprintln!("Error: Expected ';' after variable declaration.");
                None
            }
        } else {
             eprintln!("Error: Expected variable name after 'let'.");
            // If token_option was None or not an Identifier, this branch is taken.
            None
        }
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::LeftParen) {
             eprintln!("Error: Expected '(' after 'if'.");
             return None;
        }
        let condition = self.parse_expression()?;  
        if !self.match_token(Token::RightParen) {
             eprintln!("Error: Expected ')' after if condition.");
             return None;
        }

        // Assuming parse_statement handles block parsing via LeftBrace eventually
        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_token(Token::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Some(Statement::IfStatement(condition, then_branch, else_branch))
    }

     fn parse_while_statement(&mut self) -> Option<Statement> {
         if !self.match_token(Token::LeftParen) {
              eprintln!("Error: Expected '(' after 'while'.");
              return None;
         }
         let condition = self.parse_expression()?;  
         if !self.match_token(Token::RightParen) {
              eprintln!("Error: Expected ')' after while condition.");
              return None;
         }

         let body = Box::new(self.parse_statement()?);

         Some(Statement::WhileStatement(condition, body))
     }

     fn parse_for_statement(&mut self) -> Option<Statement> {
         if !self.match_token(Token::LeftParen) {
              eprintln!("Error: Expected '(' after 'for'.");
              return None;
         }

         // Initializer: Could be variable declaration or expression statement
         let initializer = if self.match_token(Token::Semicolon) {
            None 
         } else if self.match_token(Token::Let) {
             // Need to handle declaration specifically if wanted, or parse as statement
             Some(Box::new(self.parse_variable_declaration()?)) // Assuming Let was consumed
         } else {
            // Parse as expression statement, then expect semicolon
            let expr_stmt = self.parse_expression_statement()?;
            Some(Box::new(expr_stmt))
         };
         // Semicolon already consumed if initializer was None or handled by expr_stmt

         // Condition: Must be an expression
         let condition = if self.check(&Token::Semicolon) {
             // No condition (treat as true) - maybe represent with a literal true?
             // For now, let's require a condition or handle absence later
             eprintln!("Error: For loop condition is required (for now).");
             return None;
             // TODO: Handle absent condition -> treat as true
             // Some(Expression::Literal(Literal::Boolean(true)))
         } else {
             self.parse_expression()?  
         };

         if !self.match_token(Token::Semicolon) {
             eprintln!("Error: Expected ';' after for loop condition.");
             return None;
         }

         // Increment: Must be an expression (or None)
         let increment = if self.check(&Token::RightParen) {
             None // No increment expression
         } else {
             Some(self.parse_expression()?)  
         };

         if !self.match_token(Token::RightParen) {
             eprintln!("Error: Expected ')' after for loop clauses.");
             return None;
         }

         // Body
         let body = Box::new(self.parse_statement()?);

         eprintln!("Warning: For statement AST structure might need review.");
         // Some(Statement::ForStatement(initializer, condition, increment, body))
         None // Temporarily disable until AST/logic is solid for for-loops

     }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?;  
        if self.match_token(Token::Semicolon) {
            Some(Statement::ExpressionStatement(expr))
        } else {
            eprintln!("Error: Expected ';' after expression statement.");
            None
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        // Keeping old tests here for reference until release.
        // self.parse_primary(); // Start with the simplest elements
        // self.parse_unary() // handles unary operators ('-' and '~')
        // self.parse_term() // handles binary operators ('+', '-', '*', '/')
        // self.parse_comparison() // handles comparison operators ('<', '>', '<=', '>=')
        // self.parse_equality() // handles equality operators ('==', '!=')
        self.parse_logical_or() // handles logical operators ('&&', '||')
    }

    // Each function below is fed into the function below it
    fn parse_primary(&mut self) -> Option<Expression> {
        let current_token = self.current_token().clone();

        match current_token {
            Token::Number(value) => {
                self.advance();   
                Some(Expression::Literal(Literal::Number(value)))
            }
            Token::StringLiteral(value) => {
                self.advance();   
                Some(Expression::Literal(Literal::String(value)))
            }
            Token::True => {
                self.advance();   
                Some(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance();   
                Some(Expression::Literal(Literal::Boolean(false)))
            }
            Token::Identifier(name) => {
                self.advance();   
                Some(Expression::Variable(name))
            }
            Token::LeftParen => {
                self.advance(); 
                let expr = self.parse_expression()?; 

                if self.match_token(Token::RightParen) {
                     // Return the inner expression, wrapped in Grouping AST node
                    Some(Expression::Grouping(Box::new(expr)))
                } else {
                    eprintln!("Error: Expected ')' after expression in parentheses.");
                    None // Error: Missing closing parenthesis
                }
            }
            // Add cases for other primary expressions like 'nil' if you add it
            _ => {
                // Error: Unexpected token when expecting a primary expression
                eprintln!("Error: Unexpected token '{:?}' while parsing primary expression.", self.current_token());
                None
            }
        }
    }

    fn parse_unary(&mut self) -> Option<Expression> {
        let operator_token_snapshot = self.current_token().clone();

        match operator_token_snapshot {
            Token::Minus | Token::Tilde | Token::Some | Token::Not => {
                self.advance();
                let ast_operator = match operator_token_snapshot {
                    Token::Minus => Operator::Minus,
                    Token::Tilde => Operator::Almost,
                    Token::Some => Operator::Some,
                    Token::Not => Operator::Not,
                    _ => unreachable!("Lexer should not produce other tokens here if first match is minus/tilde. Checked by matches! macro."),
                };
                
                let right_operand = self.parse_unary()?;
                Some(Expression::Unary(ast_operator, Box::new(right_operand)))
            }
            _ => {
                self.parse_call_expression()
            }
        }
    }

    fn parse_term(&mut self) -> Option<Expression> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator_token = self.current_token().clone();
            self.advance();

            let ast_operator = match operator_token {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!("Lexer should not produce other tokens here if first match is plus/minus. Checked by matches! macro."),
            };

            let right_operand = self.parse_factor()?;
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let mut expr = self.parse_unary()?;

        while matches!(self.current_token(), Token::Star | Token::Slash | Token::Percent) {
            let operator_token = self.current_token().clone();
            self.advance();

            let ast_operator = match operator_token {
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::Percent => Operator::Modulus,
                _ => unreachable!("Lexer should not produce other tokens here if first match is star/slash. Checked by matches! macro."),
            };

            let right_operand = self.parse_unary()?;
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.current_token(),
            Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual
        ) {
            let operator_token = self.current_token().clone();
            self.advance(); 

            let ast_operator = match operator_token {
                Token::Greater => Operator::Greater,
                Token::GreaterEqual => Operator::GreaterEqual,
                Token::Less => Operator::Less,
                Token::LessEqual => Operator::LessEqual,
                _ => unreachable!("Checked by matches! macro"),
            };

            let right_operand = self.parse_term()?; 
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_equality(&mut self) -> Option<Expression> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current_token(), Token::EqualEqual | Token::BangEqual) {
            let operator_token = self.current_token().clone();
            self.advance(); 

            let ast_operator = match operator_token {
                Token::EqualEqual => Operator::EqualEqual,
                Token::BangEqual => Operator::NotEqual,
                _ => unreachable!("Checked by matches! macro"),
            };

            let right_operand = self.parse_comparison()?; 
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_logical_or(&mut self) -> Option<Expression> {
        let mut expr = self.parse_logical_and()?;

        while matches!(self.current_token(), Token::Or) {
            let operator_token = self.current_token().clone();
            self.advance();

            let ast_operator = Operator::Or;

            let right_operand = self.parse_logical_and()?;
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_logical_and(&mut self) -> Option<Expression> {
        let mut expr = self.parse_equality()?;

        while matches!(self.current_token(), Token::And) {
            let operator_token = self.current_token().clone();
            self.advance();

            let ast_operator = Operator::And;

            let right_operand = self.parse_equality()?;
            expr = Expression::Binary(Box::new(expr), ast_operator, Box::new(right_operand));
        }
        Some(expr)
    }

    fn parse_call_expression(&mut self) -> Option<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Some(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Option<Expression> {
        self.advance();

        let arguments = self.parse_arguments()?;

        if !self.match_token(Token::RightParen) {
            eprintln!("Error: Expected ')' after arguments in function call.");
            return None;
        }
        Some(Expression::FunctionCall(Box::new(callee), arguments))
    }

    fn parse_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut arguments = Vec::new();

        if self.check(&Token::RightParen) {
            return Some(arguments);
        }

        match self.parse_expression() {
            Some(arg) => arguments.push(arg),
            None => return None,
        }

        while self.match_token(Token::Comma) {
            match self.parse_expression() {
                Some(arg) => arguments.push(arg),
                None => return None,
            }
        }
        Some(arguments)
    }

    fn check(&self, expected: &Token) -> bool {
         if self.is_at_end() {
             return false;
         }
         self.current_token() == expected
    }

    fn current_token(&self) -> &Token {
        // Handle potential index out of bounds if current somehow exceeds length
        self.tokens.get(self.current).unwrap_or(&Token::EOF) // Return EOF if out of bounds
    }

    fn peek(&self) -> &Token {
         self.tokens.get(self.current + 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        // Return the token *before* the increment (the one just consumed)
        self.previous_token()
    }

    fn peek_and_advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        let current_matches = match (self.current_token(), &expected) {
            (Token::LeftParen, Token::LeftParen) => true,
            (Token::RightParen, Token::RightParen) => true,
            (Token::LeftBrace, Token::LeftBrace) => true,
            (Token::RightBrace, Token::RightBrace) => true,
            (Token::Comma, Token::Comma) => true,
            (Token::Dot, Token::Dot) => true,
            (Token::Minus, Token::Minus) => true,
            (Token::Plus, Token::Plus) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::Slash, Token::Slash) => true,
            (Token::Star, Token::Star) => true,
            (Token::Equal, Token::Equal) => true,
            (Token::BangEqual, Token::BangEqual) => true,
            (Token::EqualEqual, Token::EqualEqual) => true,
            (Token::Greater, Token::Greater) => true,
            (Token::GreaterEqual, Token::GreaterEqual) => true,
            (Token::Less, Token::Less) => true,
            (Token::LessEqual, Token::LessEqual) => true,
            (Token::Tilde, Token::Tilde) => true,
            (Token::TildeEqual, Token::TildeEqual) => true,
            (Token::And, Token::And) => true,
            (Token::Or, Token::Or) => true,
            (Token::If, Token::If) => true,
            (Token::Else, Token::Else) => true,
            (Token::True, Token::True) => true,
            (Token::False, Token::False) => true,
            (Token::Let, Token::Let) => true,
            (Token::Const, Token::Const) => true,
            (Token::Func, Token::Func) => true,
            (Token::Return, Token::Return) => true,
            (Token::For, Token::For) => true,
            (Token::While, Token::While) => true,
            (Token::Print, Token::Print) => true,
            (Token::Pub, Token::Pub) => true,
            (Token::Sym, Token::Sym) => true,
            (Token::Module, Token::Module) => true,
            (Token::Import, Token::Import) => true,
            (Token::EOF, Token::EOF) => true,
            // Add other simple tokens here...
             (t1, t2) => std::mem::discriminant(t1) == std::mem::discriminant(t2)
            };

        if current_matches {
            self.advance(); 
            true
        } else {
            false
        }
    }

    fn previous_token(&self) -> &Token {
        // Handle edge case where current is 0
        if self.current == 0 {
             self.tokens.get(0).unwrap_or(&Token::EOF)
        } else {
            &self.tokens[self.current - 1]
        }
    }

    // 5-7-25
    // parse_program in Parser: a cleaner way to handle parsing multiple statements
    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end_of_significant_tokens() {
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
                    // Handle error: could not parse statement
                    eprintln!("Error: Could not parse statement.");
                    break;
                }
            }
        }
        statements
    }

    fn is_at_end_of_significant_tokens(&self) -> bool {
        if self.current >= self.tokens.len() { return true; }
        matches!(self.tokens[self.current], Token::EOF)
    }

    pub fn is_at_end(&self) -> bool {
        if self.current >= self.tokens.len() { return true; }
        matches!(self.tokens[self.current], Token::EOF)
    }
    
}

