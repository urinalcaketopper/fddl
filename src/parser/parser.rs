use crate::lexer::token::Token;
use crate::parser::ast::{Expression, Statement, Literal, Operator}; 
// use crate::lexer::Lexer;

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

    fn parse_assignment_or_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?;

        self.skip_comments(); // Skip comments before semicolon

        if self.check(&Token::Equal) {
            self.advance();

            match expr {
                Expression::Variable(target_name) => {
                    self.skip_comments();
                    let value_expr = self.parse_expression()?;

                    self.skip_comments();
                    if !self.match_token(Token::Semicolon) {
                        eprintln!("Error: Expected ';' after assignment.");
                        return None;
                    }
                    Some(Statement::Assignment { target_name, value: value_expr })
                }
                _ => {
                    eprintln!("Error: Invalid assignment target. Must be an identifier.");
                    return None;
                }
            }
        } else if self.match_token(Token::Semicolon) {
            Some(Statement::ExpressionStatement(expr))
        } else {
            eprintln!("Error: Expected '=' for assignment or ';' after expression.");
            return None;
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        // Keeping old tests here for reference
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
            Token::Nil => {
                self.advance();
                Some(Expression::Literal(Literal::Nil))
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
        self.skip_comments();
        if self.is_at_end() { return None; }
        
        if self.check(&Token::Return) {
            self.parse_return_statement()
        } else if self.check(&Token::For) {
            self.parse_for_statement()
        } else if self.check(&Token::Func) {
            self.parse_function_declaration()
        } else if self.check(&Token::Print) {
            self.parse_print_statement()
        } else if self.check(&Token::Let) {
            self.parse_variable_declaration()
        } else if self.check(&Token::LeftBrace) {
            self.parse_block_statement()
        } else if self.check(&Token::If) {
            self.parse_if_statement()
        } else if self.check(&Token::While) {
            self.parse_while_statement()
        } else {
            self.parse_assignment_or_expression_statement()
        }
    }

        fn parse_function_declaration(&mut self) -> Option<Statement> {
        if !self.match_token(Token::Func) { 
            eprintln!("Internal parser error: Expected 'func' token in parse_function_declaration.");
            return None;
        }

        let name = match self.peek_and_advance() { 
            Some(Token::Identifier(name_str)) => name_str,
            _ => {
                eprintln!("Error: Expected function name (identifier) after 'func'.");
                return None;
            }
        };

        if !self.match_token(Token::LeftParen) {
            eprintln!("Error: Expected '(' after function name '{}'.", name);
            return None;
        }

        let params = self.parse_parameters()?; 

        if !self.match_token(Token::RightParen) {
            eprintln!("Error: Expected ')' after function parameters for function '{}'.", name);
            return None;
        }

        if !self.check(&Token::LeftBrace) {
            eprintln!("Error: Expected '{{' for function body of '{}'.", name);
            return None;
        }

        let body_statement = self.parse_statement()?;

        match body_statement {
            Statement::Block(body_statements) => {
                Some(Statement::FunctionDeclaration { name, params, body: body_statements })
            }
            _ => {
                eprintln!("Error: Function body must be a block statement for function '{}'.", name);
                None
            }
        }
    }

    fn parse_parameters(&mut self) -> Option<Vec<String>> {
                let mut parameters = Vec::new();

        if self.check(&Token::RightParen) {
            return Some(parameters);
        }

        match self.peek_and_advance() {
            Some(Token::Identifier(param_name)) => parameters.push(param_name),
            _ => {
                eprintln!("Error: Expected parameter name (identifier) in function parameter list.");
                return None;
            }
        }

        while self.match_token(Token::Comma) {
            match self.peek_and_advance() {
                Some(Token::Identifier(param_name)) => parameters.push(param_name),
                _ => {
                    eprintln!("Error: Expected parameter name (identifier) after comma in function parameter list.");
                    return None;
                }
            }
        }
        Some(parameters)
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
        if !self.match_token(Token::If) {
            eprintln!("Internal parser error: Expected 'if' token in parse_if_statement.");
            return None;
        }

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

        let mut else_branch_opt: Option<Box<Statement>> = None;
        if self.match_token(Token::Else) {
            if self.check(&Token::LeftBrace) {
                else_branch_opt = Some(Box::new(self.parse_statement()?));
            } else {
                eprintln!("Error: Expected '{{' or 'if' after 'else'.");
                return None;
            }
        }

        Some(Statement::IfStatement(condition, then_branch, else_branch_opt))
    }

        fn parse_while_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::While) { 
            eprintln!("Internal parser error: Expected 'while' token in parse_while_statement.");
            return None;
        }

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

    fn parse_for_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::For) { return None; } // Consume 'for'

        if !self.match_token(Token::LeftParen) {
            eprintln!("Error: Expected '(' after 'for'.");
            return None;
        }

        // 1. Initializer Statement
        self.skip_comments(); // Skip comments before initializer part
        let initializer: Box<Statement>;
        if self.check(&Token::Let) {
            self.advance(); // Consume 'let'
            let var_name = match self.peek_and_advance() {
                Some(Token::Identifier(name_str)) => name_str,
                _ => {
                    eprintln!("Error: Expected variable name after 'let' in for-loop initializer.");
                    return None;
                }
            };
            let var_initializer_expr: Option<Expression> = if self.match_token(Token::Equal) {
                self.skip_comments(); // Skip comments before the expression value
                self.parse_expression()
            } else {
                None
            };
            initializer = Box::new(Statement::VariableDeclaration(var_name, var_initializer_expr));
            self.skip_comments(); // Skip comments before the semicolon separator
            if !self.match_token(Token::Semicolon) {
                eprintln!("Error: Expected ';' after 'let' declaration in for-loop initializer.");
                return None;
            }
        } else if self.check(&Token::Semicolon) { // Check for ';' for empty initializer
            self.advance(); // Consume the ';'
            initializer = Box::new(Statement::ExpressionStatement(Expression::Literal(Literal::Nil)));
        } else { // Expression initializer
            let init_expr = self.parse_expression()?;
            initializer = Box::new(Statement::ExpressionStatement(init_expr));
            self.skip_comments(); // Skip comments before the semicolon separator
            if !self.match_token(Token::Semicolon) {
                eprintln!("Error: Expected ';' after for-loop initializer expression.");
                return None;
            }
        }

        // 2. Condition Expression
        self.skip_comments(); // Skip comments before condition part
        let condition: Expression;
        if self.check(&Token::Semicolon) { // Check for ';' for empty condition
            self.advance(); // Consume the ';'
            condition = Expression::Literal(Literal::Boolean(true)); // Default to true
        } else {
            condition = self.parse_expression()?;
            self.skip_comments(); // Skip comments before the semicolon separator
            if !self.match_token(Token::Semicolon) {
                eprintln!("Error: Expected ';' after for-loop condition.");
                return None;
            }
        }

        // 3. Increment Statement
        self.skip_comments(); // <--- CRUCIAL FIX: Skip comments before increment part
        let increment: Box<Statement> = if self.check(&Token::RightParen) {
            Box::new(Statement::ExpressionStatement(Expression::Literal(Literal::Nil)))
        } else {
            let incr_expr = self.parse_expression()?;
            Box::new(Statement::ExpressionStatement(incr_expr))
        };

        self.skip_comments(); // Skip comments before ')'
        if !self.match_token(Token::RightParen) {
            eprintln!("Error: Expected ')' after for-loop clauses.");
            return None;
        }

        // 4. Body Statement (must be a block)
        self.skip_comments(); // Skip comments before '{'
        if !self.check(&Token::LeftBrace) {
            eprintln!("Error: Expected '{{' for for-loop body.");
            return None;
        }
        // parse_statement() already calls skip_comments() at its beginning
        let body = Box::new(self.parse_statement()?);

        Some(Statement::ForStatement(initializer, condition, increment, body))
    }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::Print) { return None; } 
        let value = self.parse_expression()?;
        if !self.match_token(Token::Semicolon) {
            eprintln!("Error: Expected ';' after print value.");
            None
        } else {
            Some(Statement::PrintStatement(value))
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

            if !self.match_token(Token::Semicolon) {
                eprintln!("Error: Expected ';' after variable declaration.");
                None
            } else {
                Some(Statement::VariableDeclaration(name, initializer))
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

    fn skip_comments(&mut self) {
        while !self.is_at_end() && matches!(self.current_token(), Token::Comment(_)) {
            self.advance();
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

    //5-28/25
    fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.match_token(Token::Return) {
            eprintln!("Internal parser error: Expected 'return' token.");
            return None;
        }

        self.skip_comments(); // Skip comments before the expression

        if self.check(&Token::Semicolon) {
            self.advance();
            return Some(Statement::ReturnStatement(None));
        }

        if self.is_at_end() {
            eprintln!("Error: Expected expression or ';' after 'return'.");
            return None;
        }

        match self.parse_expression() {
            Some(expr) => {
                self.skip_comments(); // Skip comments after the expression
                if !self.match_token(Token::Semicolon) {
                    eprintln!("Error: Expected ';' after return expression.");
                    return None;
                }
                Some(Statement::ReturnStatement(Some(expr)))
            }
            None => {
                eprintln!("Error: Invalid expression after 'return'.");
                None
            }
        }
    }
    
}

