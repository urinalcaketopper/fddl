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

#[test]
fn test_modulus_operator() {
    let source = String::from("10 % 3;");
    let mut lexer = Lexer::new(source.clone()); // Assuming Lexer::new takes String or &str
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Number(10.0),
            Token::Percent,
            Token::Number(3.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_nil_keyword() {
    let source = String::from("let a = nil;");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Nil,
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_some_keyword() {
    let source = String::from("if some value { }"); // Assuming 'value' is an identifier
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::Some,
            Token::Identifier("value".to_string()),
            Token::LeftBrace,
            Token::RightBrace,
            Token::EOF
        ]
    );
}

#[test]
fn test_not_keyword() {
    let source = String::from("not true");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Not,
            Token::True,
            Token::EOF
        ]
    );
}

#[test]
fn test_block_comments() {
    let source = String::from("/* this is a block comment */ let x = 1; /* another one */");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    // Assuming your lexer includes comments as Token::Comment in the stream
    // And that consume_block_comment() correctly extracts the inner text.
    // Adjust the expected comment string based on your lexer's behavior (e.g., if it includes spaces).
    assert_eq!(
        tokens,
        vec![
            Token::Comment(" this is a block comment ".to_string()), // Content might vary based on trimming
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Semicolon,
            Token::Comment(" another one ".to_string()), // Content might vary
            Token::EOF
        ]
    );
}

#[test]
fn test_multiline_block_comment() {
    let source = String::from("let y; /* hello\n world \n spanned */ print y;");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Comment(" hello\n world \n spanned ".to_string()), // Verify exact content including newlines
            Token::Print,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_empty_block_comment() {
    let source = String::from("/**/print 1;");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Comment("".to_string()), // Empty content
            Token::Print,
            Token::Number(1.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_block_comment_at_eof() {
    let source = String::from("let z = 10; /* block comment at eof"); // Unterminated
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    // This depends on how your lexer handles unterminated block comments.
    // The consume_block_comment function we discussed would try to form a comment
    // with what it has or an error.
    // If it forms a comment with the partial content:
    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("z".to_string()),
            Token::Equal,
            Token::Number(10.0),
            Token::Semicolon,
            Token::Comment(" block comment at eof".to_string()), // Or whatever your lexer emits for unterminated
            Token::EOF                                          // EOF is always last
        ]
        // Alternative if it produces an error token for unterminated comments:
        // vec![
        //     ...,
        //     Token::Semicolon,
        //     Token::Error("Unterminated block comment".to_string()), // Or similar
        //     Token::EOF
        // ]
    );
}

#[test]
fn test_combined_new_tokens() {
    let source = String::from("let result = some value % 2 == 0 and not nil;");
    let mut lexer = Lexer::new(source.clone());
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Equal,
            Token::Some,
            Token::Identifier("value".to_string()),
            Token::Percent,
            Token::Number(2.0),
            Token::EqualEqual,
            Token::Number(0.0),
            Token::And,
            Token::Not,
            Token::Nil,
            Token::Semicolon,
            Token::EOF
        ]
    );
}