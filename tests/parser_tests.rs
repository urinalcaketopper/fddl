use fddl::lexer::Lexer;
use fddl::parser::Parser;
use fddl::parser::ast::{Statement, Expression, Literal, Operator};

#[test]
fn test_simple_print_statement_number() {
    let source = String::from("print 123;");
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program(); 

    let expected_ast = vec![
        Statement::PrintStatement(
            Expression::Literal(Literal::Number(123.0))
        )
    ];

    assert_eq!(program_ast, expected_ast, "AST for 'print 123;' did not match.");
}

#[test]
fn test_variable_declaration_with_initializer() {
    let source = String::from("let x = 10;");

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::VariableDeclaration(
            "x".to_string(),
            Some(Expression::Literal(Literal::Number(10.0)))
        )
    ];
    assert_eq!(program_ast, expected_ast, "AST for 'let x = 10;' did not match.");
}

#[test]
fn test_unary_not_expression() {
    let source = String::from("print not true;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::PrintStatement(
            Expression::Unary(
                Operator::Not, // Assuming Operator::Not exists from previous steps
                Box::new(Expression::Literal(Literal::Boolean(true)))
            )
        )
    ];
    assert_eq!(program_ast, expected_ast, "AST for 'print not true;' did not match.");
}

#[test]
fn test_binary_precedence() {
    let source = String::from("print 1 + 2 * 3;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::PrintStatement(
            Expression::Binary(
                Box::new(Expression::Literal(Literal::Number(1.0))),
                Operator::Plus,
                Box::new(Expression::Binary(
                    Box::new(Expression::Literal(Literal::Number(2.0))),
                    Operator::Multiply,
                    Box::new(Expression::Literal(Literal::Number(3.0)))
                ))
            )
        )
    ];
    assert_eq!(program_ast, expected_ast, "AST for 'print 1 + 2 * 3;' did not match.");
}

#[test]
fn test_simple_function_call_statement() {
    let source = String::from("my_func();"); // As an expression statement
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::ExpressionStatement(
            Expression::FunctionCall(
                Box::new(Expression::Variable("my_func".to_string())),
                Vec::new() // No arguments
            )
        )
    ];
    assert_eq!(program_ast, expected_ast, "AST for 'my_func();' did not match.");
}

#[test]
fn test_assignment_statement() {
    let source = String::from("count = count + 1;");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::Assignment { // Assuming you added this variant to Statement
            target_name: "count".to_string(),
            value: Expression::Binary(
                Box::new(Expression::Variable("count".to_string())),
                Operator::Plus,
                Box::new(Expression::Literal(Literal::Number(1.0)))
            )
        }
    ];
    assert_eq!(program_ast, expected_ast, "AST for 'count = count + 1;' did not match.");
}

#[test]
fn test_if_else_statement_with_blocks() {
    let source = String::from("if (x < 10) { print \"small\"; } else { print \"large\"; }");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens);
    let program_ast = parser.parse_program();

    let expected_ast = vec![
        Statement::IfStatement(
            Expression::Binary( // Condition: x < 10
                Box::new(Expression::Variable("x".to_string())),
                Operator::Less,
                Box::new(Expression::Literal(Literal::Number(10.0)))
            ),
            Box::new(Statement::Block(vec![ // Then branch
                Statement::PrintStatement(Expression::Literal(Literal::String("small".to_string())))
            ])),
            Some(Box::new(Statement::Block(vec![ // Else branch
                Statement::PrintStatement(Expression::Literal(Literal::String("large".to_string())))
            ])))
        )
    ];
    assert_eq!(program_ast, expected_ast, "AST for if-else statement did not match.");
}