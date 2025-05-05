    use crate::lexer::token::Token;
    use crate::parser::ast::Expression;
    use crate::parser::ast::Statement;
    use crate::lexer::Lexer;

    pub struct Parser {
        tokens: Vec<Token>,
        current: usize,
    }

    #[allow(dead_code)]

    impl Parser {
        fn parse_statement(&mut self) -> Option<Statement> {
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
                None
            }
        }

        fn parse_variable_declaration(&mut self) -> Option<Statement> {
            let token = self.advance().clone();

            if let Token::Identifier(name) = token {
                let has_initializer = self.match_token(Token::Equal);
                
                let initializer = if has_initializer {
                    self.parse_expression()
                } else {
                    None
                };
        
                if self.match_token(Token::Semicolon) { 
                    Some(Statement::VariableDeclaration(name, initializer))
                } else {
                    None
                }
            } else {
                None
            }   
        }

        fn parse_if_statement(&mut self) -> Option<Statement> {
            self.match_token(Token::LeftParen);
            let condition = self.parse_expression()?;
            self.match_token(Token::RightParen);

            let then_branch = Box::new(self.parse_statement()?);
            let else_branch = if self.match_token(Token::Else) {
                Some(Box::new(self.parse_statement()?))
            } else {
                None
            };

            Some(Statement::IfStatement(condition, then_branch, else_branch))
        }

        fn parse_while_statement(&mut self) -> Option<Statement> {
            self.match_token(Token::LeftParen);
            let condition = self.parse_expression()?;
            self.match_token(Token::RightParen);

            let body = Box::new(self.parse_statement()?);

            Some(Statement::WhileStatement(condition, body))
        }

        fn parse_for_statement(&mut self) -> Option<Statement> {
            self.match_token(Token::LeftParen);

            let initializer = Box::new(self.parse_statement()?);
            let condition = self.parse_expression()?;
            self.match_token(Token::Semicolon);
            let increment = Box::new(self.parse_statement()?);

            self.match_token(Token::RightParen);
            let body = Box::new(self.parse_statement()?);

            Some(Statement::ForStatement(initializer, condition, increment, body))
        }

        fn parse_expression_statement(&mut self) -> Option<Statement> {
            let expr = self.parse_expression()?;    
            if self.match_token(Token::Semicolon) {
                Some(Statement::ExpressionStatement(expr))
            } else {
                None
            }
        }

        fn parse_expression(&mut self) -> Option<Expression> {
            // Placeholder for expression parsing logic
            None
        }

        pub fn new(tokens: Vec<Token>) -> Self {
            Parser {
                tokens,
                current: 0,
            }       
        }

        fn current_token(&self) -> &Token {
            &self.tokens[self.current]
        }

        fn peek(&self) -> &Token {
            &self.tokens[self.current + 1]
        }

        fn is_at_end(&self) -> bool {
            matches!(self.current_token(), Token::EOF)
        }

        fn advance(&mut self) -> &Token {
            if !self.is_at_end() {
                self.current += 1;
            }
            self.current_token()
        }

        fn match_token(&mut self, expected: Token) -> bool {
            if self.current_token() == &expected {
                self.advance();
                true
            } else {
                false
            }
        }

        fn previous_token(&self) -> &Token {
            &self.tokens[self.current - 1]
        }
    }

    fn main() {
        let source = String::from("let x = 10;");
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens();
        let mut parser = Parser::new(tokens);

        while !parser.is_at_end() {
            println!("{:?}", parser.current_token());
            parser.advance();
        }
    }
