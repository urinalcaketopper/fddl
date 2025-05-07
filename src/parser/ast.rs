#[allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Grouping(Box<Expression>),
    Assignment(String, Box<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Unary
    Minus,        // For unary negation e.g. -5
    Almost,       // For unary ~ e.g. ~5

    // Binary
    Plus, // For addition e.g. 5 + 5
    // Minus, // Note: We have Minus for unary. We'll reuse it for binary.
              // Alternatively, you could have BinaryMinus, UnaryMinus.
              // Reusing is common if context (Expression::Unary vs Expression::Binary) distinguishes them.
    Multiply, // For multiplication e.g. 5 * 5
    Divide, // For division e.g. 5 / 5
    Modulus, // For modulus e.g. 5 % 5

    // Comparison operators
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Equality (we'll add these logic for these later)
    EqualEqual, // For equality e.g. 5 == 5
    NotEqual, // For inequality e.g. 5 != 5
    // AlmostEqual, // For ~= (binary tilde-equal) 🙃
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ExpressionStatement(Expression),
    PrintStatement(Expression),
    VariableDeclaration(String, Option<Expression>),
    Block(Vec<Statement>),
    IfStatement(Expression, Box<Statement>, Option<Box<Statement>>),
    WhileStatement(Expression, Box<Statement>),
    ForStatement(Box<Statement>, Expression, Box<Statement>, Box<Statement>),
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    ReturnStatement(Option<Expression>),
}