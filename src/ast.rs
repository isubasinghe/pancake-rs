use std::sync::Arc;


#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub enum ConstX {
    Nat(u64)
}

pub type Const = Arc<ConstX>;

pub type Ident = Arc<String>;


pub enum UOp {
    Neg,
    Raw
}

#[derive(Debug)]
pub enum Cmp {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq
}

#[derive(Debug)]
pub enum ExprX {
    BinOp(BinOp, Expr, Expr),
    CmpOp(Cmp, Expr, Expr),
    Const(Const),
    Ident(Ident)
}

pub type Expr = Arc<ExprX>;


#[derive(Debug)]
pub enum StatementX {
    DeclAssign(Ident, Expr),
    If(Expr, Vec<Statement>),
    While(Expr, Vec<Statement>),
    Return(Expr),
}

pub type Statement = Arc<StatementX>;

#[derive(Debug)]
pub struct ParamX {
    pub num_words: u64,
    pub name: Ident 
}
pub type Param = Arc<ParamX>;

#[derive(Debug)]
pub struct FunctionX {
    pub export: bool,
    pub name: Ident,
    pub params: Vec<Param>,
    pub body: Vec<Statement>
}

pub type Function = Arc<FunctionX>;
