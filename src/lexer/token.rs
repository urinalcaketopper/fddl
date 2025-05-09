#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single-character tokens
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Dot,          // .
    Minus,        // -
    Plus,         // +
    Semicolon,    // ;
    Slash,        // /
    Star,         // *
    Percent,      // %
    Equal,        // =
    BangEqual,    // !=
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=
    Tilde,        // ~
    TildeEqual,   // ~=

    // Literals
    Identifier(String),
    StringLiteral(String),
    Number(f64),

    // Keywords
    And,
    Or,
    If,
    Else,
    True,
    False,
    Let,
    Const,
    Func,
    Return,
    For,
    While,
    Print,
    Pub,
    Sym,
    Module,
    Import,
    Some,
    Not,

    // Comments
    Comment(String),

    // Errors
    Error(String),

    EOF,
}
