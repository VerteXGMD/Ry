use crate::literal::Literal;
use crate::path::Path;
use crate::statement::func_call::FuncCallStmt;
use crate::Token;

#[tokens]
pub enum Expr {
    Literal(ExprLit),
    Borrow(ExprBorrow),
    VariableAccess(ExprVarAccess),
    ConstAccess(ExprConstAccess),
    FuncCall(FuncCallStmt)
}

#[tokens]
pub struct ExprLit(pub Literal);

#[tokens]
/// crate::THING
pub struct ExprConstAccess(pub Path);

#[tokens]
/// thing
pub struct ExprVarAccess(pub Path);

#[tokens]
/// &thing or &mut thing
pub struct ExprBorrow(pub Token![&], pub Option<Token![mut]>, pub Box<Expr>);
