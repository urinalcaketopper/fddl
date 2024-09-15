#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single-character tokens
    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    Comma, // ,
    Dot, // .
    Minus, // -
    Plus, // +
    Semicolon, // ;
    Colon, // :
    Slash, // /
    Star, // *
    Percent, // %
    Caret, // ^
    Tilde, // ~
    Backtick, // `
    Dollar, // $
    At, // @
    //                      Hash, // #
    Question, // ?
    Exclamation, // !
    Pipe, // |
    Ampersand, // &

    // one or two character tokens
    BangEqual, // !=
    Equal, // =
    EqualEqual, // ==
    Greater, // >
    GreaterEqual, // >=
    Less, // <
    LessEqual, // <=
    FatArrow, // =>
    ColonEqual, // :=
    TildeEqual, // ~=
    DoublePipe, // ||
    DoubleAmpersand, // &&

    // Literals
    Identifier(String),
    StringLiteral(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    Const,
    Define,
    Lambda,
    Match,
    Case,
    Switch,
    Until,
    Repeat,
    Unless,
    Yes,
    No,
    On,
    Off,
    Module,

    // Documentation and comments
    DocComment(String), // ##!, ###
    Comment(String), // #

    EOF,
}