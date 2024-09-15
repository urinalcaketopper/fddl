use crate::lexer::token::Token;

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
                tokens.push(token);
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            '#' => self.handle_comment_or_doc(),
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
            '^' => Some(Token::Caret),  
            '~' => {
                if self.match_char('=') {
                    Some(Token::TildeEqual)
                } else {
                    Some(Token::Tilde)
                }
            },
            '`' => Some(Token::Backtick),
            '$' => Some(Token::Dollar),
            '@' => Some(Token::At),
            '?' => Some(Token::Question),
            '!' => {
                if self.match_char('=') {
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Exclamation)
                }
            },
            '|' => {
                if self.match_char('|') {
                    Some(Token::DoublePipe)
                } else {
                    Some(Token::Pipe)
                }
            },
            '&' => {
                if self.match_char('&') {
                    Some(Token::DoubleAmpersand)
                } else {
                    Some(Token::Ampersand)
                }
            },
            '=' => {
                if self.match_char('=') {
                    Some(Token::EqualEqual)
                } else if self.match_char('>') {
                    Some(Token::FatArrow)
                } else {
                    Some(Token::Equal)
                }
            },
            ':' => {
                if self.match_char('=') {
                    Some(Token::ColonEqual)
                } else {
                    // Handle single ':' if needed
                    Some(Token::Colon)
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
            '/' => {
                if self.match_char('/') {
                    // It's a comment, consume until end of line
                    let mut comment = String::new();
                    while self.peek() != '\n' && !self.is_at_end() {
                        comment.push(self.advance());
                    }
                    Some(Token::Comment(comment))
                } else {
                    Some(Token::Slash)
                }
                },
            ' ' | '\r' | '\t' => None, // Ignore whitespace
            '\n' => {
                self.line += 1;
                None
            },
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if self.is_alpha(c) => self.identifier(),
            _ => {
                eprintln!("Unexpected character '{}' on line {}", c, self.line);
                None
            }
        }
    }

    // Helper methods
    fn advance(&mut self) -> char {
        let c = if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        };
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

        // Check if we've reached the end without finding a closing quote
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
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "func" => Token::Func,
            "for" => Token::For,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "let" => Token::Let,
            "while" => Token::While,
            "const" => Token::Const,
            "define" => Token::Define,
            "lambda" => Token::Lambda,
            "match" => Token::Match,
            "case" => Token::Case,
            "switch" => Token::Switch,
            "until" => Token::Until,
            "repeat" => Token::Repeat,
            "unless" => Token::Unless,
            "yes" => Token::Yes,
            "no" => Token::No,
            "on" => Token::On,
            "off" => Token::Off,
            "module" => Token::Module,
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

    fn line_comment(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn block_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        }
    }

    fn handle_comment_or_doc(&mut self) -> Option<Token> {
        // We have matched one '#' character so far
        let mut count = 1;
    
        // Count additional consecutive '#' characters
        while self.match_char('#') {
            count += 1;
        }
    
        // Check for an exclamation mark after the '#' characters
        let has_exclamation = self.match_char('!');
    
        match (count, has_exclamation) {
            (1, _) => {
                // Single '#' - Line comment
                self.line_comment();
                None
            }
            (2, true) => {
                // '##!' - Module-level documentation comment
                self.doc_comment("module")
            }
            (2, false) => {
                // '##' - Block comment
                self.block_comment();
                None
            }
            (3, _) => {
                // '###' - Item-level documentation comment
                self.doc_comment("item")
            }
            (n, _) if n >= 4 => {
                // '####' or more - Block comment
                self.block_comment();
                None
            }
            _ => {
                // Fallback to line comment
                self.line_comment();
                None
            }
        }
    }
                     

    fn doc_comment(&mut self, _kind: &str) -> Option<Token> {
        let mut comment = String::new();
        while self.peek() != '\n' && !self.is_at_end() {
            comment.push(self.advance());
        }
    
        // Consume the newline character
        if self.peek() == '\n' {
            self.advance();
        }
    
        Some(Token::DocComment(comment.trim().to_string()))
    }
    
}
