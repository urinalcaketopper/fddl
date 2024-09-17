pub enum Expression {
    Literal(Literal),
    Variable(String),
    Binary(Box<Expression>, Operator, Box<Expression>),
    Unary(Operator, Box<Expression>),
    Grouping(Box<Expression>),
    Assignment(String, Box<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
}

pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum Operator {
    Plus,
    Minux,
    Multiply,
    Divide,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    EqualEqual,
    NotEqual,
    AlmostEqual,
    Almost,
}

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