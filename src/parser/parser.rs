use crate::lexer::token::Token;
use crate::parser::ast::{Expression, Statement, Literal, Operator}; 
use crate::lexer::Lexer;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
         if !self.match_token(Token::LeftParen) {
              eprintln!("Error: Expected '(' after 'for'.");
              return None;
         }

         let initializer = if self.match_token(Token::Semicolon) {
            None 
         } else if self.match_token(Token::Let) {
             Some(Box::new(self.parse_variable_declaration()?))
         } else {
            let expr_stmt = self.parse_expression_statement()?;
            Some(Box::new(expr_stmt))
         };
         let condition = if self.check(&Token::Semicolon) {
             eprintln!("Error: For loop condition is required (for now).");
             return None;
         } else {
             self.parse_expression()?  
         };

         if !self.match_token(Token::Semicolon) {
             eprintln!("Error: Expected ';' after for loop condition.");
             return None;
         }

         let increment = if self.check(&Token::RightParen) {
             None
         } else {
             Some(self.parse_expression()?)  
         };

         if !self.match_token(Token::RightParen) {
             eprintln!("Error: Expected ')' after for loop clauses.");
             return None;
         }

         let body = Box::new(self.parse_statement()?);

         eprintln!("Warning: For statement AST structure might need review.");
         None

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

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if self.check(&Token::Print) {
            self.parse_print_statement()
        } else if self.check(&Token::Let) {
            self.parse_variable_declaration()
        } else if self.check(&Token::LeftBrace) {
            self.parse_block_statement()
        } else if self.check(&Token::If) {
            self.parse_if_statement()
        } else if self.check(&Token::While) {
            self.parse_while_statement()
        } else if self.check(&Token::For) {
            self.parse_for_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::LeftBrace) { 
            eprintln!("Error: Expected '{{' to start a block."); 
            return None;
        }

        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
                    return None;
                }
            }
        }

        if !self.match_token(Token::RightBrace) { 
            eprintln!("Error: Expected '}}' to close a block.");
            return None;
        }

        Some(Statement::Block(statements))
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::If) { return None; } 

        if !self.match_token(Token::LeftParen) {
             eprintln!("Error: Expected '(' after 'if'.");
             return None;
        }
        let condition = self.parse_expression()?;
        if !self.match_token(Token::RightParen) {
             eprintln!("Error: Expected ')' after if condition.");
             return None;
        }

        if !self.check(&Token::LeftBrace) {
            eprintln!("Error: Expected '{{' for if statement body.");
            return None;
        }
        let then_branch = Box::new(self.parse_statement()?);

        let mut else_branch = None;
        if self.match_token(Token::Else) {
            if !self.check(&Token::LeftBrace) && !self.check(&Token::If) {
                eprintln!("Error: Expected '{{' for else statement body or 'if' for 'else if'.");
                return None;
            }
            else_branch = Some(Box::new(self.parse_statement()?));
        }

        Some(Statement::IfStatement(condition, then_branch, else_branch))
    }

     fn parse_while_statement(&mut self) -> Option<Statement> {
         if !self.match_token(Token::While) { return None; }

         if !self.match_token(Token::LeftParen) {
              eprintln!("Error: Expected '(' after 'while'.");
              return None;
         }
         let condition = self.parse_expression()?;
         if !self.match_token(Token::RightParen) {
              eprintln!("Error: Expected ')' after while condition.");
              return None;
         }

        if !self.check(&Token::LeftBrace) {
            eprintln!("Error: Expected '{{' for while statement body.");
            return None;
        }
        let body = Box::new(self.parse_statement()?);

        Some(Statement::WhileStatement(condition, body))
     }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::Print) { return None; }
        let value = self.parse_expression()?;
        if self.match_token(Token::Semicolon) {
            Some(Statement::PrintStatement(value))
        } else {
            eprintln!("Error: Expected ';' after print value.");
            None
        }
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        if !self.match_token(Token::Let) { return None; }
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
            None
        }
    }

    fn check(&self, expected: &Token) -> bool {
         if self.is_at_end() {
             return false;
         }
         self.current_token() == expected
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn peek(&self) -> &Token {
         self.tokens.get(self.current + 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
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
            // Add others
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
        if self.current == 0 {
             self.tokens.get(0).unwrap_or(&Token::EOF)
        } else {
            &self.tokens[self.current - 1]
        }
    }

    // 5-7-25
    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end_of_significant_tokens() {
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
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

