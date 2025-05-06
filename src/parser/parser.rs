// Add Literal to the use statement if it's not already implicitly included
use crate::lexer::token::Token;
use crate::parser::ast::{Expression, Statement, Literal}; // Added Literal here
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

    // --- Main Parsing Logic (Statements) ---

    fn parse_statement(&mut self) -> Option<Statement> {
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
        let value = self.parse_expression()?; // Needs implemented parse_expression
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
        let condition = self.parse_expression()?; // Needs implemented parse_expression
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
         let condition = self.parse_expression()?; // Needs implemented parse_expression
         if !self.match_token(Token::RightParen) {
              eprintln!("Error: Expected ')' after while condition.");
              return None;
         }

         let body = Box::new(self.parse_statement()?);

         Some(Statement::WhileStatement(condition, body))
     }

     // Note: For statement parsing is often complex. Let's simplify for now.
     // This implementation assumes a simple structure like `for (init; cond; incr) body`
     // It currently expects statements for init/incr which might not be ideal.
     fn parse_for_statement(&mut self) -> Option<Statement> {
         if !self.match_token(Token::LeftParen) {
              eprintln!("Error: Expected '(' after 'for'.");
              return None;
         }

         // Initializer: Could be variable declaration or expression statement
         let initializer = if self.match_token(Token::Semicolon) {
            None // No initializer
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
             self.parse_expression()? // Needs implemented parse_expression
         };

         if !self.match_token(Token::Semicolon) {
             eprintln!("Error: Expected ';' after for loop condition.");
             return None;
         }

         // Increment: Must be an expression (or None)
         let increment = if self.check(&Token::RightParen) {
             None // No increment expression
         } else {
             Some(self.parse_expression()?) // Needs implemented parse_expression
         };

         if !self.match_token(Token::RightParen) {
             eprintln!("Error: Expected ')' after for loop clauses.");
             return None;
         }

         // Body
         let body = Box::new(self.parse_statement()?);

        // Need to adjust AST for optional initializer/increment if using expressions directly
        // The current AST ForStatement expects Statements for init/incr
        // For simplicity now, let's assume the AST might need adjustment later
        // Or we wrap the initializer/increment expressions in ExpressionStatement
        // Let's stick to the original AST requiring Statements for now, meaning
        // parse_for_statement needs modification if increment isn't a full statement.
        // This is getting complicated - maybe defer full for-loop implementation?
        // Let's comment out the ForStatement return for now until AST/logic is clearer.

         eprintln!("Warning: For statement AST structure might need review.");
         // Some(Statement::ForStatement(initializer, condition, increment, body))
         None // Temporarily disable until AST/logic is solid for for-loops

     }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression()?; // Needs implemented parse_expression
        if self.match_token(Token::Semicolon) {
            Some(Statement::ExpressionStatement(expr))
        } else {
            // Error: Missing semicolon
            eprintln!("Error: Expected ';' after expression statement.");
            None
        }
    }

    // --- Expression Parsing ---

    // This is the main entry point for parsing any expression.
    // For now, it just calls parse_primary, but later it will
    // call the function for the lowest precedence level (e.g., assignment).
    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_primary() // Start with the simplest elements
        // Later: self.parse_assignment() or self.parse_equality() etc.
    }

    // parse_primary handles literals, identifiers, and grouping parentheses.
    fn parse_primary(&mut self) -> Option<Expression> {
        // Clone the token to match against it without consuming it yet
        let current_token = self.current_token().clone();

        match current_token {
            Token::Number(value) => {
                self.advance(); // Consume the token
                Some(Expression::Literal(Literal::Number(value)))
            }
            Token::StringLiteral(value) => {
                self.advance(); // Consume the token
                Some(Expression::Literal(Literal::String(value)))
            }
            Token::True => {
                self.advance(); // Consume the token
                Some(Expression::Literal(Literal::Boolean(true)))
            }
            Token::False => {
                self.advance(); // Consume the token
                Some(Expression::Literal(Literal::Boolean(false)))
            }
            Token::Identifier(name) => {
                self.advance(); // Consume the token
                Some(Expression::Variable(name))
            }
            Token::LeftParen => {
                self.advance(); // Consume '('
                // Recursively parse the expression inside the parentheses
                let expr = self.parse_expression()?; // Call the main expression parser

                // Expect and consume the closing parenthesis
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

    // Checks the current token without consuming it
    fn check(&self, expected: &Token) -> bool {
         if self.is_at_end() {
             return false;
         }
         self.current_token() == expected
    }

    // Returns the current token without consuming
    fn current_token(&self) -> &Token {
        // Handle potential index out of bounds if current somehow exceeds length
        self.tokens.get(self.current).unwrap_or(&Token::EOF) // Return EOF if out of bounds
    }

    // Returns the next token without consuming
    fn peek(&self) -> &Token {
         self.tokens.get(self.current + 1).unwrap_or(&Token::EOF)
    }

    fn is_at_end(&self) -> bool {
        // Check if current token is EOF
        matches!(self.current_token(), Token::EOF)
    }

    // Consumes the current token and returns it
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        // Return the token *before* the increment (the one just consumed)
        self.previous_token()
    }

    // Like advance, but clones the token for ownership if needed
    fn peek_and_advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            None
        } else {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            Some(token)
        }
    }


    // Consumes the current token *if* it matches the expected type
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
            self.advance(); // Consume the token if it matches
            true
        } else {
            false
        }
    }


    // Returns the token *before* the current one
    fn previous_token(&self) -> &Token {
        // Handle edge case where current is 0
        if self.current == 0 {
             self.tokens.get(0).unwrap_or(&Token::EOF)
        } else {
            &self.tokens[self.current - 1]
        }
    }
}