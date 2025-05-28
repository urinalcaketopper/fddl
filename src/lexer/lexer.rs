use crate::lexer::token::Token;

#[allow(dead_code)]

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                if matches!(token, Token::Error(_)) {
                    tokens.push(token);
                    break;
                } else {
                    tokens.push(token);
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            // Single-character tokens
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),
            '%' => Some(Token::Percent),
            '~' => {
                if self.match_char('=') {
                    Some(Token::TildeEqual)
                } else {
                    Some(Token::Tilde)
                }
            },

            '/' => {
                if self.match_char('/') { // For line comments //
                    // Consume the rest of the line
                    let mut comment_text = String::new();
                    while self.peek() != '\n' && !self.is_at_end() {
                        comment_text.push(self.advance());
                    }
                    Some(Token::Comment(comment_text)) // Or None if you want to skip comments
                } else if self.match_char('*') { // For block comments /* ... */
                    self.consume_block_comment() // Call a new helper function
                } else {
                    Some(Token::Slash) // Division operator
                }
            },

            '#' => {
                let mut comment_text = String::new();
                while self.peek() != '\n' && !self.is_at_end() {
                    comment_text.push(self.advance());
                }
                Some(Token::Comment(comment_text))
            },

            // One or two character tokens
            '!' => {
                if self.match_char('=') {
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Error(format!("Unexpected character '{}'", c)))
                }
            },
            '=' => {
                if self.match_char('=') {
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            },
            '<' => {
                if self.match_char('=') {
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            },
            '>' => {
                if self.match_char('=') {
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            },

            // Whitespace
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            },

            // Literals
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if self.is_alpha(c) => self.identifier(),

            // Any other character
            _ => {
                eprintln!("Unexpected character '{}' on line {}", c, self.line);
                Some(Token::Error(format!("Unexpected character '{}'", c)))
            }
        }
    }

    fn consume_block_comment(&mut self) -> Option<Token> {
        let mut comment_text = String::new();
        let start_line = self.line;

        loop {
            if self.is_at_end() {
                eprintln!("Error (L{}): Unterminated block comment", start_line);
                if !comment_text.is_empty() {
                    return Some(Token::Comment(comment_text));
                }
                return None;
            }

            let current_char = self.peek();
            if current_char == '*' && self.peek_next() == '/' {
                self.advance(); // Consume '*'
                self.advance(); // Consume '/'
                return Some(Token::Comment(comment_text));
            } else {
                if current_char == '\n' {
                    self.line += 1;
                }
                comment_text.push(self.advance()); 

            }
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string on line {}", self.line);
            return None;
        }

        // Consume the closing quote
        self.advance();

        // Extract the string value
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        Some(Token::StringLiteral(value))
    }

    // Function to handle number literals
    fn number(&mut self) -> Option<Token> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value_str: String = self.source[self.start..self.current]
            .iter()
            .collect();
        let value = value_str.parse::<f64>().unwrap();
        Some(Token::Number(value))
    }

    // Function to handle identifiers and keywords
    fn identifier(&mut self) -> Option<Token> {
        while self.is_alphanumeric(self.peek()) || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current]
            .iter()
            .collect();

        // Check for reserved keywords
        let token = match text.as_str() {
            "and" => Token::And,
            "or" => Token::Or,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            "nil" => Token::Nil,
            "let" => Token::Let,
            "const" => Token::Const,
            "func" => Token::Func,
            "return" => Token::Return,
            "for" => Token::For,
            "while" => Token::While,
            "print" => Token::Print,
            "pub" => Token::Pub,
            "sym" => Token::Sym,
            "module" => Token::Module,
            "import" => Token::Import,
            "some" => Token::Some,
            "not" => Token::Not,
            _ => Token::Identifier(text),
        };

        Some(token)
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    // Function to handle line comments
    fn line_comment(&mut self) -> Option<Token>{
        let mut comment = String::new();

        while self.peek() != '\n' && !self.is_at_end() {
            comment.push(self.advance());
        }

        Some(Token::Comment(comment))
    }
}

