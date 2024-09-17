use crate::lexer::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_token(Token::Print) {
            self.parse_print_statement()
        }
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