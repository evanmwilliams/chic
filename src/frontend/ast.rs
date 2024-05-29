#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Bool(bool),
    Char(char),
    String(String),
}

#[derive(Debug)]
pub struct ArrayIndex {
    pub identifier: String,
    pub index: Box<Expr>, // Needs to be an expression
}

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: String,
    pub arguments: Vec<Expr>, // Should be a vector of expressions
}

#[derive(Debug)]
pub struct LengthExpression {
    pub argument: Expr, // Should be an expression
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub elements: Vec<Expr>, // Should be a vector of expressions
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
    ArrayIndex(ArrayIndex),
    FunctionCall(FunctionCall),
    ArrayLiteral(ArrayLiteral),
    LenExpression,
}

#[derive(Debug)]
pub enum Uop {
    Neg,
    Not,
}

#[derive(Debug)]
pub struct UExpr {
    pub operator: Uop,
    pub operand: Box<Expr>,
}

#[derive(Debug)]
pub enum Bop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    HMul,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug)]
pub struct BExpr {
    pub operator: Bop,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Primary(Primary),
    Unary(UExpr),
    Binary(BExpr),
}

#[derive(Debug)]
pub struct AssignStmt {
    pub identifier: String,
    pub expression: Expr,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    Char,
    String,
    Array(Box<Type>),
    Void,
}

#[derive(Debug)]
pub struct Declaration {
    pub decl_type: Type,
    pub ids: Vec<String>,
    pub exprs: Vec<Expr>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Expr,
    pub block: Block,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub expression: Option<Expr>,
}

#[derive(Debug)]
pub struct ProcedureCall {
    pub identifier: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Assign(AssignStmt),
    Block(Block),
    If(IfStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    ProcedureCall(ProcedureCall),
}

#[derive(Debug)]
pub struct Function {
    pub return_type: Type,
    pub identifier: String,
    pub arguments: Vec<(Type, String)>,
    pub block: Block,
}

#[derive(Debug)]
pub struct UseStmt {
    pub identifier: String,
}

#[derive(Debug)]
pub struct Program {
    pub global_declarations: Vec<Declaration>,
    pub functions: Vec<Function>,
    pub use_statements: Vec<UseStmt>,
}
