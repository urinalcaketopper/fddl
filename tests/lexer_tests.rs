use fiddle::lexer::Lexer;
use fiddle::lexer::token::Token;

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
fn test_identifier_and_keywords() {
    let source = String::from("let $varName := 123; ");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::Let,
            Token::Dollar,
            Token::Identifier("varName".to_string()),
            Token::ColonEqual,
            Token::Number(123.0),
            Token::Semicolon,
            Token::EOF
        ]
    );
}

#[test]
fn test_doc_comments() {
    let source = String::from("##! Module documentation
module test {
    ### Function documentation
    func example() {
        # Regular comment
        return 42;
    }
}      
");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    println!("Tokens: {:?}", tokens);

    assert_eq!(tokens[0], Token::DocComment("Module documentation".to_string()));
    assert_eq!(tokens[1], Token::Module);
    assert_eq!(tokens[2], Token::Identifier("test".to_string()));
    assert_eq!(tokens[3], Token::LeftBrace);
    assert_eq!(tokens[4], Token::DocComment("Function documentation".to_string()));
    assert_eq!(tokens[5], Token::Func);
    assert_eq!(tokens[6], Token::Identifier("example".to_string()));
    assert_eq!(tokens[7], Token::LeftParen);
    assert_eq!(tokens[8], Token::RightParen);
    assert_eq!(tokens[9], Token::LeftBrace);
    assert_eq!(tokens[10], Token::Return);
    assert_eq!(tokens[11], Token::Number(42.0));
    assert_eq!(tokens[12], Token::Semicolon);
    assert_eq!(tokens[13], Token::RightBrace); // Closes function body
    assert_eq!(tokens[14], Token::RightBrace); // Closes module
    assert_eq!(tokens[15], Token::EOF);
}

#[test]
fn test_tilde_operator() {
    let source = String::from("if (a ~= b) { ~c }");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    assert_eq!(
        tokens,
        vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::TildeEqual,
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Tilde,
            Token::Identifier("c".to_string()),
            Token::RightBrace,
            Token::EOF
        ]
    );
}
