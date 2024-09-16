use fddl::lexer::Lexer;
use fddl::lexer::token::Token;

#[test]
fn test_single_tokens() {
    let source = String::from("()+-*/;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::RightParen,
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_keywords_and_identifiers() {
    let source = String::from("sym myVar = 123;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Sym,
            Token::Identifier("myVar".to_string()),
            Token::Equal,
            Token::Number(123.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
    println!("{:?}", tokens);
}

#[test]
fn test_pub_keyword() {
    let source = String::from("pub func example() { return 42; }");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Pub,
            Token::Func,
            Token::Identifier("example".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(42.0),
            Token::Semicolon,
            Token::RightBrace,
            Token::EOF
        ]
    );
}

#[test]
fn test_comments() {
    let source = String::from("# This is a comment\nlet a = 5;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Comment(" This is a comment".to_string()),
            Token::Let,
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Number(5.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_operators_and_comparison() {
    let source = String::from("a >= 10 != b == 5;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Identifier("a".to_string()),
            Token::GreaterEqual,
            Token::Number(10.0),
            Token::BangEqual,
            Token::Identifier("b".to_string()),
            Token::EqualEqual,
            Token::Number(5.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_tilde_operator() {
    let source = String::from("if (a != b) { let c = ~5; }");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::BangEqual,
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Let,
            Token::Identifier("c".to_string()),
            Token::Equal,
            Token::Tilde,
            Token::Number(5.0),
            Token::Semicolon,
            Token::RightBrace,
            Token::EOF
        ]
    );
}