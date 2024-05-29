#[derive(Debug)]
pub enum Literal {
    Int(i64), 
    Bool(bool), 
    Char(char), 
    String(String)
}

#[derive(Debug)]
pub struct ArrayIndex {
    identifier: String, 
    index: Expr // Needs to be an expression
}

#[derive(Debug)]
pub struct FunctionCall {
    identifier: String, 
    arguments: Vec<Expr>, // Should be a vector of expressions
}

#[derive(Debug)]
pub struct LengthExpression {
    argument: Expr // Should be an expression
}

#[derive(Debug)]
pub struct ArrayLiteral {
    elements: Vec<Expr> // Should be a vector of expressions
}

#[derive(Debug)]
pub enum Primary {
    Literal(Literal),
    Identifier(String),
    ArrayIndex(ArrayIndex), 
    FunctionCall(FunctionCall), 
    ArrayLiteral(ArrayLiteral), 
    LenExpression
}

#[derive(Debug)]
pub enum uop {
    Neg, 
    Not
}

#[derive(Debug)]
pub struct UExpr {
    operator: uop, 
    operand: Box<Expr>
}

#[derive(Debug)]
pub enum bop {
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
    Gte
}

#[derive(Debug)]
pub struct BExpr {
    operator: bop, 
    left: Box<Expr>, 
    right: Box<Expr>
}

#[derive(Debug)]
pub enum Expr {
    Primary(Primary), 
    Unary(UExpr), 
    Binary(BExpr)
}

#[derive(Debug)]
pub struct AssignStmt {
    identifier: String, 
    expression: Expr
}

pub enum Type {
    Int, 
    Bool, 
    Char, 
    String, 
    Array(Box<Type>), 
    Void
}

#[derive(Debug)]
pub struct Declaration {
    decl_type: Type,
    identifier: String, 
    expression: Option<Expr>
}

#[derive(Debug)]
pub struct Block {
    statements: Vec<Statement>
}

#[derive(Debug)]
pub struct IfStmt {
    condition: Expr, 
    then_block: Block, 
    else_block: Option<Block>
}

#[derive(Debug)]
pub struct WhileStmt {
    condition: Expr, 
    block: Block
}

#[derive(Debug)]
pub struct ReturnStmt {
    expression: Option<Expr>
}

#[derive(Debug)]
pub struct ProcedureCall {
    identifier: String, 
    arguments: Vec<Expr>
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration), 
    Assign(AssignStmt), 
    Block(Block), 
    If(IfStmt), 
    While(WhileStmt), 
    Return(ReturnStmt), 
    ProcedureCall(ProcedureCall)
}

#[derive(Debug)]
pub struct Function {
    return_type: Type, 
    identifier: String, 
    arguments: Vec<(Type, String)>, 
    block: Block
}

#[derive(Debug)]
pub struct GlobalDecl {
    decl_type: Type, 
    identifier: String
    expression: Option<Expr>
}

#[derive(Debug)]
pub struct UseStmt {
    identifier: String
}

#[derive(Debug)]
pub struct Program {
    global_declarations: Vec<GlobalDecl>, 
    functions: Vec<Function>, 
    use_statements: Vec<UseStmt>
}